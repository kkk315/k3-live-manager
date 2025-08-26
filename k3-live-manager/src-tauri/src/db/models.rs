use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// users テーブルの構造体
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// service_credentials テーブルの構造体
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct ServiceCredential {
    pub id: i64,
    pub service_name: String,
    pub client_id: String,
    pub client_secret: String,
}

// oauth_tokens テーブルの構造体
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct OauthToken {
    pub id: i64,
    pub credentials_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
    pub scope: Option<String>,
}

// フロントエンドからデータを受け取るための構造体 (ペイロード)
#[derive(Debug, Deserialize)]
pub struct AddCredentialPayload {
    pub service_name: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AddTokenPayload {
    pub credentials_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: String,
    pub scope: Option<String>,
}