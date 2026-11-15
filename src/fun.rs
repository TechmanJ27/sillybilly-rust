use crate::Data;
use crate::Error;

use rand::RngExt;
use poise::{ChoiceParameter};
use std::{borrow::Cow};

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
)] 
pub async fn fruit(
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
pub async fn roll(
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
