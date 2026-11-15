mod fun;
mod utility;

use poise::{serenity_prelude::{
    self as serenity, Color, CreateInteractionResponse, CreateInteractionResponseMessage, InteractionResponseFlags
}};
use rand::seq::IndexedRandom;

use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap, fs::File, io::BufReader};
use reqwest;

pub struct Data {
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
    url: String,
}

// Get a random cat image
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    required_permissions = "EMBED_LINKS"
)]
async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let cat_key = std::env::var("CAT_API_KEY").expect("missing CAT_API_KEY");

    let resp = client
        .get("https://api.thecatapi.com/v1/images/search")
        .header("x-api-key", cat_key)
        .send()
        .await?;

    if !resp.status().is_success() {
        eprintln!("HTTP error! Status: {}", resp.status());
        ctx.send(poise::CreateReply::default().content("Could not fetch a cat right now."))
            .await?;
        return Ok(());
    }

    let cats: Vec<CatResponse> = resp.json().await?;

    if let Some(cat) = cats.first() {
        let cat_embed_footer = serenity::CreateEmbedFooter::new("Powered by thecatapi.com");
        let cat_embed = serenity::CreateEmbed::default()
            .image(&cat.url)
            .color(Color::new(5160429))
            .footer(cat_embed_footer);

        ctx.send(poise::CreateReply::default().embed(cat_embed))
            .await?;
    } else {
        ctx.send(poise::CreateReply::default().content("No cat image was returned."))
            .await?;
    }

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
    ctx: poise::ApplicationContext<'_, Data, Error>,
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

    let data = CreateInteractionResponseMessage::new()
        .content(parse_response(template, target_id, author_id))
        .flags(InteractionResponseFlags::SUPPRESS_NOTIFICATIONS);

    let builder = CreateInteractionResponse::Message(data);
    ctx.interaction.create_response(ctx, builder).await?;

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

    commands.push(utility::about());
    commands.push(fun::roll());
    commands.push(cat());
    commands.push(fun::fruit());
    commands.push(fun::eight_ball());

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
