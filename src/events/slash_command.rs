use {
	std::collections::HashMap,
	crate::{commands, Bot},
	anyhow::Result,
	serenity::{
		builder::CreateEmbed,
		http::Http,
		json,
		model::{
			application::interaction::application_command::ApplicationCommandInteraction,
			user::User,
		},
		prelude::Context,
	},
};

pub async fn handle(
	client: &Bot,
	ctx: Context,
	interaction: ApplicationCommandInteraction,
) -> Result<()> {
	let ctx = InteractionData::new(&interaction, &ctx.http, client).await?;
	// REGISTER COMMANDS HERE
	match interaction.data.name.as_str() {
		"ping" => commands::ping::execute(ctx).await,
		unkown_command => unimplemented!("Command `{}` not found.", unkown_command),
	}
}

#[derive(Debug, Clone)]
pub enum InteractionResponseData<'a> {
	Message(&'a str),
	Embed(CreateEmbed),
}

#[derive(Debug, Clone)]
pub struct InteractionData<'a> {
	interaction: &'a ApplicationCommandInteraction,
	http: &'a Http,
	deferred: bool,
	pub client: &'a Bot,
	pub user: &'a User,
	pub opts: HashMap<String, json::Value>,
}

impl<'a> InteractionData<'a> {
	async fn new(
		interaction: &'a ApplicationCommandInteraction,
		http: &'a Http,
		client: &'a Bot,
	) -> Result<InteractionData<'a>> {
		let mut opts: HashMap<String, json::Value> = HashMap::new();
		for opt in &interaction.data.options {
			if let Some(value) = opt.value.to_owned() {
				opts.insert(opt.name.clone(), value);
			}
		}

		return Ok(Self {
			interaction,
			http,
			deferred: false,
			client,
			user: &interaction.user,
			opts,
		});
	}

	// some commands need to load a bit longer, so we can tell discord to remember an interaction
	pub async fn defer(&mut self) -> Result<()> {
		self.interaction.defer(self.http).await?;
		self.deferred = true;
		return Ok(());
	}

	pub async fn reply(&self, content: InteractionResponseData<'_>) -> Result<()> {
		if self.deferred {
			self.interaction
				.edit_original_interaction_response(self.http, |response| match content {
					InteractionResponseData::Message(msg) => response.content(msg),
					InteractionResponseData::Embed(embed) => response.set_embed(embed),
				})
				.await?;
		} else {
			self.interaction
				.create_interaction_response(self.http, |response| {
					response.interaction_response_data(|response| match content {
						InteractionResponseData::Message(msg) => response.content(msg),
						InteractionResponseData::Embed(embed) => response.set_embed(embed),
					})
				})
				.await?
		}

		return Ok(());
	}

	fn get(&self, name: &'a str) -> Option<json::Value> {
		if let Some(value) = self.opts.get(name) {
			return Some(value.to_owned());
		}
		return None;
	}

	pub fn get_string(&self, name: &'a str) -> Option<String> {
		if let Some(json::Value::String(string)) = self.get(name) {
			return Some(string);
		}
		return None;
	}

	pub fn get_int(&self, name: &'a str) -> Option<i64> {
		if let Some(json::Value::Number(number)) = self.get(name) {
			return number.as_i64();
		}
		return None;
	}

	pub fn get_float(&self, name: &'a str) -> Option<f64> {
		if let Some(json::Value::Number(number)) = self.get(name) {
			return number.as_f64();
		}
		return None;
	}

	pub fn get_bool(&self, name: &'a str) -> Option<bool> {
		if let Some(json::Value::Bool(boolean)) = self.get(name) {
			return Some(boolean);
		}
		return None;
	}

	pub fn get_user(&self, name: &'a str) -> Option<u64> {
		if let Some(json::Value::String(string)) = self.get(name) {
			if let Ok(user_id) = string.parse::<u64>() {
				return Some(user_id);
			}
		}
		return None;
	}
}
