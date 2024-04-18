use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub message: String,
    pub given_message: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct MessageRequest {
    pub message: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::messages)]
pub struct NewMessage {
    pub message: String,
    pub given_message: String,
}
