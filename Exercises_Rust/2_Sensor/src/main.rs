// std and main are not available for bare metal software
#![no_main]
#![no_std]

// Panic handler
use panic_halt as _;
use cortex_m_rt::entry;

// HAL library for stm32f4xx board
use stm32f4xx_hal::{
    spi::{Mode, NoMosi, Phase, Polarity, Spi},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {

    // Get access to device and core peripherals
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    // Get access to RCC 
    let rcc = device.RCC.constrain();
    // Set the sysclock and freeze it
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    //  Get access to GPIOA
    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();

    // Set sclk, cs and miso for spi protocol
    let sclk = gpiob.pb10.into_alternate();
    let miso = gpiob.pb14.into_alternate();
    let mut cs = gpiob.pb12.into_push_pull_output();
    // Set the led pin
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {

    }
}