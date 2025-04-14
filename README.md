# ⚡ BOLT12 Offer

A Rust-based backend service to generate and manage [BOLT 12](https://github.com/lightning/bolts/blob/master/12-offer-encoding.md) Lightning Offers, integrated with [Blockstream Greenlight](https://docs.blockstream.com/greenlight/) for non-custodial node operations.

## 🚀 Features

- Generate and decode BOLT 12 offers
- Secure communication with TLS
- Greenlight node registration & authentication
- BIP-353 username support (optional)
- Built with Rust and Axum framework

## 🧰 Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum) web framework
- [TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security) with OpenSSL / Rustls
- [Greenlight SDK](https://docs.blockstream.com/greenlight/)
- [BOLT 12](https://github.com/lightning/bolts/blob/master/12-offer-encoding.md) protocol
  [NEXT JS](https://nextjs.org/)
---

## 🔧 Setup

### Prerequisites

- Rust and Cargo installed (recommended via [rustup](https://rustup.rs))
- OpenSSL installed (especially for Windows users)
- A valid TLS certificate and private key

### 🛠 Environment Variables

Create a `.env` file in the root directory and define:

```env
GL_CERT_PATH=certs/client.crt
GL_KEY_PATH=certs/client-key.pem
```
## Components

- **bolt12-frontend**: Frontend
- **bolt12-backend**: Rust-based service for Greenlight integration


## Project Structure
```
├── bolt12-frontend/        # Next.js frontend for username registration
│   └── src/                
│       ├── app/            # Main application pages and routing
│       └── components/     # UI components like Create Offer form
│
├── bolt12-backend/         # Rust backend integrated with Greenlight
│   └── src/                # Core logic for generating BOLT12 offers
│
└── README.md               # Project overview and setup instructions
```

## Features

- Register Bitcoin usernames with BOLT12 offers
- Automatic DNS record creation using Cloudflare
- Health monitoring via GitHub Actions
- BIP-353 compliant DNS record generation
- Production deployment on Vercel

## Setup Instructions

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/bolt12-offer.git
cd bolt12-offer
```


### 2. bolt12-backend Setup

1. Navigate to the backend directory:
   ```bash
   cd bolt12-backend
   ```

2. Place your Greenlight certificate and key files in the backend directory:
   - `certs/client.crt`
   - `certs/client-key.pem`

3. Build and run the backend:
   ```bash
   cargo run
   ```

### 3. bolt12-frontend Setup

1. Navigate to the backend directory:
   ```bash
   cd bolt12-frontend
   ```

2. Install:
 ```bash
   npm i
   ```

3. Build and run the backend:
   ```bash
   npm run dev
   ```

## Development

- Frontend is running on Port 3000
- Greenlight backend runs on port 8081 by default
- Both services have CORS enabled for cross-origin requests


## License

MIT


