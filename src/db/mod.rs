pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::db::models::{NewPost, Post};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(text: &str) -> Post {
    use crate::db::schema::posts;

    let connection = &mut establish_connection();
    let new_post = NewPost { text };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn list_post() -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    posts.limit(5)
        .order(id.desc())
        .load::<Post>(connection)
        .expect("Error loading posts")
}