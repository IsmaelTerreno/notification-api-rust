// @generated automatically by Diesel CLI.

diesel::table! {
    notification (id) {
        id -> Uuid,
        topic -> Varchar,
        body -> Text,
        read -> Bool,
        user_id -> Nullable<Varchar>,
    }
}
