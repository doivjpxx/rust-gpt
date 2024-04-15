use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub message: String,
    pub created_at: Option<NaiveDateTime>,
}
