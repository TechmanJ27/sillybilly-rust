use crate::Data;
use crate::Error;
use poise::serenity_prelude as serenity;

// Who am I?
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel")
] pub async fn about(
    ctx: poise::ApplicationContext<'_, Data, Error>,
) -> Result<(), Error> {
    let about_me_embed = serenity::CreateEmbed::default()
        .title("About Me")
        .description("Hey~\nI\'m Sillybilly, a fun roleplay/action bot developed solely by MikuBerry (@techmanj27)\n
        Originally created for users in the UES server and inspired by Goober Bot, people all across the platform have begun using me!\n
        In addition to rp commands, I also have some fun and useful features :3")
        .color(0x4EBDED);

    ctx.send(
        poise::CreateReply::default()
            .embed(about_me_embed)
    ).await?;
    Ok(())
}
