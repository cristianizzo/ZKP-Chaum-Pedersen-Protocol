use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub user_id: String,
    pub y1: String,
    pub y2: String,
}
