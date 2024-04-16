use axum::http::StatusCode;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    models::{Message, NewMessage},
    schema::messages::{self},
};

pub async fn create_message(
    pool: &deadpool_diesel::postgres::Pool,
    new_message: NewMessage,
) -> Result<Message, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_server_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(messages::table)
                .values(new_message)
                .returning(Message::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_server_error)?
        .map_err(internal_server_error)?;

    Ok(res)
}

pub async fn get_messages(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<Vec<Message>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_server_error)?;

    let res = conn
        .interact(|conn| {
            messages::table
                .select(messages::all_columns)
                .load::<Message>(conn)
        })
        .await
        .map_err(internal_server_error)?
        .map_err(internal_server_error)?;

    Ok(res)
}

fn internal_server_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
