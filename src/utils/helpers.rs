use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Common error type for the application
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] mongodb::error::Error),

    #[error("Blockchain error: {0}")]
    Blockchain(String),

    #[error("AI service error: {0}")]
    AIService(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

// Time-related helper functions
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

// Numeric helper functions
pub fn calculate_percentage_change(old_value: f64, new_value: f64) -> f64 {
    if old_value == 0.0 {
        return 0.0;
    }
    ((new_value - old_value) / old_value) * 100.0
}

pub fn format_usd(amount: f64) -> String {
    format!("${:.2}", amount)
}

// Token-related helper functions
pub fn format_token_amount(amount: u64, decimals: u8) -> f64 {
    amount as f64 / 10f64.powi(decimals as i32)
}

pub fn parse_token_amount(amount_str: &str, decimals: u8) -> Result<u64, AppError> {
    let parts: Vec<&str> = amount_str.split('.').collect();
    match parts.len() {
        1 => {
            let whole = parts[0]
                .parse::<u64>()
                .map_err(|e| AppError::InvalidInput(format!("Invalid amount: {}", e)))?;
            Ok(whole * 10u64.pow(decimals as u32))
        }
        2 => {
            let whole = parts[0]
                .parse::<u64>()
                .map_err(|e| AppError::InvalidInput(format!("Invalid amount: {}", e)))?;
            let decimal = parts[1]
                .parse::<u64>()
                .map_err(|e| AppError::InvalidInput(format!("Invalid amount: {}", e)))?;
            Ok(whole * 10u64.pow(decimals as u32) + decimal)
        }
        _ => Err(AppError::InvalidInput("Invalid amount format".to_string())),
    }
}

// Risk calculation helpers
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub volatility: f64,
    pub concentration: f64,
    pub liquidity: f64,
}
