use async_trait::async_trait;
use rustycrab_model::color::ColorResolvables;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, collections::HashMap };

use crate::{
    twilightrs::{
        commands::context::{
            ContextCommand,
            context_command_dispatcher::ContextCommandDispatcher,
            ParsedArg,
            ArgSpec,
            ArgType,
            ContextCommandHandler,
            context_command::GuildConfigModel,
        },
        discord_client::{ DiscordClient, MessageContent },
        messages::{ DiscordEmbed, DiscordEmbedField },
    },
    cdn_guild_icon,
    cdn_avatar,
    queries::bot_queries::BotQueries,
};

pub struct HelpCommand;

impl HelpCommand {
    async fn display_general_help(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        dispatcher: ContextCommandDispatcher
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild = if let Some(guild_id) = msg.guild_id {
            Some(client.get_guild(guild_id).await?)
        } else {
            None
        };
        let bot = client.get_bot().await?;
        let bot_id = bot.id.to_string();
        let bot_info = BotQueries::find_by_discord_id(&client.db, &bot_id).await;
        // General help logic
        // Display a list of commands with brief descriptions
        let mut categories: HashMap<String, Vec<String>> = HashMap::new();

        // let user_permissions = client.cache
        //     .permissions()
        //     .in_channel(msg.author.id, msg.channel_id)?;

        for (command_name, command_handler) in &dispatcher.handlers {
            let category = command_handler.category_name.to_lowercase();
            let command = command_name.to_lowercase();

            // let required_permissions = command_handler.command.permissions();
            // if !required_permissions.is_empty() {
            //     let has_permission = required_permissions
            //         .iter()
            //         .any(|&req_perm| user_permissions.contains(req_perm));
            //     if !has_permission {
            //         // User does not have any of the required permissions
            //         continue;
            //     }
            // }
            categories
                .entry(category)
                .and_modify(|commands| commands.push(command.clone()))
                .or_insert_with(|| vec![command]);
        }
        client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    title: Some(format!("{}'s Commands", bot.name)),
                    description: Some(
                        format!(
                            "More details on a command, use:\n`{}help [command name]`",
                            config.prefix
                        )
                    ),
                    fields: Some(
                        categories
                            .into_iter()
                            // .filter(|x| )
                            .map(|(category, command_names)| {
                                let capitalized_category =
                                    category
                                        .chars()
                                        .next()
                                        .map(|c| c.to_uppercase().collect::<String>())
                                        .unwrap_or_default() + &category[1..];

                                DiscordEmbedField {
                                    name: capitalized_category,
                                    value: command_names
                                        .into_iter()
                                        .filter(|command_name| command_name != "help")
                                        .map(|command_name| format!("`{}` ", command_name))
                                        .collect(),
                                    inline: false,
                                }
                            })
                            .collect()
                    ),
                    author_name: guild.as_ref().map(|guild| guild.name.clone()),
                    author_icon_url: guild
                        .as_ref()
                        .and_then(|guild|
                            guild.icon.map(|icon_hash| cdn_guild_icon!(guild.id, icon_hash))
                        ),
                    thumbnail: bot.avatar.map(|avatar_hash| cdn_avatar!(bot.id, avatar_hash)),
                    timestamp: Some(true),
                    color: if let Ok(info) = bot_info {
                        Some(
                            ColorResolvables::HexString(
                                format!("{}", &info.theme_hex_color)
                            ).as_u32()
                        )
                    } else {
                        None
                    },
                    ..Default::default()
                }]
            )
        ).await?;

        Ok(())
    }

    async fn display_command_help(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_handler: &ContextCommandHandler,
        args: &[String]
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild = if let Some(guild_id) = msg.guild_id {
            Some(client.get_guild(guild_id).await?)
        } else {
            None
        };

        let bot = client.get_bot().await?;
        let bot_info = BotQueries::find_by_discord_id(&client.db, &bot.id.to_string()).await;

        let (command_usage, command_aliases, subcommand_usage, command_description) =
            command_handler.command.get_help(&config.locale, config.prefix.to_string(), args);

        let _ = client.reply_message(
            msg.channel_id,
            msg.id,
            MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    description: command_description,
                    timestamp: Some(true),
                    fields: {
                        let mut discord_fields: Vec<DiscordEmbedField> = Vec::new();
                        discord_fields.push(DiscordEmbedField {
                            name: "Category".to_string(),
                            value: format!(
                                "{}",
                                command_handler.category_name
                                    .chars()
                                    .next()
                                    .map(|c| c.to_uppercase().collect::<String>())
                                    .unwrap_or_default() + &command_handler.category_name[1..]
                            ),
                            inline: false,
                        });
                        // if command_handler.command.aliases
                        if command_aliases.len() > 0 {
                            discord_fields.push(DiscordEmbedField {
                                name: "Aliases".to_string(),
                                value: format!("{}", command_aliases.join(", ")),
                                inline: false,
                            });
                        }
                        if command_handler.command.permissions().len() > 0 {
                            discord_fields.push(DiscordEmbedField {
                                name: "Permission(s)".to_string(),
                                value: format!(
                                    "{}",
                                    command_handler.command
                                        .permissions()
                                        .into_iter()
                                        .map(|perm| format!("{:?}", perm).to_lowercase())
                                        .collect::<Vec<String>>()
                                        .join(", ")
                                ),
                                inline: false,
                            });
                        }
                        discord_fields.push(DiscordEmbedField {
                            name: "Usage".to_string(),
                            value: format!("```fix\n{}```", command_usage),
                            inline: false,
                        });

                        if subcommand_usage.len() > 0 {
                            discord_fields.push(DiscordEmbedField {
                                name: "Subcommands".to_string(),
                                value: format!(
                                    "```fix\n{}```",
                                    subcommand_usage
                                        .into_iter()
                                        .map(|sub| format!("{}{}", config.prefix, sub))
                                        .collect::<Vec<String>>()
                                        .join("\n")
                                ),
                                inline: false,
                            });
                        }
                        Some(discord_fields)
                    },
                    color: if let Ok(info) = bot_info {
                        Some(
                            ColorResolvables::HexString(
                                format!("{}", &info.theme_hex_color)
                            ).as_u32()
                        )
                    } else {
                        None
                    },
                    thumbnail: bot.avatar.map(|avatar_hash|
                        cdn_avatar!(bot.id.to_string(), avatar_hash)
                    ),
                    author_name: guild.as_ref().map(|guild| guild.name.clone()),
                    author_icon_url: guild
                        .as_ref()
                        .and_then(|guild|
                            guild.icon.map(|icon_hash|
                                cdn_guild_icon!(guild.id.to_string(), icon_hash)
                            )
                        ),
                    ..Default::default()
                }]
            )
        ).await?;

        Ok(())
    }
}

#[async_trait]
impl ContextCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new("command", ArgType::Args, true)] // User argument is optional
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let dispatcher = ContextCommandDispatcher::new();

        match command_args.get(0) {
            Some(ParsedArg::Args(args)) if !args.is_empty() => {
                let command_name = &args[0];
                if let Some(command_name) = dispatcher.commands_aliases.get(command_name) {
                    if let Some(command_handler) = dispatcher.handlers.get(command_name) {
                        let _ = self.display_command_help(
                            client,
                            config,
                            msg,
                            command_handler,
                            &args[1..]
                        ).await?;
                        return Ok(());
                    }
                }
            }
            _ => {}
        }

        self.display_general_help(client, config, msg, dispatcher).await?;
        Ok(())
    }
}
