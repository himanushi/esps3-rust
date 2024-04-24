use std::thread;
use std::time::Duration;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("start!!!");

    loop {
        thread::sleep(Duration::from_millis(1000));
        log::info!("heyhey!!!");
    }
}
