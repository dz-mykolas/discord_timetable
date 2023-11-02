use poise::serenity_prelude as serenity;
use poise::Event;

use std::env;

mod commands;
mod database_utils;
mod interaction_handlers;
mod utils;

use crate::commands::Data;
use crate::commands::{
    insert_assessment, insert_course, list_assessments, list_courses, remove_assessment,
    remove_course,
};

use std::sync::atomic::{AtomicU32, Ordering};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("DATABASE_URL", "sqlite:./courses.db");

    let options = poise::FrameworkOptions {
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        commands: vec![
            insert_course(),
            remove_course(),
            list_courses(),
            list_assessments(),
            insert_assessment(),
            remove_assessment(),
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(env::var("DISCORD_TOKEN").expect("Expected a token in the environment"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::InteractionCreate { interaction, .. } => {
            if let serenity::model::application::interaction::Interaction::MessageComponent(
                component,
            ) = interaction
            {
                let msg = component.message.clone();

                let command = match msg.interaction {
                    Some(interaction) => interaction.name,
                    None => "".to_string(),
                };

                match command.as_str() {
                    "list_courses" => {
                        interaction_handlers::list_courses_handler(ctx, interaction).await?;
                    }
                    "list_assessments" => {
                        interaction_handlers::list_assessments_handler(ctx, interaction).await?;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(())
}
