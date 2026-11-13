use poise::serenity_prelude::{self as serenity};
use rand::seq::IndexedRandom;
use rand::RngExt;
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap, fs::File, io::BufReader};
use reqwest::header::USER_AGENT;

struct Data {
    bot_id: serenity::UserId,
    actions: HashMap<String, CommandData>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CommandData {
    name: String,
    description: String,
    option_desc: String,
    self_response: String,
    bot: String,
    responses: Vec<String>,
}

#[derive(Deserialize)]
struct CatResponse {
    id: String,
    url: String,
    width: u32,
    height: u32,
}

// Get a random cat image
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn cat(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let cat_key_header = HeaderName::from_static("x-api-key");
    let cat_key = std::env::var("CAT_API_KEY").expect("missing CAT_API_KEY");
    let resp = client
        .get("https://beta-api.thecatapi.com/v1/images/search")
        .header(cat_key_header, cat_key)
        .await?;
    if !is_success(resp.status) {
        print!("HTTP error! Status: {}", resp.status);
        ctx.send(poise::CreateReply::default().content("Could not fetch a cat right now.")).await?;
        Ok(())
    }

    let data = resp.json::<CatResponse>[]().await?;

    if ctx.CommandInteraction.Permissions.embed_links() {
        let cat_embed = serenity::CreateEmbed::default()
            .image(data[0].url)
            .color("#4EBDED")
            .footer("Powered by thecatapi.com");
        ctx.send(poise::CreateReply::default().embed(cat_embed)).await?;
    } else {
       ctx.send(poise::CreateReply::default().content(data[0].url)).await?;
    }
    
    Ok(())
}

// Send a cat
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn cat_art(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content("_  _╱|、\n
            (˚ˎ 。7\n
            |、˜〵\n
            じしˍ,)ノ")
    ).await?;
    Ok(())
}

/// Who am I?
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel")
] async fn about(
    ctx: Context<'_>,
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
            .reply(true)
    ).await?;
    Ok(())
}

fn footer(result: u32, sides: u32) -> Cow<'static, str> {
    match sides {
        4 => Cow::Borrowed("\n-# d4"),
        6 => Cow::Borrowed("\n-# d6"),
        8 => Cow::Borrowed("\n-# d8"),
        10 => Cow::Borrowed("\n-# d10"),
        12 => Cow::Borrowed("\n-# d12"),
        20 => match result {
            1 => Cow::Borrowed(" - Nat1\n-# d20"),
            20 => Cow::Borrowed(" - Nat20\n-# d20"),
            _ => Cow::Borrowed("\n-# d20"),
        },
        100 => Cow::Borrowed("\n-# d100"),
        _ => Cow::Owned(format!("\n-# {}-sided die", sides)),
    }
}

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn roll(
    ctx: Context<'_>,
    #[description = "How many sides should the dice have?"]
    #[min = 2]
    sides: u32,
) -> Result<(), Error> {
    let result = rand::rng().random_range(1..=sides);
    let message = format!("You rolled a {}!{}", result, footer(result, sides));

    ctx.send(
        poise::CreateReply::default()
            .content(message)
            .reply(true),
    )
    .await?;
    Ok(())
}

fn parse_response(template: &str, target_id: serenity::UserId, user_id: serenity::UserId) -> String {
    template
        .replace("{user}", &format!("<@{}>", user_id))
        .replace("{target}", &format!("<@{}>", target_id))
        .trim()
        .to_string()
}

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel")
] async fn action(
    ctx: Context<'_>,
    #[description = "placeholder"] user: serenity::User,
) -> Result<(), Error> {
    let data = ctx
        .data()
        .actions
        .get(&*ctx.command().name)
        .ok_or("unknown action command")?;

    let author_id = ctx.author().id;
    let target_id = user.id;

    let random = || {
    data.responses
        .choose(&mut rand::rng())
        .map(String::as_str)
        .unwrap_or("")
    };

    let template = if target_id == author_id && !data.self_response.is_empty() {
        data.self_response.as_str()
    } else if target_id == ctx.data().bot_id && !data.bot.is_empty() {
        data.bot.as_str()
    } else {
        random()
    };

    ctx.send(
        poise::CreateReply::default()
            .content(parse_response(template, target_id, author_id))
    ).await?;
    Ok(())
}

fn build_dynamic_command(data: &CommandData) -> poise::Command<Data, Error> {
    let mut cmd = action();
    cmd.name = Cow::Owned(data.name.clone());
    cmd.qualified_name = Cow::Owned(data.name.clone());
    cmd.identifying_name = Cow::Owned(data.name.clone());
    cmd.description = Some(Cow::Owned(data.description.clone()));
    cmd.parameters[0].description = Some(Cow::Owned(data.option_desc.clone()));
    cmd
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let file = File::open("./actionCommands.json").expect("Could not open actionCommands.json");
    let raw: Vec<CommandData> = serde_json::from_reader(BufReader::new(file))
        .expect("Error while reading or parsing actionCommands.json");

    let mut commands: Vec<_> = raw.iter().map(build_dynamic_command).collect();
    let actions: HashMap<String, CommandData> =
        raw.into_iter().map(|c| (c.name.clone(), c)).collect();

    commands.push(about());
    commands.push(roll());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    bot_id: ready.user.id,
                    actions,
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}
