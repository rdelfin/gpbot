use crate::commands::{CUSTOM_HELP, GENERAL_GROUP};
use anyhow::Context as AnyhowContext;
use dotenv::dotenv;
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use serenity::{async_trait, framework::standard::StandardFramework, prelude::*};
use std::env;

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    configure_logging()?;
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .normal_message(commands::normal_message)
        .help(&CUSTOM_HELP)
        .group(&GENERAL_GROUP);

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .context("Err creating client")?;

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    client.start().await.context("client error")?;

    Ok(())
}

fn configure_logging() -> anyhow::Result<()> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {h({l})} [{M}] {m}{n}")))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("tracing::span", LevelFilter::Warn))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))?;
    log4rs::init_config(config)?;
    Ok(())
}
