#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut blueled = Output::new(p.PC13, Level::Low, Speed::Low);
    let mut redled = Output::new(p.PC14, Level::Low, Speed::Low);
    let mut greenled = Output::new(p.PC15, Level::Low, Speed::Low);

    loop {
        info!("high");
        redled.set_low();
        blueled.set_high();
        greenled.set_high();
        Timer::after_millis(300).await;

        info!("low");
        redled.set_high();
		blueled.set_low();
		greenled.set_low();
        Timer::after_millis(300).await;
    }
}
