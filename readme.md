# Insight Wallet

An AI-driven crypto wallet analysis platform built in Rust.

## 🚀 Features

- Real-time wallet analysis
- AI-powered portfolio recommendations
- Token performance tracking
- Historical transaction analysis
- Mobile-responsive web interface
- Solana blockchain integration

## 📁 Project Structure

```
insight-wallet/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   └── handlers.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── wallet.rs
│   │   ├── token.rs
│   │   └── transaction.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── ai_analysis.rs
│   │   ├── blockchain.rs
│   │   └── portfolio.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── mongodb.rs
│   └── utils/
│       ├── mod.rs
│       └── helpers.rs
├── tests/
│   ├── api_tests.rs
│   ├── service_tests.rs
│   └── integration_tests.rs
└── docs/
    ├── API.md
    └── DEPLOYMENT.md
```

## 🛠️ Technology Stack

- Rust 1.75+
- Actix-web for HTTP server
- MongoDB for data storage
- tokio for async runtime
- serde for serialization
- solana-sdk for blockchain interaction

## 📦 Installation

1. Clone the repository:
```bash
git clone https://github.com/insight/insight-wallet.git
cd insight-wallet
```

2. Install dependencies:
```bash
cargo build
```

3. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

4. Run the project:
```bash
cargo run
```

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

Run with coverage:
```bash
cargo tarpaulin
```

## 📚 Documentation

- [API Documentation](docs/API.md)
- [Deployment Guide](docs/DEPLOYMENT.md)

## 🤝 Contributing

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 📞 Contact

- Website: https://insight.io
- Twitter: @insight_wallet
- Email: support@insight.io