// @generated automatically by Diesel CLI.

diesel::table! {
    post_image (id) {
        id -> Int4,
        path -> Varchar,
        post_id -> Int4,
    }
}

diesel::table! {
    post_video (id) {
        id -> Int4,
        path -> Varchar,
        post_id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        text -> Text,
    }
}

diesel::joinable!(post_image -> posts (post_id));
diesel::joinable!(post_video -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    post_image,
    post_video,
    posts,
);
