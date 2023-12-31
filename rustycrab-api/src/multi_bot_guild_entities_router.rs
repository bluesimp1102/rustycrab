use async_trait::async_trait;
use axum::{ Router, Extension, extract::Path, Json, routing::get };
use rustycrab_model::response::ResponseDataList;
use sea_orm::{ EntityTrait, IntoActiveModel, PrimaryKeyTrait };

use crate::{
    default_router::DefaultRoutes,
    app_state::AppState,
    default_queries::DefaultSeaQueries,
    utilities::app_error::AppError,
    multi_bot_guild_entities_queries::MultipleBotGuildEntityQueries,
};

/// The `MultipleBotGuildEntitiesRoutes` trait extends the `DefaultRoutes` trait to include specialized
/// routes and behaviors for entities related to both bots and guilds, typically in a Discord context.
/// It integrates with `MultipleBotGuildEntityQueries` to facilitate CRUD operations specific to bot-guild associations.
///
/// ## Requirements
/// - Implementing structs must already implement `DefaultRoutes`.
/// - The associated queries (`Self::Queries`) must implement `BotGuildEntityQueries`.
///
/// ## Specialized Routes
/// This trait provides additional route handlers specifically designed for entities associated with
/// both bots and guilds. These routes allow fetching and updating entities based on Discord IDs for
/// both bots and guilds.
#[async_trait]
pub trait MultipleBotGuildEntitiesRoutes
    : DefaultRoutes
    where Self::Queries: MultipleBotGuildEntityQueries
{
    /// Retrieves a single instance of the entity associated with specific Discord IDs for a bot and a guild.
    ///
    /// ### Parameters
    /// - `Extension(state)`: The shared application state, typically containing a database connection.
    /// - `Path((bot_discord_id, guild_discord_id))`: The Discord IDs for the bot and the guild, extracted from the path.
    ///
    /// ### Type Constraints
    /// - The primary key value type of the entity must be convertible from `i32`.
    /// - The entity model must be convertible into its corresponding active model.
    ///
    /// ### Returns
    /// - `Result<Json<ResponseDataList<Self::ResponseJson>>, AppError>`: On success, returns a JSON response containing the retrieved entities. On failure, returns an `AppError`.
    ///
    /// ### Description
    /// This function performs a database query to find an entity that is linked with both a specific bot and guild, identified
    async fn get_by_discord_ids(
        Extension(state): Extension<AppState>,
        Path((bot_discord_id, guild_discord_id)): Path<(String, String)>
    )
        -> Result<Json<ResponseDataList<Self::ResponseJson>>, AppError>
        where
            Self::Queries: MultipleBotGuildEntityQueries,
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<Self::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let models: Vec<<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::Model> =
            Self::Queries::find_by_discord_ids(
                &state.db,
                &bot_discord_id,
                &guild_discord_id
            ).await?;
        let response: Vec<Self::ResponseJson> = models
            .into_iter()
            .map(Self::ResponseJson::from)
            .collect::<Vec<Self::ResponseJson>>();

        Ok(Json(ResponseDataList { data: response }))
    }

    /// Retrieves a single instance of the entity associated with specific Discord IDs for a bot and a guild.
    ///
    /// ### Parameters
    /// - `Extension(state)`: The shared application state, typically containing a database connection.
    /// - `Path((bot_discord_id, guild_discord_id))`: The Discord IDs for the bot and the guild, extracted from the path.
    ///
    /// ### Type Constraints
    /// - The primary key value type of the entity must be convertible from `i32`.
    /// - The entity model must be convertible into its corresponding active model.
    ///
    /// ### Returns
    /// - `Result<Json<ResponseDataJson<Self::ResponseJson>>, AppError>`: On success, returns a JSON response containing the retrieved entity. On failure, returns an `AppError`.
    ///
    /// ### Description
    /// This function performs a database query to find an entity that is linked with both a specific bot and guild, identified
    async fn bot_guild_router() -> Router
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<Self::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let path = Self::path();
        Router::new().route(
            &format!("/{}/:bot_discord_id/:guild_discord_id", &path),
            get(Self::get_by_discord_ids)
        )
    }

    /// Creates a comprehensive router combining the default CRUD routes, additional custom routes, and bot-guild-specific routes.
    ///
    /// ### Parameters
    /// - `state`: The shared application state, typically including a database connection and other shared resources.
    ///
    /// ### Type Constraints
    /// - The primary key value type of the entity must be convertible from `i32`.
    /// - The entity model must be convertible into its corresponding active model.
    ///
    /// ### Returns
    /// - `Router`: An `axum::Router` instance combining all routes.
    ///
    /// ### Description
    /// This function assembles a complete router by merging the default CRUD routes, any additional custom routes defined in `more_routes`,
    /// and bot-guild-specific routes from `bot_guild_router`. It ensures comprehensive coverage of all required endpoints for the application.
    async fn router() -> Router
        where
            <<<Self::Queries as DefaultSeaQueries>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
            <<Self::Queries as DefaultSeaQueries>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<<Self::Queries as DefaultSeaQueries>::ActiveModel>
    {
        let default_routes = Self::default_router().await;
        let custom_routes = Self::more_routes().await;
        let bot_guild_routes = Self::bot_guild_router().await;

        default_routes.merge(custom_routes).merge(bot_guild_routes)
    }
}
