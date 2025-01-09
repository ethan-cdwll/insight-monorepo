// src/models/mod.rs
mod token;
mod transaction;
mod wallet;

pub use token::Token;
pub use transaction::Transaction;
pub use wallet::Wallet;

// src/models/wallet.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub address: String,
    pub total_value_usd: f64,
    pub tokens: Vec<TokenBalance>,
    pub risk_score: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    pub token_address: String,
    pub amount: f64,
    pub value_usd: f64,
}

impl Wallet {
    pub fn new(address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            address,
            total_value_usd: 0.0,
            tokens: Vec::new(),
            risk_score: 0.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

// src/models/token.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id")]
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub price_usd: f64,
    pub market_cap_usd: f64,
    pub volume_24h: f64,
    pub price_change_24h: f64,
}

// src/models/transaction.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub signature: String,
    pub block_time: DateTime<Utc>,
    pub success: bool,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub token_address: Option<String>,
    pub fee: u64,
}
