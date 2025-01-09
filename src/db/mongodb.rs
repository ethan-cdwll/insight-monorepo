use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection, Database,
};
use anyhow::Result;
use uuid::Uuid;
use crate::models::{Wallet, Token, Transaction};

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    pub async fn new() -> Result<Self> {
        let mongodb_uri = std::env::var("MONGODB_URI")?;
        let client_options = ClientOptions::parse(&mongodb_uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("insight_wallet");
        
        Ok(Self { db })
    }

    pub async fn init_collections(&self) -> Result<()> {
        // Create indexes for collections
        self.create_wallet_indexes().await?;
        self.create_token_indexes().await?;
        self.create_transaction_indexes().await?;
        Ok(())
    }

    async fn create_wallet_indexes(&self) -> Result<()> {
        let collection = self.db.collection::<Document>("wallets");
        collection
            .create_index(
                doc! {
                    "address": 1
                },
                None,
            )
            .await?;
        Ok(())
    }

    async fn create_token_indexes(&self) -> Result<()> {
        let collection = self.db.collection::<Document>("tokens");
        collection
            .create_index(
                doc! {
                    "address": 1
                },
                None,
            )
            .await?;
        Ok(())
    }

    async fn create_transaction_indexes(&self) -> Result<()> {
        let collection = self.db.collection::<Document>("transactions");
        collection
            .create_index(
                doc! {
                    "signature": 1
                },
                None,
            )
            .await?;
        Ok(())
    }

    // Wallet Operations
    pub async fn get_wallet(&self, id: Uuid) -> Result<Wallet> {
        let collection = self.db.collection::<Wallet>("wallets");
        let wallet = collection
            .find_one(doc! { "_id": id.to_string() }, None)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;
        Ok(wallet)
    }

    pub async fn save_wallet(&self, wallet: &Wallet) -> Result<()> {
        let collection = self.db.collection::<Wallet>("wallets");
        collection
            .replace_one(
                doc! { "_id": wallet.id.to_string() },
                wallet,
                None,
            )
            .await?;
        Ok(())
    }

    // Token Operations
    pub async fn get_token(&self, address: &str) -> Result<Token> {
        let collection = self.db.collection::<Token>("tokens");
        let token = collection
            .find_one(doc! { "address": address }, None)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Token not found"))?;
        Ok(token)
    }

    pub async fn save_token(&self, token: &Token) -> Result<()> {
        let collection = self.db.collection::<Token>("tokens");
        collection
            .replace_one(
                doc! { "address": &token.address },
                token,
                None,
            )
            .await?;
        Ok(())
    }

    // Transaction Operations
    pub async fn save_transaction(&self, transaction: &Transaction) -> Result<()> {
        let collection = self.db.collection::<Transaction>("transactions");
        collection
            .insert_one(transaction, None)
            .await?;
        Ok(())
    }

    pub async fn get_wallet_transactions(
        &self,
        wallet_address: &str,
        limit: i64,
        skip: i64,
    ) -> Result<Vec<Transaction>> {
        let collection = self.db.collection::<Transaction>("transactions");
        let mut cursor = collection
            .find(
                doc! {
                    "$or": [
                        { "from_address": wallet_address },
                        { "to_address": wallet_address }
                    ]
                },
                None,
            )
            .await?;

        let mut transactions = Vec::new();
        while let Some(transaction) = cursor.try_next().await? {
            transactions.push(transaction);
        }
        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_wallet_operations() {
        let db = MongoDB::new().await.unwrap();
        
        let wallet = Wallet {
            id: Uuid::new_v4(),
            address: "test_address".to_string(),
            total_value_usd: 1000.0,
            tokens: vec![],
            risk_score: 0.5,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Test save
        db.save_wallet(&wallet).await.unwrap();

        // Test get
        let retrieved_wallet = db.get_wallet(wallet.id).await.unwrap();
        assert_eq!(wallet.address, retrieved_wallet.address);
    }

    #[tokio::test]
    async fn test_token_operations() {
        let db = MongoDB::new().await.unwrap();
        
        let token = Token {
            address: "test_token".to_string(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            total_supply: 1_000_000_000,
            price_usd: 1.0,
            market_cap_usd: 1_000_000_000.0,
            volume_24h: 1_000_000.0,
            price_change_24h: 5.0,
        };

        // Test save
        db.save_token(&token).await.unwrap();

        // Test get
        let retrieved_token = db.get_token(&token.address).await.unwrap();
        assert_eq!(token.symbol, retrieved_token.symbol);
    }
}