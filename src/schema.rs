// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        message -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
