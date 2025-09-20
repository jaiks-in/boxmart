pub mod components;
pub mod helper_functions;
pub mod pages;
pub mod utils;
use leptos::*;
use leptos::mount::mount_to_body;

pub mod app;

fn main() {
    // Leptos app ko mount karo
    mount_to_body(app::App);
}
