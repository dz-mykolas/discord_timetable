use poise::serenity_prelude as serenity;
use std::env;

mod commands;
mod database_utils;
mod utils;

use crate::commands::Data;
use crate::commands::{
    insert_course, list_assessments, list_courses, remove_course, insert_assessment, remove_assessment,
};

#[tokio::main]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("DATABASE_URL", "sqlite:./courses.db");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                insert_course(),
                remove_course(),
                list_courses(),
                list_assessments(),
                insert_assessment(),
                remove_assessment(),
            ],
            ..Default::default()
        })
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
