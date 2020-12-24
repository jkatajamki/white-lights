table! {
    wl_users (user_id) {
        user_id -> Int8,
        created_at -> Timestamp,
        email -> Text,
        user_secret -> Text,
        updated_at -> Nullable<Timestamp>,
    }
}
