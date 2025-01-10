#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{otg_fs::Usb, prelude::*};
use log::info;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

extern crate alloc;

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER)
        .split::<esp_hal::timer::systimer::Target>();
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timer1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // TODO: Spawn some tasks
    let _ = spawner;

    // set USB as keyboard input
    // let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
    // let mut ep_out_buffer = [0u8; 1024];
    // let config = esp_hal::otg_fs::asynch::Config::default();

    // let driver = esp_hal::otg_fs::asynch::Driver::new(usb, &mut ep_out_buffer, config);

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.22.0/examples/src/bin
}
