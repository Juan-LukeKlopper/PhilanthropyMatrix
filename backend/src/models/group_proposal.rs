use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct GroupProposal {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub cashout_wallet_address: String,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub about_us: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
