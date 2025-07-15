# 🎬 rust_netflix

A full-stack video streaming app built with **Rust** and **Leptos**.

---

## 📁 Project Structure

rust_netflix/
├── frontend/ # Leptos (WASM) frontend app
├── media/ # HLS video files (e.g. .m3u8, .ts)
├── src/ # Axum backend source
├── Cargo.toml # Rust project config
└── README.md


---

## 🚀 Running the App

### 1️⃣ Start the Frontend (Leptos + Trunk)

```bash
cd frontend
trunk serve

This will build and serve the Leptos app at:
👉 http://localhost:8080

Start the Backend (Axum API Server)
cd ..
cargo run
This runs the Axum server (video streaming) at:
👉 http://localhost:8000
