use async_trait::async_trait;
use axum::{ Router, Extension, extract::Path, Json, routing::{ get, patch } };
use rustycrab_model::response::{ ResponseDataJson, bot_users::ResponseBotUser };
use sea_orm::{ EntityTrait, PrimaryKeyTrait, IntoActiveModel };

use crate::{
    database::bot_users::Model as BotUserModel,
    default_router::DefaultRoutes,
    queries::bot_user_queries::BotUserQueries,
    app_state::AppState,
    utilities::app_error::AppError,
    default_queries::DefaultSeaQueries,
};
pub struct BotUsersRoutes {}

impl BotUsersRoutes {
    async fn get_by_discord_ids(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, user_discord_id)): Path<(String, String)>
    )
        -> Result<Json<ResponseDataJson<<BotUsersRoutes as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model: <<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = <BotUsersRoutes as DefaultRoutes>::Queries::find_by_discord_ids(
            &state.db,
            &bot_discord_id,
            &user_discord_id
        ).await?;
        let response: <BotUsersRoutes as DefaultRoutes>::ResponseJson = <BotUsersRoutes as DefaultRoutes>::ResponseJson::from(
            model
        );

        Ok(Json(ResponseDataJson { data: response }))
    }

    async fn update_by_discord_ids(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, user_discord_id)): Path<(String, String)>,
        Json(
            update_dto,
        ): Json<<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::UpdateData>
    )
        -> Result<Json<ResponseDataJson<<BotUsersRoutes as DefaultRoutes>::ResponseJson>>, AppError>
        where
            <<<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let model: <<<BotUsersRoutes as DefaultRoutes>::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model = <BotUsersRoutes as DefaultRoutes>::Queries::update_by_discord_ids(
            &state.db,
            &bot_discord_id,
            &user_discord_id,
            update_dto
        ).await?;

        let response: <BotUsersRoutes as DefaultRoutes>::ResponseJson = <BotUsersRoutes as DefaultRoutes>::ResponseJson::from(
            model
        );

        Ok(Json(ResponseDataJson { data: response }))
    }
}

#[async_trait]
impl DefaultRoutes for BotUsersRoutes {
    type Queries = BotUserQueries;

    type ResponseJson = ResponseBotUser;

    fn path() -> String {
        format!("bot-users")
    }

    async fn more_routes() -> Router {
        Router::new().nest(
            &format!("/{}", &Self::path()),
            Router::new()
                .route("/:bot_discord_id/:user_discord_id", get(Self::get_by_discord_ids))
                .route("/:bot_discord_id/:user_discord_id", patch(Self::update_by_discord_ids))
        )
    }
}

impl From<BotUserModel> for ResponseBotUser {
    fn from(model: BotUserModel) -> Self {
        Self {
            id: model.id,
            bot_id: model.bot_id,
            user_id: model.user_id,
            balance: model.balance,
            pray_points: model.pray_points,
            inventory: model.inventory,
        }
    }
}
