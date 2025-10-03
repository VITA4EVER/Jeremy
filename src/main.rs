use log::info;
use poise::serenity_prelude as serenity;
use poise::{Context};
use serde::*;

mod leaderboard;
mod genstats;

struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_, Data, Error>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping(),
                
                // Leaderboard category
                leaderboard::daily_leaderboard(),
                leaderboard::weekly_leaderboard(),
                leaderboard::monthly_leaderboard(),
                leaderboard::leaderboard(),
                
                // Genstats category
                genstats::players_in_multiplayer(),
                
                ],
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &_framework.options().commands).await?;
                Ok(Data)
            })
        })
        .build();

    let mut client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
