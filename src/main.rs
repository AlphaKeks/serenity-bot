#![allow(dead_code)]

use {
	std::env,
	dotenv::dotenv,
	anyhow::Result,
	serenity::{
		async_trait,
		model::{application::interaction::Interaction, channel::Message, prelude::Ready},
		prelude::{Context, EventHandler, GatewayIntents},
		Client,
	},
	simple_logger::SimpleLogger,
};

mod commands;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
	// load environment variables
	dotenv()?;

	// instantiate logging
	SimpleLogger::new()
		.with_colors(true)
		.with_level(log::LevelFilter::Warn)
		.init()?;

	// create metadata
	let token = env::var("DISCORD_TOKEN")?;
	let schnose = Bot::new(token);

	// create serenity client
	let mut client =
		Client::builder(&schnose.token, schnose.intents).event_handler(schnose).await?;

	// connect to discord
	client.start().await?;

	return Ok(());
}

#[derive(Debug, Clone)]
pub struct Bot {
	pub token: String,
	pub intents: GatewayIntents,
}

impl Bot {
	fn new(token: String) -> Self {
		Self {
			token,
			intents: GatewayIntents::GUILDS
				| GatewayIntents::GUILD_MEMBERS
				| GatewayIntents::GUILD_MESSAGES
				| GatewayIntents::GUILD_MESSAGE_REACTIONS
				| GatewayIntents::MESSAGE_CONTENT,
		}
	}
}

#[async_trait]
impl EventHandler for Bot {
	// gets triggered once when the bot is starting
	async fn ready(&self, ctx: Context, ready: Ready) {
		let _ = events::ready::handle(self, ctx, ready).await;
	}

	// gets triggered on every message the bot can see
	async fn message(&self, ctx: Context, msg: Message) {
		let _ = events::message::handle(self, ctx, msg).await;
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		match interaction {
			Interaction::ApplicationCommand(slash_command) => {
				let _ = events::slash_command::handle(self, ctx, slash_command).await;
			},
			_ => unimplemented!(),
		}
	}
}
