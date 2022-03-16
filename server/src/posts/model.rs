#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub slug: String,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub slug: String,
}

pub impl NewPost {
    pub fn new(title: String, body: String) -> NewPost {
        use urlencoding;
        NewPost {
            title,
            body,
            slug: urlencoding::encode(title),
        }
    }
}
