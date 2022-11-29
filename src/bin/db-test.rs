use diesel::pg::PgConnection;
use touzhele::db::models::{NewPost, Post};
use touzhele::db::establish_connection;
use diesel::prelude::*;

pub fn create_post(conn: &mut PgConnection, text: &str) -> Post {
    use touzhele::db::schema::posts;

    let new_post = NewPost { text };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    let connection = &mut establish_connection();

    let mut text = "墨尔本州立图书馆前 民众举行悼念活动";

    let post = create_post(connection, &text);
    println!("\nSaved draft with id {}", post.id);
}
