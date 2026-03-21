# TV-Robot

A Rust web application that turns your phone into a remote control for your computer. Scan a QR code to connect, then control play/pause, volume, arrow keys, skip intro, and a touchpad — all from the phone's browser.

## Tech Stack

- **Language**: Rust (edition 2018)
- **Web framework**: Actix-web 4
- **Input simulation**: enigo (keyboard/mouse)
- **Volume control**: macOS `osascript` (platform-specific)
- **Frontend**: Single-page HTML (`public/index.html`) with vanilla JS, Semantic UI CSS
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

### Remote Control
- `POST /api/space` — play/pause (spacebar)
- `POST /api/left` / `/api/right` — arrow keys
- `POST /api/volume_up` / `/api/volume_down` — volume control

### Streaming Shortcuts
- `POST /api/fullscreen` — toggle fullscreen (F key)
- `POST /api/mute` — toggle mute (M key)
- `POST /api/skip_intro` — skip intro (S key, Netflix)
- `POST /api/captions` — toggle captions (C key, YouTube)

### Mouse Control
- `POST /api/mouse/move` — relative mouse movement (JSON: `{ dx, dy }`)
- `POST /api/mouse/left_click` / `/api/mouse/right_click` — mouse clicks
- `POST /api/mouse/scroll` — scroll (JSON: `{ dy }`)
