use poise::serenity_prelude as serenity;
use std::env;

mod commands;
mod database_utils;
mod utils;

use crate::commands::Data;
use crate::commands::{age, courses, remove_course, insert_course, write_table, list_courses};

#[tokio::main]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("DATABASE_URL", "sqlite:./courses.db");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), write_table(), courses(), insert_course(), remove_course(), list_courses()],
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
