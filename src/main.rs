mod app;
mod components;
mod config;
mod models;
mod pages;
mod services;
mod state;
mod utils;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
