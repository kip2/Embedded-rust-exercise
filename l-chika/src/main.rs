#![no_std]
#![no_main]
use panic_halt as _;
use wio::prelude::*;
use wio_terminal as wio;

#[wio_terminal::entry]
fn main() -> ! {
    // 初期化
    let mut peripherals = wio::pac::Peripherals::take().unwrap();
    let core = wio::pac::CorePeripherals::take().unwrap();
    let mut clocks = wio::hal::clock::GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = wio::hal::delay::Delay::new(core.SYST, &mut clocks);
    let sets = wio::Pins::new(peripherals.PORT).split();

    // 内蔵LED
    let mut user_led = sets.user_led.into_push_pull_output();
    user_led.set_low().unwrap();

    loop {
        // LED点滅
        user_led.toggle().ok();
        delay.delay_ms(200_u16);
    }
}
