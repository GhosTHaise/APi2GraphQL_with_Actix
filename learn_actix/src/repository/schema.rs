// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Integer,
        link -> Text,
        title -> Text,
        date_created -> Text,
    }
}
