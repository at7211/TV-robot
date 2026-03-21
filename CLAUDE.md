# TV-Robot

A Rust web application that turns your phone into a remote control for your computer. Scan a QR code to connect, then control play/pause, volume, arrow keys, skip intro, and a touchpad — all from the phone's browser.

## Tech Stack

- **Language**: Rust (edition 2018)
- **Web framework**: Actix-web 4
- **Input simulation**: enigo (keyboard/mouse)
- **Volume control**: macOS `osascript` (platform-specific)
- **Frontend**: Single-page HTML (`public/index.html`) with jQuery, Axios, RxJS, Semantic UI
- **Asset embedding**: rust-embed bundles `public/` into the binary

## Project Structure

```
src/main.rs        — HTTP server, API handlers, input simulation
public/index.html  — Embedded frontend UI
```

## Build & Run

```sh
cargo build          # compile
cargo run            # start server on port 3000
cargo install --path .  # install locally
```

## API Endpoints

- `POST /api/space` — play/pause (spacebar)
- `POST /api/left` / `/api/right` — arrow keys
- `POST /api/volume_up` / `/api/volume_down` — volume control
- `POST /api/skip_intro` — skip intro shortcut
- `POST /api/touch` — touchpad movement (JSON body: `{ dx, dy }`)
