use sea_orm::{ DatabaseConnection, Set, EntityTrait, ActiveValue };

use crate::{
    database::{
        messages::{ Entity as Messages, Model as MessageModel, ActiveModel as MessageActiveModel },
        embed_info::Model as EmbedModel,
    },
    routes::{ RequestCreateUpdateMessage, ResponseMessage, ResponseEmbed },
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
};

use super::{ message_embed_queries::{ create_embed, update_embed, get_embed }, save_active_model };

pub async fn get_message(db: &DatabaseConnection, id: &i32) -> Result<MessageModel, AppError> {
    Messages::find_by_id(*id)
        .one(db).await
        .map_err(convert_seaorm_error)?
        .ok_or_else(|| AppError::not_found("Message not found"))
}

pub async fn fetch_message_response(
    db: &DatabaseConnection,
    id: &Option<i32>
) -> Result<Option<ResponseMessage>, AppError> {
    if let Some(msg_id) = id {
        let message = get_message(db, msg_id).await?;

        let embed = if let Some(e_id) = message.embed_id {
            let embed_model = get_embed(db, &e_id).await?;
            Some(ResponseEmbed::from(embed_model)) // Assuming `From` trait is implemented for `ResponseEmbed`
        } else {
            None
        };

        Ok(
            Some(ResponseMessage {
                id: message.id,
                r#type: message.r#type,
                content: message.content,
                embed,
            })
        )
    } else {
        Ok(None) // No message ID provided
    }
}

pub async fn create_message(
    db: &DatabaseConnection,
    create_dto: RequestCreateUpdateMessage
) -> Result<ResponseMessage, AppError> {
    // First, handle the embed creation if it's present in the DTO
    let embed_model: Option<EmbedModel> = if let Some(embed_data) = create_dto.embed {
        Some(create_embed(db, embed_data).await?)
    } else {
        None
    };

    // Now, create the message itself
    let new_message: MessageActiveModel = MessageActiveModel {
        // Assuming `type` and `content` are required fields in the MessageModel.
        // Replace `Set` with the appropriate wrapper or constructor for your fields.
        r#type: Set(create_dto.r#type.unwrap_or_default()),
        content: Set(create_dto.content),
        embed_id: Set(embed_model.as_ref().map(|e| e.id)), // Assuming there's an `embed_id` field linking to the embed
        ..Default::default() // Fill in other default values as necessary
    };

    // Insert the new message into the database
    let message: MessageModel = save_active_model(db, new_message).await?;

    // Construct the response
    let response: ResponseMessage = ResponseMessage {
        id: message.id,
        r#type: message.r#type, // Assuming `type` is wrapped in `Set`
        content: message.content, // Assuming `content` is wrapped in `Set`
        embed: embed_model.map(|e| { e.into() }),
    };

    Ok(response)
}

pub async fn update_message(
    db: &DatabaseConnection,
    id: &i32,
    update_dto: RequestCreateUpdateMessage
) -> Result<ResponseMessage, AppError> {
    let mut message: MessageActiveModel = get_message(db, id).await?.into();

    if let Some(r#type) = update_dto.r#type {
        message.r#type = Set(r#type);
    }
    if let Some(content) = update_dto.content {
        message.content = Set(Some(content));
    }

    // Update embed if provided
    let updated_embed: Option<EmbedModel> = if let Some(embed_data) = update_dto.embed {
        // Check if `message.embed_id` is `ActiveValue::Set(Some(id))`
        if let ActiveValue::Set(Some(e_id)) = message.embed_id {
            Some(update_embed(db, &e_id, embed_data).await?)
        } else {
            Some(create_embed(db, embed_data).await?)
        }
    } else {
        None
    };

    let updated_message: MessageModel = save_active_model(db, message).await?;

    Ok(ResponseMessage {
        id: updated_message.id,
        r#type: updated_message.r#type,
        content: updated_message.content,
        embed: updated_embed.map(|e| { e.into() }),
    })
}

pub async fn delete_message(db: &DatabaseConnection, id: &i32) -> Result<(), AppError> {
    let _ = Messages::delete_by_id(*id)
        .exec(db).await
        .map_err(|err| {
            eprintln!("Error deleting button with id {}: {:?}", id, err);
            AppError::internal_server_error("There was an error deleting the button")
        });

    Ok(())
}