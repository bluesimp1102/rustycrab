//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "marriage")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub timestamp: i32,
    #[sea_orm(column_name = "imageURL")]
    pub image_url: Option<String>,
    #[sea_orm(column_name = "thumbnailURL")]
    pub thumbnail_url: Option<String>,
    #[sea_orm(column_name = "ringId")]
    pub ring_id: Option<i32>,
    #[sea_orm(column_name = "user1Id", unique)]
    pub user1_id: Option<i32>,
    #[sea_orm(column_name = "user2Id", unique)]
    pub user2_id: Option<i32>,
    pub caption: String,
    pub quote: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bot_user_info::Entity",
        from = "Column::User1Id",
        to = "super::bot_user_info::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    BotUserInfo2,
    #[sea_orm(
        belongs_to = "super::bot_user_info::Entity",
        from = "Column::User2Id",
        to = "super::bot_user_info::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    BotUserInfo1,
    #[sea_orm(
        belongs_to = "super::items::Entity",
        from = "Column::RingId",
        to = "super::items::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Items,
}

impl Related<super::items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Items.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
