#![no_std]
#![no_main]

// Simple UART example

use nrf52840_pac as pac;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    // let p = hal::pac::Peripherals::take().unwrap();
    let r = unsafe { &*pac::CLOCK::ptr() };

    info!("Before HFCLK started");

    r.events_hfclkstarted.write(|w| unsafe { w.bits(0) });
    r.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
    while r.events_hfclkstarted.read().bits() == 0 {}

    info!("After HFCLK started");

    loop {
        cortex_m::asm::wfi();
    }
}
