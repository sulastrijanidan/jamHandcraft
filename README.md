# Aplikasi Jual Beli Jam dan Jam Handcrafted

Aplikasi ini memungkinkan pengguna untuk membeli dan menjual jam serta jam handcrafted.

## Fitur Utama:
- Pengguna dapat menambah produk jam/jam handcrafted.
- Pengguna dapat melihat daftar produk yang dijual.
- Pengguna dapat membeli produk.

## Cara Menjalankan

### Backend
1. Masuk ke direktori `backend/`.
2. Bangun dan jalankan backend:
    ```bash
    cargo build --release
    cargo run
    ```

### Frontend
1. Masuk ke direktori `frontend/`.
2. Bangun frontend dengan WebAssembly:
    ```bash
    wasm-pack build --target web
    ```
3. Jalankan server lokal untuk mengakses aplikasi web:
    ```bash
    python3 -m http.server 8080
    ```

Akses aplikasi di `http://localhost:8080`.
