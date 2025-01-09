use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod db;
mod models;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();

    info!("Starting Insight Wallet Analysis Platform...");

    // Initialize database connection
    let db = db::mongodb::init_database().await.expect("Failed to connect to database");

    // Initialize blockchain client
    let blockchain_client = services::blockchain::SolanaClient::new()
        .await
        .expect("Failed to initialize blockchain client");

    // Initialize AI service
    let ai_service = services::ai_analysis::AIService::new()
        .await
        .expect("Failed to initialize AI service");

    // Create shared application state
    let app_state = web::Data::new(AppState {
        db: db.clone(),
        blockchain_client: blockchain_client.clone(),
        ai_service: ai_service.clone(),
    });

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(app_state.clone())
            .configure(api::routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(num_cpus::get())
    .run()
    .await
}

pub struct AppState {
    db: mongodb::Database,
    blockchain_client: Arc<services::blockchain::SolanaClient>,
    ai_service: Arc<services::ai_analysis::AIService>,
}