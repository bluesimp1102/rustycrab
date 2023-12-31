use std::{ time::SystemTime, error::Error };

use regex::Regex;
use sea_orm::DbErr;
use twilight_http::Client;

use crate::{ cdn_guild_icon, cdn_avatar };

use super::app_error::AppError;

/// Converts a SeaORM database error into an application-specific error.
pub fn convert_seaorm_error(err: DbErr) -> AppError {
    eprintln!("Database error: {:?}", err); // Make sure to use the log crate to log the error.

    // You can add specific matches for different error types if needed.
    // For example, if you have unique constraint violations that might be caused by client input,
    // you can return a 400 error instead.
    match err {
        DbErr::Query(query_error) => {
            // Handle specific query errors if necessary
            AppError::internal_server_error(format!("Database query error: {}", query_error))
        }
        DbErr::RecordNotFound(_) => {
            // This might happen due to a client error, if they reference a non-existent record
            AppError::not_found("The requested record does not exist.")
        }
        // Add more matches as necessary for different kinds of errors
        _ => {
            // For any other database error, return a 500 internal server error
            AppError::internal_server_error(
                "An internal error occurred while accessing the database."
            )
        }
    }
}

use twilight_model::{
    channel::message::component::ButtonStyle,
    id::{ Id, marker::{ GuildMarker, UserMarker } },
    guild::Guild,
    user::User,
};

pub fn color_to_button_style(color: &str) -> ButtonStyle {
    match color.to_lowercase().as_str() {
        "red" => ButtonStyle::Danger,
        "green" => ButtonStyle::Success,
        "blue" => ButtonStyle::Primary,
        "gray" | "grey" => ButtonStyle::Secondary,
        "link" => ButtonStyle::Link,
        _ => ButtonStyle::Secondary, // Default case
    }
}

use chrono::{ DateTime, Utc };

pub fn process_placeholders_sync(
    text: String,
    guild: &Option<Guild>,
    user: &Option<User>
) -> String {
    let mut replaced_text = text;
    if let Some(guild) = guild {
        replaced_text = replaced_text.replace("{server}", &guild.name);

        if let Some(hash) = &guild.icon {
            replaced_text = replaced_text.replace(
                "{server-icon}",
                cdn_guild_icon!(guild.id, hash).as_str()
            );
        }
        replaced_text = replaced_text.replace("{server-id}", &guild.id.to_string());
    }

    if let Some(user) = user {
        replaced_text = replaced_text
            .replace("{user-id}", &user.id.to_string())
            .replace("{user}", &format!("<@{}>", user.id.to_string()));
        if let Some(hash) = user.avatar {
            replaced_text = replaced_text.replace("{avatar}", cdn_avatar!(user.id, hash).as_str());
        }
        replaced_text = replaced_text.replace("{username}", &user.name);

        // replace {account-age}
        let creation_date = DateTime::<Utc>
            ::from_timestamp(((user.id.get() >> 22) as i64) + 1_420_070_400_000, 0 as u32)
            .unwrap_or_else(|| Utc::now());
        let account_age = Utc::now().signed_duration_since(creation_date);
        let account_age_str = format!("{} days", account_age.num_days());
        replaced_text = replaced_text.replace("{account-age}", &account_age_str);
    }

    replaced_text = replaced_text.replace("{everyone}", "@everyone");
    replaced_text = replaced_text.replace("{here}", "@here");

    replaced_text
}

pub async fn process_placeholders(
    http: &Client,
    text: String,
    guild_id: &Option<Id<GuildMarker>>,
    user_id: &Option<Id<UserMarker>>
) -> String {
    let mut replaced_text = text;
    if let Some(guild_id) = guild_id {
        replaced_text = replaced_text.replace("{server-id}", &guild_id.to_string());
    }
    if let Some(user_id) = user_id {
        replaced_text = replaced_text
            .replace("{user-id}", &user_id.to_string())
            .replace("{user}", &format!("<@{}>", user_id.to_string()));
    }
    let guild = if let Some(guild_id) = guild_id {
        if let Ok(guild) = http.guild(guild_id.clone()).await {
            guild.model().await.ok()
        } else {
            None
        }
    } else {
        None
    };

    let user = if let Some(user_id) = user_id {
        if let Ok(user) = http.user(user_id.clone()).await {
            if let Ok(user) = user.model().await { Some(user) } else { None }
        } else {
            None
        }
    } else {
        None
    };

    replaced_text = process_placeholders_sync(replaced_text, &guild, &user);
    replaced_text
}

pub fn current_unix_timestamp() -> Result<u32, Box<dyn Error + Send + Sync>> {
    // Convert SystemTime to Timestamp
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as u32)
}

pub fn validate_image_url(url: &str) -> bool {
    // Updated regex pattern to allow for optional characters after the file extension
    let image_extension_pattern = Regex::new(r"\.(jpg|jpeg|png|gif|bmp|webp)(\?.*)?$").unwrap();

    // Check if the URL matches the pattern
    image_extension_pattern.is_match(url)
}

use csscolorparser::Color;

pub fn parse_colorhex(input: &str) -> Option<String> {
    // Try parsing the input directly
    let color = input.parse::<Color>().or_else(|_| {
        // If direct parsing fails, try with a "#" prefix
        format!("#{}", input).parse::<Color>()
    });

    // Convert the parsed color to a hex string if successful
    color.ok().map(|c| c.to_hex_string())
}
