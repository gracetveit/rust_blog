table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        slug -> Varchar,
    }
}
