use poise::{ChoiceParameter, serenity_prelude::{
    self as serenity, 
    CreateInteractionResponse, 
    CreateInteractionResponseMessage, 
    InteractionResponseFlags,
}};
use rand::seq::IndexedRandom;
use rand::RngExt;
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap, fs::File, io::BufReader};

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
    ).await?;
    Ok(())
}

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum Fruit {
    #[name = "avocado"]
    Avocado,
    #[name = "lemon"]
    Lemon,
    #[name = "tomato"]
    Tomato,
    #[name = "apple"]
    Apple,
    #[name = "banana"]
    Banana,
    #[name = "orange"]
    Orange,
    #[name = "grape"]
    Grape,
    #[name = "strawberry"]
    Strawberry,
    #[name = "watermelon"]
    Watermelon,
    #[name = "pineapple"]
    Pineapple,
    #[name = "cherry"]
    Cherry,
    #[name = "peach"]
    Peach,
    #[name = "pear"]
    Pear,
    #[name = "mango"]
    Mango,
    #[name = "kiwi"]
    Kiwi,
    #[name = "blueberry"]
    Blueberry,
    #[name = "coconut"]
    Coconut,
    #[name = "melon"]
    Melon,
    #[name = "green apple"]
    GreenApple,
    #[name = "olive"]
    Olive,
}

impl Fruit {
    pub fn emoji(&self) -> &'static str {
        match self {
            Fruit::Avocado => "🥑",
            Fruit::Lemon => "🍋",
            Fruit::Tomato => "🍅",
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Grape => "🍇",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
            Fruit::Pineapple => "🍍",
            Fruit::Cherry => "🍒",
            Fruit::Peach => "🍑",
            Fruit::Pear => "🍐",
            Fruit::Mango => "🥭",
            Fruit::Kiwi => "🥝",
            Fruit::Blueberry => "🫐",
            Fruit::Coconut => "🥥",
            Fruit::Melon => "🍈",
            Fruit::GreenApple => "🍏",
            Fruit::Olive => "🫒",
        }
    }
    
}

// Fruit somebody
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)] async fn fruit(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "The fruit to fruit somebody with"]
    fruit: Fruit,
) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content(format!("${} casts ${}!", ctx.author(), fruit.name()))
    )
    .await?;
    ctx.send(
        poise::CreateReply::default()
            .content(format!("# {}", fruit.emoji()))
    )
    .await?;
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

// Roll a die - Or, flip a coin
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn roll(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "How many sides should the die have?"]
    #[min = 2]
    sides: u32,
) -> Result<(), Error> {
    let result = rand::rng().random_range(1..=sides);
    let mut message = format!("You rolled a {}!{}", result, footer(result, sides));

    if sides == 2 {
      message = match result{
        1=>"Heads\n-# Coinflip".to_string(),
        2=>"Tails\n-# Coinflip".to_string(),
        _=>message,
      }
    }

    ctx.send(
        poise::CreateReply::default()
            .content(message)
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