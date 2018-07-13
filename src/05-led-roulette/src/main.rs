#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 500_u16;
    let micro_period = 50_u16;

    loop {
      for idx in 0..4 {
         let next = (idx + 1) % 4;

         leds[idx].on();
         delay.delay_ms(micro_period);
         leds[idx + 4].on();
         delay.delay_ms(half_period);

         leds[idx].off();
         delay.delay_ms(micro_period);
         leds[idx + 4].off();
         delay.delay_ms(half_period);
      }
   }
}
