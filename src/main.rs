use std::process::Command;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use enigo::*;
use serde::Deserialize;

use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

// ref: https://github.com/pyros2097/rust-embed/blob/master/examples/actix.rs
fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn press(key: enigo::Key) {
    let mut en = Enigo::new();
    en.key_click(key);
}

fn moving(x: i32, y: i32) {
    let mut en = Enigo::new();
    en.mouse_move_to(x, y);
}

fn get_volume() -> i8 {
    let script = "osascript -e 'output volume of (get volume settings)'";
    let output = Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to get volume");

    let vol = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .parse()
        .expect("failed to parse volume into integer");

    return vol;
}

fn set_volume(vol: i8) {
    let script = format!("osascript -e 'set Volume output volume {}'", vol);
    Command::new("sh")
        .arg("-c")
        .arg(script)
        .spawn()
        .expect("failed to spawn process")
        .wait()
        .unwrap();
}

async fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

async fn press_space() -> impl Responder {
    press(Key::Space);
    "Ok"
}

async fn press_left() -> impl Responder {
    press(Key::LeftArrow);
    "Ok"
}

async fn press_right() -> impl Responder {
    press(Key::RightArrow);
    "Ok"
}

async fn volume_down() -> impl Responder {
    let current_vol = get_volume();
    set_volume(current_vol - 7);
    "Ok"
}

async fn volume_up() -> impl Responder {
    let current_vol = get_volume();
    set_volume(current_vol + 7);
    "Ok"
}

async fn skip_intro() -> impl Responder {
    press(Key::Raw(0x01));
    "Ok"
}

#[derive(Deserialize)]
struct TouchParams {
    x: i32,
    y: i32,
}

async fn touch(params: web::Json<TouchParams>) -> impl Responder {
    moving(params.x, params.y);
    HttpResponse::Ok().body("Ok")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ip = local_ip::get().unwrap().to_string();
    let url = format!("http://{}:3000/", ip);
    qr2term::print_qr(&url).unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/space", web::post().to(press_space))
            .route("/api/left", web::post().to(press_left))
            .route("/api/right", web::post().to(press_right))
            .route("/api/volume_down", web::post().to(volume_down))
            .route("/api/volume_up", web::post().to(volume_up))
            .route("/api/skip_intro", web::post().to(skip_intro))
            .route("/api/touch", web::post().to(touch))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
