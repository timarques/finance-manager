#![cfg_attr(
    target_os = "windows",
    windows_subsystem = "windows"
)]

mod pages;
mod app;
mod data;
mod metadata;
mod prelude;
mod window;
mod context;
mod utils;

fn main() {
    app::App::new().init();
}
