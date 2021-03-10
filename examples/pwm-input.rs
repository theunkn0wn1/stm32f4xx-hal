#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal::{prelude::*, pwm, stm32, timer, pwm_input};

#[entry]
fn main() -> ! {
    if let Some(dp) = stm32::Peripherals::take() {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        let channels = (
            gpioa.pa8.into_alternate_af1(),
            gpioa.pa9.into_alternate_af1()
        );
        // configure tim1 as a PWM output of known frequency.
        let pwm = pwm::tim1(dp.TIM1, channels, clocks, 501u32.hz());
        let (mut ch1, _ch2) = pwm;
        let max_duty = ch1.get_max_duty();
        ch1.set_duty(max_duty / 2);
        ch1.enable();
        // configure tim8 as a timer, using the best-guess frequency of the input signal.
        // FIXME: using this for the side effect of configuring the timer, do this ourselves.
        let tim8 = timer::Timer::tim8(dp.TIM8, 500.hz(), clocks).release();

        // Configure tim8's channel 1 pin into AF3
        let pwm_reader_ch1 = gpioc.pc6.into_alternate_af3();
        // instantiate the PWM reader from tim8 and tim8 channel 1.

        let pwm_reader = pwm_input::PwmInput::tim8(tim8, pwm_reader_ch1);

        let duty = pwm_reader.get_duty_cycle();


    }

    loop {
        cortex_m::asm::nop();
    }
}
