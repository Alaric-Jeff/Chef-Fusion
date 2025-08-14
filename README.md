# Chef Fusion

An integrated native desktop and web application solution, enabling **offline operations** and **remote access**, specifically developed to streamline and modernize the workflows of small food service businesses.

---

## 📦 Tech Stack

- **Frontend:** [SolidJS](https://www.solidjs.com/) + Vite
- **Backend:** [Actix Web](https://actix.rs/) (Rust)
- **Desktop Wrapper:** [Tauri](https://tauri.app/) for native executables
- **Databases:**
  - **Local:** SQLite (offline mode)
  - **Cloud:** PostgreSQL (online mode)
- **Containerization:** Docker + Docker Compose
- **Syncing:** HTTP-based API calls, local-first design

---

## 📂 Folder Structure

project-root/
│
├── docker/                          # Docker-related configuration
│   ├── backend.Dockerfile           # Dockerfile for Actix backend
│   ├── frontend.Dockerfile          # Dockerfile for SolidJS frontend
│   ├── docker-compose.yml           # Orchestrates backend, frontend, and database containers
│   └── scripts/                     # Helper scripts for building and running containers
│
├── backend/                         # Actix web server (Rust)
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── src/
│   │   ├── main.rs                   # Entry point
│   │   ├── config.rs                 # Config loading (env vars, db URLs, etc.)
│   │   ├── routes/                   # All API routes
│   │   │   ├── mod.rs
│   │   │   ├── inventory.rs
│   │   │   ├── sales.rs
│   │   │   └── sync.rs
│   │   ├── services/                 # Business logic
│   │   │   ├── inventory_service.rs
│   │   │   ├── sales_service.rs
│   │   │   └── sync_service.rs
│   │   ├── database/                 # DB connection logic
│   │   │   ├── mod.rs
│   │   │   ├── sqlite.rs
│   │   │   └── postgres.rs
│   │   ├── models/                   # Data models
│   │   │   ├── inventory.rs
│   │   │   ├── sales.rs
│   │   │   └── common.rs
│   │   ├── utils/                    # Helpers and utility functions
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
│   │   ├── services/                 # API calls to Actix backend
│   │   │   ├── api.ts
│   │   │   ├── inventory.ts
│   │   │   └── sales.ts
│   │   ├── store/                    # Global state (SolidJS store)
│   │   │   ├── inventoryStore.ts
│   │   │   └── salesStore.ts
│   │   └── styles/
│   │       └── main.css
│   └── .env                          # Frontend environment variables
│
├── src-tauri/                        # Tauri project files
│   ├── tauri.conf.json               # Tauri configuration
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                   # Tauri entry point
│   │   ├── commands.rs               # Tauri commands to bridge backend and frontend
│   │   ├── offline_sync.rs           # Sync logic when app goes online
│   │   └── local_db.rs               # SQLite local operations
│
├── scripts/                          # General helper scripts (non-Docker)
│   ├── build_all.sh
│   ├── run_dev.sh
│   └── deploy.sh
│
├── .env                              # Global environment variables
├── .gitignore
└── README.md
