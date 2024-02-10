// @generated automatically by Diesel CLI.

diesel::table! {
    configs (filename) {
        filename -> Varchar,
        shorthand -> Varchar,
        content -> Text,
    }
}

diesel::table! {
    secrets (project, path) {
        project -> Varchar,
        path -> Varchar,
        content -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(configs, secrets);
