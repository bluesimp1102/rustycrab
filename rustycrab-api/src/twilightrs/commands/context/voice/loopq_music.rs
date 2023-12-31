use std::error::Error;

use async_trait::async_trait;
use rustycrab_model::{ color::ColorResolvables, music::PlayerLoopState };
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::twilightrs::{
    commands::context::{ context_command::{ ContextCommand, GuildConfigModel }, ParsedArg },
    discord_client::DiscordClient,
    utils::reply_command,
};
pub struct LoopQueueMusicCommand {}

#[async_trait]
impl ContextCommand for LoopQueueMusicCommand {
    fn name(&self) -> &'static str {
        "loopq"
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        _: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or("command-guildonly")?;

        let _ = client.voice_music_manager.fetch_call_lock(guild_id).await?;
        client.verify_same_voicechannel(guild_id, msg.author.id).await?;

        let (key, color) = {
            client.voice_music_manager.set_loop_state(guild_id, PlayerLoopState::LoopQueue);
            ("command-loop-queue", ColorResolvables::Green)
        };

        reply_command(&client, config, msg, key, None, color).await?;

        Ok(())
    }
}
