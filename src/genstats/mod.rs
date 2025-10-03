use crate::*;
mod prometheus_stats;

#[poise::command(slash_command)]
pub async fn players_in_multiplayer(ctx: Context<'_, Data, Error>) -> Result<(), Error> {
    let players_in_multiplayer_count_wrapped = prometheus_stats::get_active_users().await;

    match players_in_multiplayer_count_wrapped {
        Ok(result) => {
            ctx.say(format!("There are currently **{}** playing multiplayer! Join them?", result)).await?;
        }
        Err(error_string) => {
            ctx.say(format!("Oh NO! Something went wrong while fetching. Here is some info: {}", error_string)).await?;
        }
    }
    Ok(())
}