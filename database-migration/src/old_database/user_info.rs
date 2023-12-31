//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_info")]
pub struct Model {
    #[sea_orm(column_name = "discordID", primary_key, auto_increment = false, unique)]
    pub discord_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bot_user_info::Entity")]
    BotUserInfo,
}

impl Related<super::bot_user_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BotUserInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
