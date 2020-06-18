#![no_std]
#![no_main]

// Halt when the program panics.
extern crate panic_halt;

// Includes.
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f30x_hal as hal;
use hal::prelude::*;
use hal::stm32f30x;

#[entry]
fn main() -> ! {
  // Set up SysTick peripheral.
  let cmp = cortex_m::Peripherals::take().unwrap();
  let mut syst = cmp.SYST;
  syst.set_clock_source( SystClkSource::Core );
  // ~1ms period; STM32F3 resets to 8MHz internal oscillator.
  syst.set_reload( 8_000_000 );
  syst.enable_counter();

  // Set up GPIO pin B13 (LED #1)
  let p = stm32f30x::Peripherals::take().unwrap();
  let mut rcc = p.RCC.constrain();
  let mut gpiob = p.GPIOB.split( &mut rcc.ahb );
  let mut ld4 = gpiob.pb13.into_push_pull_output( &mut gpiob.moder, &mut gpiob.otyper );

  loop {
    while !syst.has_wrapped() {};
    ld4.set_high();
    while !syst.has_wrapped() {};
    ld4.set_low();
  }
}