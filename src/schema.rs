// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        given_message -> Text,
        message -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
