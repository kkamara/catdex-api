use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Cat {
    pub id: u64,
    pub name: String,
    pub image_path: String,
}
