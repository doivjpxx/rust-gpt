use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{Message, MessageRequest, NewMessage},
    services::{messages_service, vertex_service::GeminiResponse},
};

pub async fn create_message(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_message): Json<MessageRequest>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let reponse = crate::services::vertex_service::get_gpt_message(&new_message.message)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get gemini message: {:?}", e);
            e
        });

    let gemini_message = match reponse {
        Ok(res) => res
            .candidates
            .iter()
            .map(|c| c.content.parts[0].text.clone())
            .collect::<String>(),
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let new_message = NewMessage {
        given_message: new_message.message,
        message: gemini_message,
    };

    let created_message = messages_service::create_message(&pool, new_message)
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

pub async fn get_gemini_message(
    Path(message): Path<String>,
) -> Result<Json<GeminiResponse>, (StatusCode, String)> {
    let reponse = crate::services::vertex_service::get_gpt_message(&message)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get gemini message: {:?}", e);
            e
        });

    match reponse {
        Ok(res) => Ok(Json(GeminiResponse {
            contents: res
                .candidates
                .iter()
                .map(|c| c.content.parts[0].text.clone())
                .collect::<String>(),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
