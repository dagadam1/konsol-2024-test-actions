// @generated automatically by Diesel CLI.

diesel::table! {
    slides (id) {
        id -> Text,
        caption -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        active -> Bool,
        filetype -> Text,
    }
}
