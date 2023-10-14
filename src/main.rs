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

    // Watchdog Config.
    let mut _watchdog = p.WDT_A.constrain();                                 // Setup WatchdogTimer

    _watchdog.set_timer_interval(TimerInterval::At31);
    _watchdog.feed().unwrap();

    // PCM Config.
    let pcm = p.PCM.constrain()                                              // Setup PCM
        .set_vcore(VCoreSel::DcdcVcore1)                                     // Set DCDC Vcore1 -> 48 MHz Clock
        .freeze();
    let _pcm_sel = pcm.get_powermode();                                      // Get the current powermode

    // Flash Control Config.
    let _flash_control = p.FLCTL.constrain()                                         // Setup Flash
        .set_waitstates(FlashWaitStates::_2)                               // Two wait states -> 48 Mhz Clock
        .freeze();

    let _clock = p.CS.constrain()                                            // Setup CS
        .mclk_dcosource_selection(DCOFrequency::_48MHz, MPrescaler::DIVM_0)  // 48 MHz DCO
        .smclk_prescaler(SMPrescaler::DIVS_1)                                // 24 MHz SMCLK
        .freeze();

    hprintln!("Hello World Example - PCM: {}", _pcm_sel as u32);

    let gpio = p.DIO.split();
    let mut red = gpio.p2_1.into_output();

    let mut tim0 = p.TIMER_A0.constrain().set_clock(_clock);
    let count = Count(10, TimerUnit::Hertz);
    tim0.start(count).unwrap();


    red.set_high().unwrap();

    loop {
        _watchdog.feed().unwrap();
        red.toggle().unwrap();
        block!(tim0.wait()).unwrap();
    }
}


// panic handler
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}