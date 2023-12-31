use rustycrab_model::response::logs::setting::{ RequestCreateLogSetting, RequestUpdateLogSetting };
use sea_orm::{ DatabaseConnection, Set, RelationTrait };
use async_trait::async_trait;

use crate::unique_bot_guild_entity_queries::UniqueBotGuildEntityQueries;
use crate::default_queries::DefaultSeaQueries;
use crate::queries::bot_queries::BotQueries;
use crate::queries::guild_queries::GuildQueries;
use crate::{
    database::log_settings::{
        self,
        Model as LogSettingModel,
        Entity as LogSettings,
        ActiveModel as LogSettingActiveModel,
    },
    utilities::app_error::AppError,
};

pub struct LogSettingQueries;

impl UniqueBotGuildEntityQueries for LogSettingQueries {
    fn bot_relation() -> sea_orm::entity::RelationDef {
        log_settings::Relation::Bots.def()
    }
    fn guild_relation() -> sea_orm::entity::RelationDef {
        log_settings::Relation::GuildInfo.def()
    }
}

#[async_trait]
impl DefaultSeaQueries for LogSettingQueries {
    type Entity = LogSettings;
    type ActiveModel = LogSettingActiveModel;

    type CreateData = RequestCreateLogSetting;
    type UpdateData = RequestUpdateLogSetting;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<LogSettingModel, AppError> {
        if
            let Ok(model) = Self::find_by_discord_ids(
                db,
                &create_data.bot_discord_id,
                &create_data.guild_discord_id
            ).await
        {
            return Ok(model);
        }

        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.guild_discord_id).await?;

        let active_model = LogSettingActiveModel {
            bot_id: Set(bot.id),
            guild_id: Set(guild.id),
            ..Default::default()
        };

        Self::save_active_model(db, active_model).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.specify_channels {
            active_model.specify_channels = Set(value);
        }

        if let Some(value) = update_data.new_account_age {
            active_model.new_account_age = Set(value);
        }

        Ok(())
    }
}
