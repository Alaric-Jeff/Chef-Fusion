# 🍳 Chef Fusion

An integrated **native desktop** and **web application** designed for small food service businesses, enabling **offline operations** and **remote access**. Built to streamline workflows, ensure data consistency, and bridge the gap between on-site and cloud-based operations.

---

## 📦 Tech Stack

- **Frontend:** [SolidJS](https://www.solidjs.com/) + Vite
- **Backend:** [Actix Web](https://actix.rs/) (Rust)
- **Desktop Wrapper:** [Tauri](https://tauri.app/) for native executables
- **Databases:**
  - **Local:** SQLite (offline mode)
  - **Cloud:** PostgreSQL (online mode)
- **Containerization:** Docker + Docker Compose
- **Syncing:** HTTP-based API calls (local-first design)

---

## 📂 Folder Structure

```plaintext
project-root/
│
├── docker/                          # Docker-related configuration
│   ├── backend.Dockerfile           # Dockerfile for Actix backend
│   ├── frontend.Dockerfile          # Dockerfile for SolidJS frontend
│   ├── docker-compose.yml           # Orchestrates backend, frontend, and database containers
│   └── scripts/                     # Helper scripts for Docker builds and runs
│
├── backend/                         # Actix Web backend (Rust)
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── src/
│   │   ├── main.rs                   # Entry point
│   │   ├── config.rs                 # Environment/config loader
│   │   ├── routes/                   # API endpoints
│   │   │   ├── mod.rs
│   │   │   ├── inventory.rs
│   │   │   ├── sales.rs
│   │   │   └── sync.rs
│   │   ├── services/                 # Business logic layer
│   │   │   ├── inventory_service.rs
│   │   │   ├── sales_service.rs
│   │   │   └── sync_service.rs
│   │   ├── database/                 # Database connection logic
│   │   │   ├── mod.rs
│   │   │   ├── sqlite.rs
│   │   │   └── postgres.rs
│   │   ├── models/                   # Data models
│   │   │   ├── inventory.rs
│   │   │   ├── sales.rs
│   │   │   └── common.rs
│   │   ├── utils/                    # Helper utilities
│   │   │   ├── mod.rs
│   │   │   └── logger.rs
│   │   └── lib.rs
│   └── .env                          # Backend environment variables
│
├── frontend/                         # SolidJS frontend
│   ├── package.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── index.tsx
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── InventoryTable.tsx
│   │   │   ├── SalesReport.tsx
│   │   │   └── SyncStatus.tsx
│   │   ├── pages/
│   │   │   ├── Home.tsx
│   │   │   ├── Inventory.tsx
│   │   │   ├── Sales.tsx
│   │   │   └── Settings.tsx
│   │   ├── services/                 # API calls to backend
│   │   │   ├── api.ts
│   │   │   ├── inventory.ts
│   │   │   └── sales.ts
│   │   ├── store/                    # Global state management
│   │   │   ├── inventoryStore.ts
│   │   │   └── salesStore.ts
│   │   └── styles/
│   │       └── main.css
│   └── .env                          # Frontend environment variables
│
├── src-tauri/                        # Tauri wrapper
│   ├── tauri.conf.json               # Tauri config
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                   # Tauri entry
│   │   ├── commands.rs               # Tauri command bridge
│   │   ├── offline_sync.rs           # Local-to-cloud sync logic
│   │   └── local_db.rs               # SQLite operations
│
├── scripts/                          # General helper scripts
│   ├── build_all.sh
│   ├── run_dev.sh
│   └── deploy.sh
│
├── .env                              # Global environment variables
├── .gitignore
└── README.md
