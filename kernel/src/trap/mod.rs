use log::info;

mod context;
mod handlers;

pub fn init() {
    handlers::init();
    info!("Trap handler initialized.");
}