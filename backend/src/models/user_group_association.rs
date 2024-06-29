use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserGroup {
    pub user_id: i32,
    pub group_id: i32,
    pub is_admin: bool,
}
