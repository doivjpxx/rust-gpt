use axum::{extract::State, http::StatusCode, Json};

use crate::{
    models::{Message, NewMessage},
    services::messages_service,
};

pub async fn create_message(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_post): Json<NewMessage>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let new_message_db = NewMessage {
        message: new_post.message,
    };

    let created_message = messages_service::create_message(&pool, new_message_db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create message: {:?}", e);
            e
        })?;

    Ok(Json(created_message))
}

pub async fn get_messages(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let messages = messages_service::get_messages(&pool).await.map_err(|e| {
        tracing::error!("Failed to get messages: {:?}", e);
        e
    })?;

    Ok(Json(messages))
}
