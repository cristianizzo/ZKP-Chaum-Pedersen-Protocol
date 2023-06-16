// @generated automatically by Diesel CLI.

diesel::table! {
    auth (user_id) {
        user_id -> Text,
        r1 -> Text,
        r2 -> Text,
        c -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Text,
        y1 -> Text,
        y2 -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(auth, users,);
