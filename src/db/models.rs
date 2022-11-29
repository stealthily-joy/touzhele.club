use diesel::prelude::*;
use crate::db::schema::posts;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub text: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub text: &'a str,
}