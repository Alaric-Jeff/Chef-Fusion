# Chef-Fusion

A desktop application built with **Svelte** and **Tauri**.

## Prerequisites

Make sure the following are installed:

- [Node.js](https://nodejs.org/) (LTS recommended)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/#installing-tauri-cli)

```
cargo install create-tauri-app
```

---

## Getting Started

### 1. Frontend Setup

```
cd frontend
npm install
npm run dev
```

> This will start the Vite development server for Svelte.

---

### 2. Tauri Setup (In a New Terminal)

```
cd src-tauri
cargo run
```

> This will compile and run the Tauri application, loading the frontend you started earlier.

---

## Build for Production

To build the full Tauri app:

```
cd frontend
npm run build

cd ../src-tauri
cargo build --release
```

> The final binary will be in the `src-tauri/target/release` folder.

---

## Project Structure

- `frontend/`: Svelte frontend (Vite)
- `src-tauri/`: Tauri backend (Rust)