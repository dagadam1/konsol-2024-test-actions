// @generated automatically by Diesel CLI.

diesel::table! {
    slides (id) {
        id -> Text,
        caption -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        active -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    slides,
    users,
);
