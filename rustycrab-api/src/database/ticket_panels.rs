//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ticket_panels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "mentionOnOpen", column_type = "Text")]
    pub mention_on_open: String,
    #[sea_orm(column_name = "namingScheme")]
    pub naming_scheme: String,
    #[sea_orm(column_name = "channelID")]
    pub channel_id: String,
    #[sea_orm(column_name = "ticketCategory")]
    pub ticket_category: String,
    #[sea_orm(column_name = "sentMessageID")]
    pub sent_message_id: String,
    #[sea_orm(column_name = "botId")]
    pub bot_id: i32,
    #[sea_orm(column_name = "guildId")]
    pub guild_id: i32,
    #[sea_orm(column_name = "messageId", unique)]
    pub message_id: i32,
    #[sea_orm(column_name = "buttonId", unique)]
    pub button_id: i32,
    #[sea_orm(column_name = "welcomeMessageId", unique)]
    pub welcome_message_id: i32,
    #[sea_orm(column_name = "supportTeamId")]
    pub support_team_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bots::Entity",
        from = "Column::BotId",
        to = "super::bots::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Bots,
    #[sea_orm(
        belongs_to = "super::buttons::Entity",
        from = "Column::ButtonId",
        to = "super::buttons::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Buttons,
    #[sea_orm(
        belongs_to = "super::guild_info::Entity",
        from = "Column::GuildId",
        to = "super::guild_info::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GuildInfo,
    #[sea_orm(
        belongs_to = "super::messages::Entity",
        from = "Column::WelcomeMessageId",
        to = "super::messages::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Messages2,
    #[sea_orm(
        belongs_to = "super::messages::Entity",
        from = "Column::MessageId",
        to = "super::messages::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Messages1,
    #[sea_orm(has_many = "super::ticket_multi_panels_panels_ticket_panels::Entity")]
    TicketMultiPanelsPanelsTicketPanels,
    #[sea_orm(
        belongs_to = "super::ticket_support_teams::Entity",
        from = "Column::SupportTeamId",
        to = "super::ticket_support_teams::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    TicketSupportTeams,
}

impl Related<super::bots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bots.def()
    }
}

impl Related<super::buttons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Buttons.def()
    }
}

impl Related<super::guild_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuildInfo.def()
    }
}

impl Related<super::ticket_multi_panels_panels_ticket_panels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TicketMultiPanelsPanelsTicketPanels.def()
    }
}

impl Related<super::ticket_support_teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TicketSupportTeams.def()
    }
}

impl Related<super::ticket_multi_panels::Entity> for Entity {
    fn to() -> RelationDef {
        super::ticket_multi_panels_panels_ticket_panels::Relation::TicketMultiPanels.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::ticket_multi_panels_panels_ticket_panels::Relation::TicketPanels
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
