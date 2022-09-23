// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Integer,
        link -> Text,
        title -> Text,
        date_created -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        title -> Varchar,
        url -> Varchar,
        created_at -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    links,
    projects,
);
