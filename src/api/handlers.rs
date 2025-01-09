use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::{models::{Wallet, Token}, AppState};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct WalletAnalysisRequest {
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct WalletAnalysisResponse {
    pub wallet: Wallet,
    pub analysis: WalletAnalysis,
    pub recommendations: Vec<String>,
}

pub async fn analyze_wallet(
    data: web::Json<WalletAnalysisRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.blockchain_client.get_wallet_tokens(&data.address).await {
        Ok(tokens) => {
            let wallet = Wallet {
                id: Uuid::new_v4(),
                address: data.address.clone(),
                tokens,
                ..Default::default()
            };

            match state.ai_service.analyze_wallet(&wallet).await {
                Ok(analysis) => {
                    let response = WalletAnalysisResponse {
                        wallet,
                        analysis,
                        recommendations: vec![
                            "Consider diversifying your portfolio".to_string(),
                            "Reduce exposure to high-risk tokens".to_string(),
                        ],
                    };
                    HttpResponse::Ok().json(response)
                }
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenAnalysisRequest {
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct TokenAnalysisResponse {
    pub token: Token,
    pub analysis: TokenAnalysis,
    pub price_prediction: PricePrediction,
}

pub async fn analyze_token(
    data: web::Json<TokenAnalysisRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.blockchain_client.get_token_info(&data.address).await {
        Ok(token) => {
            match state.ai_service.analyze_token(&token).await {
                Ok(analysis) => {
                    let price_prediction = state.ai_service.predict_token_price(&token).await?;
                    let response = TokenAnalysisResponse {
                        token,
                        analysis,
                        price_prediction,
                    };
                    HttpResponse::Ok().json(response)
                }
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_portfolio_metrics(
    wallet_id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.db.get_wallet(wallet_id.into_inner()).await {
        Ok(wallet) => {
            let metrics = state.portfolio_service.calculate_metrics(&wallet).await?;
            HttpResponse::Ok().json(metrics)
        }
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

pub async fn get_transaction_history(
    wallet_id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.db.get_wallet(wallet_id.into_inner()).await {
        Ok(wallet) => {
            let transactions = state.blockchain_client
                .get_transactions(&wallet.address)
                .await?;
            HttpResponse::Ok().json(transactions)
        }
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_analyze_wallet() {
        let app_state = web::Data::new(AppState::new().await);
        let req = test::TestRequest::post()
            .set_json(&WalletAnalysisRequest {
                address: "test_address".to_string(),
            })
            .to_http_request();

        let resp = analyze_wallet(web::Json(req.into()), app_state).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_analyze_token() {
        let app_state = web::Data::new(AppState::new().await);
        let req = test::TestRequest::post()
            .set_json(&TokenAnalysisRequest {
                address: "test_token".to_string(),
            })
            .to_http_request();

        let resp = analyze_token(web::Json(req.into()), app_state).await;
        assert!(resp.status().is_success());
    }
}