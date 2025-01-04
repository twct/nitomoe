// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Int8,
        slug -> Text,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
