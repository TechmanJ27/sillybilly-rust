// import { Collection, GuildMember, PermissionFlagsBits, type GuildTextBasedChannel} from "discord.js";
use poise::{serenity_prelude::{
  self as serenity, Member, builder::GetMessages, model::id::{ChannelId, MessageId}
}};
use std::cmp::min;
use std::collections::HashMap;
use time;

#[derive(Hash, Eq, PartialEq, Debug)]
struct MarkovEntry {
    date_time: i32; 
    weights: HashMap<string, Vec<String>>;
}

impl MarkovEntry {
    fn new(weights: HashMap<string, Vec<String>>) -> MarkovEntry {
        MarkovEntry { date_time: time::Timestamp::now().as_seconds(), weights: weights }
    }
}

let global_weights = HashMap::new()<string, HashMap<string, MarkovEntry>>();

pub async fn fetch_in_channel(
  channel_id: String,
  author_id: String,
  max_messages_to_scan: i32,
  mut collected: Vec<String>,
  max_total_messages: i32,
) -> Result<Vec<String>, Error> {
  let mut before: String;
  let mut scanned = 0;
  while scanned < max_messages_to_scan && collected.len() < max_total_messages as usize {
    let limit = min(100, max_messages_to_scan - scanned);
    let builder = GetMessages::new().before(MessageId::new(before)).limit(limit);
    let batch = channel_id.messages(&http, builder).await?;
    if batch.len() == 0 { break; }
    for message in batch {
      if collected.len() >= max_total_messages as usize { break; }

      if message.author.id == author_id && message.content {
        collected.push(message.content);  
      }
    }
    scanned = scanned + batch.len();
    before = batch.last();

    if before.len() == 0 || batch.len() < limit { break; }
  }
  collected
}

pub async fn index_messages(
    user: Member,
    max_messages_per_channel: i32,
    max_total_messages: i32,
    concurrency: i32,
) {
global_weights
}

// export async function indexMessages(
//     user: GuildMember,
//     maxMessagesPerChannel: number,
//     maxTotalMessages = 1500,
//     concurrency = 4,
// ): Promise<string[]> {
//     const me = user.guild.members.me;
//     if (!me) return [];

//     const neededBotPerms = [
//         PermissionFlagsBits.ViewChannel,
//         PermissionFlagsBits.ReadMessageHistory,
//     ];

//     const readableChannels = [...user.guild.channels.cache.values()].filter(
//         (channel): channel is GuildTextBasedChannel => {
//             if (!channel.isTextBased() || channel.isDMBased()) return false;
//             if (!channel.lastMessageId) return false;

//             const botPerms = channel.permissionsFor(me);
//             const userPerms = channel.permissionsFor(user);
//             return Boolean(
//                 botPerms?.has(neededBotPerms) &&
//                 userPerms?.has(PermissionFlagsBits.ViewChannel),
//             );
//         },
//     );

//     const messages: string[] = [];
//     let cursor = 0;

//     async function worker() {
//         while (
//             cursor < readableChannels.length &&
//             messages.length < maxTotalMessages
//             ) {
//             const channel = readableChannels[cursor++]!;
//             try {
//                 await fetchInChannel(
//                     channel,
//                     user.id,
//                     maxMessagesPerChannel,
//                     messages,
//                     maxTotalMessages,
//                 );
//             } catch (err) {
//                 console.error(`Failed to read #${channel.name}:`, err);
//             }
//         }
//     }

//     const workers = Array.from(
//         { length: Math.min(concurrency, readableChannels.length) },
//         () => worker(),
//     );

//     await Promise.all(workers);
//     return messages;
// }

// export async function generateMarkovWeights(user: GuildMember) {
//     const messages = await indexMessages(user, 500);
//     if (messages.length === 0) return undefined;

//     const weights = new Map<string, string[]>();

//     for (const message of messages) {
//         const words = message.split(/\s+/).filter((word) => word !== '');
//         if (words.length === 0) continue;

//         const tokens = ['<START>', ...words, '<END>'];

//         for (let i = 0; i < tokens.length - 1; i++) {
//             const current = tokens[i]!;
//             const next = tokens[i + 1]!;
//             const followers = weights.get(current);

//             if (followers) {
//                 followers.push(next);
//             } else {
//                 weights.set(current, [next]);
//             }
//         }
//     }

//     const guildWeights = globalWeights.ensure(user.guild.id, () => new Collection());
//     guildWeights.set(user.id, {
//         dateTime: Math.floor(Date.now() / 1000),
//         weights,
//     });

//     return weights;
// }


/*-------------------------------------------------*/


// import { globalWeights } from "./markov_index.js";

// function pickRandom<T>(items: T[]): T | undefined {
//     if (items.length === 0) return undefined;
//     return items[Math.floor(Math.random() * items.length)];
// }

// export function generateMarkovstr(guildId: string, userId: string): string {
//     const weights = globalWeights.get(guildId)?.get(userId)?.weights;
//     if (!weights) return '';

//     const words: string[] = [];
//     let current = '<START>';

//     for (let i = 0; i < 2; i++) {
//         while (words.length < 50) {
//             const next = pickRandom(weights.get(current) ?? []);
//             if (!next || next === '<END>') break;

//             words.push(next);
//             current = next;
//         }

//         current = '<START>';
//         if (words.length >= 50) break;
//     }

//     return words.join(' ');
// }
