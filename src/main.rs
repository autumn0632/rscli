use log::{debug, error, info, trace};

pub mod cmd;
pub mod custom_logger;
pub mod input;
pub mod utils;

fn main() {
    custom_logger::init().unwrap();
    info!("start ddi3cli");
    input::set_custom_render_config();
    input::pares_stdin();
}
