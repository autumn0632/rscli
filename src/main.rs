pub mod cmd;
pub mod input;
pub mod utils;

fn main() {
    input::set_custom_render_config();
    input::pares_stdin();
}
