#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system
    rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOA and GPIOC peripherals
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();

    // Configure PA5 as a push-pull output (LED)
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    // Configure PC13 as an input (Button)
    let button = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);

    loop {
        // Check if the button is pressed (it's active-low, so we check for low state)
        if button.is_low() {
            // Button is pressed, turn on the LED
            led.set_high();
        } else {
            // Button is not pressed, turn off the LED
            led.set_low();
        }

        // Small delay to debounce the button
        for _ in 0..1000 {
            cortex_m::asm::nop();
        }
    }
}
