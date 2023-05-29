use regex::Regex;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{command, group, help, hook},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{
        channel::{Message, ReactionType},
        id::UserId,
    },
    prelude::*,
};
use std::collections::HashSet;
use tracing::{debug, info};

const GP_COPYPASTA: &'static str = "Gracious Professionalism is part of the ethos of \
    FIRST. It's a way of doing things that encourages high-quality work, emphasizes the \
    value of others, and respects individuals and the community. With Gracious \
    Professionalism, fierce competition and mutual gain are not separate notions. \
    Gracious professionals learn and compete like crazy but treat one another with \
    respect and kindness in the process. They avoid treating anyone like losers. No \
    chest thumping tough talk, but no sticky-sweet platitudes either. Knowledge, \
    competition, and empathy are comfortably blended. In the long run, Gracious \
    Professionalism is part of pursuing a meaningful life. One can add to society and \
    enjoy the satisfaction of knowing one has acted with integrity and sensitivity.";

#[group]
// Set a description to appear if a user wants to display a single group
// e.g. via help using the group-name or one of its prefixes.
#[description = "Most of the GP commands can be found under here"]
// Summary only appears when listing multiple groups.
#[summary = "Graciously Professional!™"]
#[commands(ping)]
pub struct General;

#[command]
#[description = "Send this command and find out ;)"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    info!(event = "requested_ping", msg = ?msg);
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    let re = Regex::new(r"[Gg]racious [Pp]rofessionalism").unwrap();
    debug!(event = "normal_message", message_contents = msg.content);
    if re.is_match(&msg.content) {
        info!(
            event = "gp_alert",
            sender = ?msg.author,
            message_contents = msg.content
        );
        msg.react(ctx, ReactionType::Unicode("😱".into()))
            .await
            .unwrap();
        msg.reply(ctx, GP_COPYPASTA).await.unwrap();
    }
}

#[help]
async fn custom_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
