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
