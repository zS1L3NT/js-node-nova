// @generated automatically by Diesel CLI.

diesel::table! {
    configs (filename) {
        filename -> Text,
        shorthand -> Text,
        content -> Text,
    }
}

diesel::table! {
    secrets (project, path) {
        project -> Text,
        path -> Text,
        content -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(configs, secrets);
