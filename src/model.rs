use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub uid: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
    pub name: String,
}
