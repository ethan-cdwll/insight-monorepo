use anyhow::Result;
use rust_bert::pipelines::sequence_classification::SequenceClassificationModel;
use crate::models::{Token, Wallet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAnalysis {
    pub risk_score: f64,
    pub diversity_score: f64,
    pub recommendations: Vec<String>,
    pub token_insights: HashMap<String, TokenInsight>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAnalysis {
    pub sentiment_score: f64,
    pub price_prediction: PricePrediction,
    pub market_sentiment: MarketSentiment,
    pub technical_indicators: TechnicalIndicators,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInsight {
    pub risk_level: RiskLevel,
    pub concentration: f64,
    pub suggested_action: Action,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Hold,
    Buy,
    Sell,
    ReduceExposure,
    IncreasePosition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricePrediction {
    pub price_24h: f64,
    pub price_7d: f64,
    pub price_30d: f64,
    pub confidence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketSentiment {
    pub overall_score: f64,
    pub social_sentiment: f64,
    pub news_sentiment: f64,
    pub trading_volume_sentiment: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnicalIndicators {
    pub rsi: f64,
    pub macd: MACD,
    pub moving_averages: MovingAverages,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MACD {
    pub value: f64,
    pub signal: f64,
    pub histogram: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovingAverages {
    pub ma_20: f64,
    pub ma_50: f64,
    pub ma_200: f64,
}

pub struct AIService {
    model: SequenceClassificationModel,
    historical_data: HashMap<String, Vec<HistoricalDataPoint>>,
}

struct HistoricalDataPoint {
    timestamp: chrono::DateTime<chrono::Utc>,
    price: f64,
    volume: f64,
}

impl AIService {
    pub async fn new() -> Result<Self> {
        let model = SequenceClassificationModel::new(Default::default())?;
        Ok(Self { 
            model,
            historical_data: HashMap::new(),
        })
    }

    pub async fn analyze_wallet(&self, wallet: &Wallet) -> Result<WalletAnalysis> {
        let mut token_insights = HashMap::new();
        let mut total_value = 0.0;
        
        // Analyze each token in the wallet
        for token_balance in &wallet.tokens {
            total_value += token_balance.value_usd;
            let insight = self.analyze_token_position(token_balance).await?;
            token_insights.insert(token_balance.token_address.clone(), insight);
        }

        // Calculate portfolio metrics
        let risk_score = self.calculate_risk_score(wallet).await?;
        let diversity_score = self.calculate_diversity_score(wallet, total_value).await?;
        let recommendations = self.generate_recommendations(wallet, &token_insights).await?;

        Ok(WalletAnalysis {
            risk_score,
            diversity_score,
            recommendations,
            token_insights,
        })
    }

    async fn analyze_token_position(&self, token_balance: &TokenBalance) -> Result<TokenInsight> {
        let concentration = token_balance.value_usd / total_portfolio_value;
        let risk_level = self.determine_risk_level(concentration, token_balance).await?;
        let suggested_action = self.suggest_action(risk_level, concentration).await?;

        Ok(TokenInsight {
            risk_level,
            concentration,
            suggested_action,
        })
    }

    async fn calculate_risk_score(&self, wallet: &Wallet) -> Result<f64> {
        let mut risk_score = 0.0;
        let total_value = wallet.tokens.iter().map(|t| t.value_usd).sum::<f64>();

        for token in &wallet.tokens {
            let concentration = token.value_usd / total_value;
            let token_volatility = self.calculate_token_volatility(&token.token_address).await?;
            risk_score += concentration * token_volatility;
        }

        Ok(risk_score.min(1.0))
    }

    async fn calculate_diversity_score(&self, wallet: &Wallet, total_value: f64) -> Result<f64> {
        let mut herfindahl_index = 0.0;

        for token in &wallet.tokens {
            let weight = token.value_usd / total_value;
            herfindahl_index += weight * weight;
        }

        // Convert Herfindahl index to diversity score (1 - HHI)
        Ok(1.0 - herfindahl_index)
    }

    async fn generate_recommendations(
        &self,
        wallet: &Wallet,
        token_insights: &HashMap<String, TokenInsight>,
    ) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Check portfolio concentration
        if wallet.tokens.len() < 5 {
            recommendations.push("Consider diversifying your portfolio across more assets".to_string());
        }

        // Analyze high-risk exposures
        for (token_addr, insight) in token_insights {
            if matches!(insight.risk_level, RiskLevel::High | RiskLevel::VeryHigh) 
                && insight.concentration > 0.2 {
                recommendations.push(
                    format!("Consider reducing exposure to token {}", token_addr)
                );
            }
        }

        // Add general recommendations based on portfolio metrics
        if self.calculate_diversity_score(wallet, wallet.total_value_usd).await? < 0.5 {
            recommendations.push("Portfolio is highly concentrated. Consider rebalancing.".to_string());
        }

        Ok(recommendations)
    }

    pub async fn analyze_token(&self, token: &Token) -> Result<TokenAnalysis> {
        let sentiment_score = self.calculate_sentiment_score(token).await?;
        let price_prediction = self.predict_token_price(token).await?;
        let market_sentiment = self.analyze_market_sentiment(token).await?;
        let technical_indicators = self.calculate_technical_indicators(token).await?;

        Ok(TokenAnalysis {
            sentiment_score,
            price_prediction,
            market_sentiment,
            technical_indicators,
        })
    }

    async fn calculate_sentiment_score(&self, token: &Token) -> Result<f64> {
        // Analyze market data and social sentiment
        let price_sentiment = self.analyze_price_trend(token).await?;
        let volume_sentiment = self.analyze_volume_trend(token).await?;
        let social_sentiment = self.analyze_social_metrics(token).await?;

        // Weighted average of different sentiment factors
        Ok((price_sentiment * 0.4 + volume_sentiment * 0.3 + social_sentiment * 0.3)
            .max(0.0)
            .min(1.0))
    }

    async fn predict_token_price(&self, token: &Token) -> Result<PricePrediction> {
        let historical_data = self.get_historical_data(&token.address).await?;
        
        // Use time series analysis for predictions
        let (price_24h, conf_24h) = self.forecast_price(&historical_data, 24)?;
        let (price_7d, conf_7d) = self.forecast_price(&historical_data, 168)?;
        let (price_30d, conf_30d) = self.forecast_price(&historical_data, 720)?;

        Ok(PricePrediction {
            price_24h,
            price_7d,
            price_30d,
            confidence: (conf_24h + conf_7d + conf_30d) / 3.0,
        })
    }

    async fn analyze_market_sentiment(&self, token: &Token) -> Result<MarketSentiment> {
        let social_sentiment = self.analyze_social_metrics(token).await?;
        let news_sentiment = self.analyze_news_sentiment(token).await?;
        let volume_sentiment = self.analyze_volume_trend(token).await?;

        let overall_score = (social_sentiment + news_sentiment + volume_sentiment) / 3.0;

        Ok(MarketSentiment {
            overall_score,
            social_sentiment,
            news_sentiment,
            trading_volume_sentiment: volume_sentiment,
        })
    }

    async fn calculate_technical_indicators(&self, token: &Token) -> Result<TechnicalIndicators> {
        let historical_data = self.get_historical_data(&token.address).await?;
        
        let rsi = self.calculate_rsi(&historical_data)?;
        let macd = self.calculate_macd(&historical_data)?;
        let moving_averages = self.calculate_moving_averages(&historical_data)?;

        Ok(TechnicalIndicators {
            rsi,
            macd,
            moving_averages,
        })
    }

    // Helper methods for technical analysis
    fn calculate_rsi(&self, data: &[HistoricalDataPoint]) -> Result<f64> {
        if data.len() < 14 {
            return Ok(50.0);
        }

        let mut gains = Vec::new();
        let mut losses = Vec::new();

        for i in 1..data.len() {
            let price_change = data[i].price - data[i-1].price;
            if price_change >= 0.0 {
                gains.push(price_change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-price_change);
            }
        }

        let avg_gain = gains.iter().sum::<f64>() / gains.len() as f64;
        let avg_loss = losses.iter().sum::<f64>() / losses.len() as f64;

        if avg_loss == 0.0 {
            return Ok(100.0);
        }

        let rs = avg_gain / avg_loss;
        Ok(100.0 - (100.0 / (1.0 + rs)))
    }

    fn calculate_macd(&self, data: &[HistoricalDataPoint]) -> Result<MACD> {
        let ema_12 = self.calculate_ema(data, 12)?;
        let ema_26 = self.calculate_ema(data, 26)?;
        let macd_value = ema_12 - ema_26;
        let signal = self.calculate_ema(&data[14..], 9)?;
        let histogram = macd_value - signal;

        Ok(MACD {
            value: macd_value,
            signal,
            histogram,
        })
    }

    fn calculate_moving_averages(&self, data: &[HistoricalDataPoint]) -> Result<MovingAverages> {
        Ok(MovingAverages {
            ma_20: self.calculate_sma(data, 20)?,
            ma_50: self.calculate_sma(data, 50)?,
            ma_200: self.calculate_sma(data, 200)?,
        })
    }

    fn calculate_ema(&self, data: &[HistoricalDataPoint], period: usize) -> Result<f64> {
        if data.len() < period {
            return Ok(data.last().map(|p| p.price).unwrap_or(0.0));
        }

        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut ema = data[0].price;

        for point in data.iter().skip(1) {
            ema = (point.price - ema) * multiplier + ema;
        }

        Ok(ema)
    }

    fn calculate_sma(&self, data: &[HistoricalDataPoint], period: usize) -> Result<f64> {
        if data.len() < period {
            return Ok(data.last().map(|p| p.price).unwrap_or(0.0));
        }

        let sum: f64 = data.iter().rev().take(period).map(|p| p.price).sum();
        Ok(sum / period as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_wallet_analysis() {
        let service = AIService::new().await.unwrap();
        let wallet = Wallet {
            id: uuid::Uuid::new_v4(),
            address: "test_wallet".to_string(),
            total_value_usd: 1000.0,
            tokens: vec![
                TokenBalance {
                    token_address: "token1".to_string(),
                    amount: 100.0,
                    value_usd: 500.0,
                },
                TokenBalance {
                    token_address: "token2".to_string(),
                    amount: 200.0,
                    value_usd: 500.0,
                },
            ],
            risk_score: 0.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let analysis = service.analyze_wallet(&wallet).await.unwrap();
        assert!(analysis.risk_score >= 0.0 && analysis.risk_score <= 1.0);
        assert!(analysis.diversity_score >= 0.0 && analysis.diversity_score <= 1.0);
        assert!(!analysis.recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_token_analysis() {
        let service = AIService::new().await.unwrap();
        let token = Token {
            address: "test_token".to_string(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 18,
            total_supply: 1_000_000,
            price_usd: 1.0,
            market_cap_usd: 1_000_000.0,
            volume_24h: 100_000.0,
            price_change_24h: 5.0,
        };

        let analysis = service.analyze_token(&token).await.unwrap();
        assert!(analysis.sentiment_score >= 0.0 && analysis.sentiment_score <= 1.0);
        assert!(analysis.market_sentiment.overall_score >= 0.0 && analysis.market_sentiment.overall_score <= 1.0);
    }
}