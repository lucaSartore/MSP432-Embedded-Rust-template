#![no_main]
#![no_std]


use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use nb::block;

use msp432p401r as pac;
use msp432p401r_hal as hal;

use hal::{clock::*, flash::*, gpio::*, pcm::*, timer::*, watchdog::*};



#[entry]
// main
fn main() -> ! {

    // Take the Peripherals
    let p = pac::Peripherals::take().unwrap();

    // disable wd timer to avoid reser
    let wd = p.WDT_A.constrain();
    let _ = wd.disable().unwrap();

    hprintln!("A simple blink example");

    let gpio = p.DIO.split();
    let mut red = gpio.p2_0.into_output();


    for _ in 0..10 {
        red.toggle().unwrap();
        hprintln!("A simple blink example");
    }
    panic!("my_panic");
}


// panic handler
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hprintln!("PANIC: {:?}",_info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}