#![no_std]
#![no_main]

use bl602_hal::delay::McycleDelay;
use bl602_hal::pac;
use bl602_hal::prelude::*;

use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let parts = dp.GLB.split();
    let mut blue = parts.pin11.into_pull_down_output();
    let mut green = parts.pin14.into_pull_down_output();
    let mut red = parts.pin17.into_pull_down_output();
    loop {
        red.try_set_high().unwrap();
        green.try_set_high().unwrap();
        blue.try_set_high().unwrap();
        sleep();

        red.try_set_low().unwrap();
        sleep();
        red.try_set_high().unwrap();

        green.try_set_low().unwrap();
        sleep();
        green.try_set_high().unwrap();

        blue.try_set_low().unwrap();
        sleep();
        blue.try_set_high().unwrap();
    }
}

fn sleep() {
    McycleDelay::delay_cycles(50_000_000);
}
