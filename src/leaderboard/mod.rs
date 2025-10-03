use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiplayerLeaderboardRow {
	AccountID:     String,
	OnlineID:      String,
	TotalKills:    i64,
	TotalDeaths:   i64,
	TotalVdollar:  i64,
	TotalDistance: i64,
	GamesPlayed:   i64,
	TotalWins:     i64,
	TotalLosses:   i64,
}

#[derive(Debug, Deserialize)]
struct Root {
    leaderboard: Vec<MultiplayerLeaderboardRow>,
}

#[poise::command(slash_command)]
pub async fn daily_leaderboard(ctx: Context<'_, Data, Error>) -> Result<(), Error> {
    let response = reqwest::get("http://mirage.yuv.pink/api/v1/leaderboard/daily")
        .await?
		.text()
		.await?;

	let mut result = serde_json::from_str::<Root>(&response)?;

	let _ = result.leaderboard.pop();

	let mut result_to_display : String = "# Top 10 Players today!\n".to_string();

	for (index, small_piece) in result.leaderboard.iter().enumerate() {
		result_to_display += &format!("**{}.** - {} - **{}** VDollars\n", (index + 1), small_piece.OnlineID, small_piece.TotalVdollar);
	}

	ctx.say(result_to_display).await?;

	Ok(())
}

#[poise::command(slash_command)]
pub async fn weekly_leaderboard(ctx: Context<'_, Data, Error>) -> Result<(), Error> {
    let response = reqwest::get("http://mirage.yuv.pink/api/v1/leaderboard/weekly")
        .await?
		.text()
		.await?;

	let mut result = serde_json::from_str::<Root>(&response)?;

	let _ = result.leaderboard.truncate(10);

	let mut result_to_display : String = "# Top 10 Players this week!\n".to_string();

	for (index, small_piece) in result.leaderboard.iter().enumerate() {
		result_to_display += &format!("**{}.** - {} - **{}** VDollars\n", (index + 1), small_piece.OnlineID, small_piece.TotalVdollar);
	}

	ctx.say(result_to_display).await?;

	Ok(())
}

#[poise::command(slash_command)]
pub async fn monthly_leaderboard(ctx: Context<'_, Data, Error>) -> Result<(), Error> {
    let response = reqwest::get("http://mirage.yuv.pink/api/v1/leaderboard/monthly")
        .await?
		.text()
		.await?;

	let mut result = serde_json::from_str::<Root>(&response)?;

	let _ = result.leaderboard.truncate(10);

	let mut result_to_display : String = "# Top 10 Players this month!\n".to_string();

	for (index, small_piece) in result.leaderboard.iter().enumerate() {
		result_to_display += &format!("**{}.** - {} - **{}** VDollars\n", (index + 1), small_piece.OnlineID, small_piece.TotalVdollar);
	}

	ctx.say(result_to_display).await?;

	Ok(())
}

#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_, Data, Error>) -> Result<(), Error> {
    let response = reqwest::get("http://mirage.yuv.pink/api/v1/leaderboard/total")
        .await?
		.text()
		.await?;

	let mut result = serde_json::from_str::<Root>(&response)?;

	let _ = result.leaderboard.truncate(10);

	let mut result_to_display : String = "# Top 10 Players in TOTAL!\n".to_string();

	for (index, small_piece) in result.leaderboard.iter().enumerate() {
		result_to_display += &format!("**{}.** - {} - **{}** VDollars\n", (index + 1), small_piece.OnlineID, small_piece.TotalVdollar);
	}

	ctx.say(result_to_display).await?;

	Ok(())
}