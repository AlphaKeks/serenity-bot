use {
	crate::Bot,
	anyhow::Result,
	serenity::{model::channel::Message, prelude::Context},
};

pub async fn handle(_client: &Bot, ctx: Context, msg: Message) -> Result<()> {
	if msg.content.to_lowercase().starts_with("bing?") {
		msg.reply(&ctx.http, "chilling 🥶").await?;
	}

	return Ok(());
}
