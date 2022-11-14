use {
	std::env,
	crate::{commands, Bot},
	anyhow::Result,
	serenity::{
		model::{
			application::command::Command,
			prelude::{Activity, GuildId, Ready},
		},
		prelude::Context,
	},
};

pub async fn handle(_client: &Bot, ctx: Context, ready: Ready) -> Result<()> {
	// set status
	ctx.set_activity(Activity::listening("☕ ~ Lofi Beats ~")).await;

	println!("Connected to Discord as {}.", ready.user.tag());

	// registering commands
	let mode = env::var("MODE")?;
	match mode.as_str() {
		"DEV" => {
			let dev_guild = GuildId(env::var("DEV_GUILD")?.parse::<u64>()?);
			if let Ok(commands) = dev_guild
				.set_application_commands(&ctx.http, |commands| {
					commands.create_application_command(|cmd| commands::ping::register(cmd))
				})
				.await
			{
				let command_names: Vec<String> = commands.into_iter().map(|cmd| cmd.name).collect();
				print_commands(command_names, mode);
			}
		},
		"PROD" => {
			if let Ok(commands) = Command::set_global_application_commands(&ctx.http, |commands| {
				commands.create_application_command(|cmd| commands::ping::register(cmd))
			})
			.await
			{
				let command_names: Vec<String> = commands.into_iter().map(|cmd| cmd.name).collect();
				print_commands(command_names, mode);
			}
		},
		_ => unreachable!("[env] Invalid `MODE`. Use `DEV` or `PROD`."),
	}

	return Ok(());
}

fn print_commands(names: Vec<String>, mode: String) {
	println!(
		"[MODE: {}] {}",
		mode,
		if names.len() > 0 {
			format!("Registered commands:\n> {}", names.join("\n> "))
		} else {
			String::from("No commands registered.")
		}
	);
}
