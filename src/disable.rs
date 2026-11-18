// import path from "node:path";
// import fs from "node:fs";
// import { fileURLToPath } from "node:url";

// const __filename = fileURLToPath(import.meta.url);
// const __dirname = path.dirname(__filename);

// export const disabledPath = path.join(__dirname, "", "..", "data", "disabled.json");
static FILE: LazyLock<File> = LazyLock::new(||{File::open("./actionCommands.json").expect("Could not open actionCommands.json");});

pub fn fetch_cache_json() {
    serde_json::from_reader(BufReader::new(FILE))
        .expect("Error while reading or parsing actionCommands.json");
}

// export type DisabledMap = Record<string, string[]>;

// export let disabledCache: DisabledMap = {};

// export function initializeDisabledCache(): void {
//     try {
//         if (!fs.existsSync(disabledPath)) {
//             fs.writeFileSync(disabledPath, JSON.stringify({}), "utf8");
//         }
//         const data = fs.readFileSync(disabledPath, "utf8");
//         const parsed = JSON.parse(data);
//         disabledCache = parsed !== null && typeof parsed === "object" ? parsed : {};
//     } catch (error) {
//         console.error("Failed to load disabled file during initialization:", error);
//         disabledCache = {};
//     }
// }

// export function updateDisabledCache(newData: DisabledMap): void {
//     disabledCache = newData;
//     try {
//         fs.writeFileSync(disabledPath, JSON.stringify(newData, null, 2), "utf8");
//     } catch (error) {
//         console.error("Failed to write to disabled commands file:", error);
//     }
// }


/*-----------------------------------------*/


// import { type ChatInputCommandInteraction, InteractionContextType, PermissionFlagsBits, SlashCommandBuilder } from "discord.js";
// import { disabledCache, updateDisabledCache } from "../../functions/commandCache.js";

// export default {
//     data: new SlashCommandBuilder()
//         .setName('disable')
//         .setDescription('Disable a command in this server')
//         .addStringOption(option =>
//             option
//                 .setName('command')
//                 .setDescription('The command to disable')
//                 .setRequired(true)
//         )
//         .setContexts(InteractionContextType.Guild)
//         .setDefaultMemberPermissions(PermissionFlagsBits.Administrator),

//     async execute(interaction: ChatInputCommandInteraction) {
//         await interaction.deferReply({});
//         const server = interaction.guildId;
//         const command = interaction.options.getString('command');

//         if (command === 'disable' || command === 'enable') return await interaction.editReply('You cannot disable this command.');
//         if (!command) return await interaction.editReply('Command not found');
//         if (!server) return await interaction.editReply('Guild not found');

//         const updatedDisabled = { ...disabledCache };
//         if (!updatedDisabled[server]) updatedDisabled[server] = [];

//         if (updatedDisabled[server].includes(command)) return await interaction.editReply('Command already disabled.');

//         updatedDisabled[server].push(command);

//         updateDisabledCache(updatedDisabled);

//         return await interaction.editReply(`Successfully disabled ${command}`);
//     }
// }


/*-----------------------------------------*/


// import { type ChatInputCommandInteraction, InteractionContextType, PermissionFlagsBits, SlashCommandBuilder } from "discord.js";
// import { disabledCache, updateDisabledCache } from "../../functions/commandCache.js";

// export default {
//     data: new SlashCommandBuilder()
//         .setName('enable')
//         .setDescription('Enable a command in this server')
//         .addStringOption(option =>
//             option
//                 .setName('command')
//                 .setDescription('The command to enable')
//                 .setRequired(true)
//         )
//         .setContexts(InteractionContextType.Guild)
//         .setDefaultMemberPermissions(PermissionFlagsBits.Administrator),

//     async execute(interaction: ChatInputCommandInteraction) {
//         await interaction.deferReply({});
//         const server = interaction.guildId;
//         const command = interaction.options.getString('command');

//         if (!command) return await interaction.editReply('Command not found');
//         if (!server) return await interaction.editReply('Guild not found');

//         const updatedDisabled = { ...disabledCache };
//         if (!updatedDisabled[server]) updatedDisabled[server] = [];

//         if (updatedDisabled[server].includes(command)) {
//             updatedDisabled[server] = updatedDisabled[server].filter((cmd: string) => cmd !== command);

//             updateDisabledCache(updatedDisabled);

//             return await interaction.editReply(`Successfully enabled ${command}`);
//         } else {
//             return interaction.editReply('Command is not disabled.');
//         }
//     }
// }
