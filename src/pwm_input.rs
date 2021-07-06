use crate::timer::PinC1;
macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $en_bit:expr, $reset_bit:expr, $apbenr:ident, $apbrstr:ident, $bits:ident),)+) => {
        $(
            // Drag the associated TIM object into scope.
            // Note: its drawn in via the macro to avoid duplicating the feature gate
            // this macro is expecting to be guarded by.
            use crate::stm32::$TIM;

            impl<PINS> PwmInput<$TIM, PINS> {
                #[allow(unused_unsafe)] // some TIMs mark the bits operation safe, other's do not.
                /// Configures a TIM peripheral as a quadrature encoder interface input
                pub fn $tim(tim: $TIM, pins: PINS) -> Self
                where
                    PINS: Pins<$TIM>
                {
                    // Select the active input for TIMx_CCR1: write the CC1S bits to 01 in the TIMx_CCMR1
                    // register (TI1 selected).
                    tim.ccmr1_input().modify(|_, w| unsafe {
                        w.cc1s().bits(0b01)
                    });

                    // Select the active polarity for TI1FP1 (used both for capture in TIMx_CCR1 and counter
                    // clear): write the CC1P and CC1NP bits to ‘0’ (active on rising edge).

                    tim.ccer.modify(|_, w| {
                        w.cc1p().clear_bit().cc2p().clear_bit()
                    });

                    // disable filters and disable the prescalers.
                    tim.ccmr1_input().modify(|_, w| unsafe {
                        w.ic1f().bits(0).ic2f().bits(0)
                            .ic1psc().bits(0).ic2psc().bits(0)
                    });

                    // Select the active input for TIMx_CCR2: write the CC2S bits to 10 in the TIMx_CCMR1
                    // register (TI1 selected)
                    tim.ccmr1_input().modify(|_, w| unsafe {
                        w.cc2s().bits(0b01)
                    });

                    // Select the active polarity for TI1FP2 (used for capture in TIMx_CCR2): write the CC2P
                    // and CC2NP bits to ‘1’ (active on falling edge).
                    tim.ccer.modify(|_, w| {
                        w.cc2p().set_bit().cc2np().set_bit()
                    });

                    // Select the valid trigger input: write the TS bits to 101 in the TIMx_SMCR register
                    // (TI1FP1 selected).
                    tim.smcr.modify(|_, w| unsafe {
                        w.ts().bits(0b101)
                    });

                    // Configure the slave mode controller in reset mode: write the SMS bits to 100 in the
                    // TIMx_SMCR register.
                    tim.smcr.modify(|_, w| unsafe {
                        w.sms().bits(0b100)
                    }
                    );

                    // Enable the captures: write the CC1E and CC2E bits to ‘1’ in the TIMx_CCER register.
                    tim.ccer.modify(|_, w| {
                        w.cc1e().set_bit().cc2e().set_bit()
                    });

                    // enable interrupts.
                    tim.dier.modify(|_, w| {
                        w.cc2ie().set_bit()
                    });
                    tim.cr1.modify(|_, w| { w.cen().enabled() });

                    PwmInput { tim, pins }                }

                /// Releases the TIM peripheral and QEI pins
                pub fn release(self) -> ($TIM, PINS) {
                    (self.tim, self.pins)
                }

                /// Period of PWM signal in terms of clock cycles
                pub fn get_period_clocks(&self) -> $bits {
                    // TODO: express in terms of hz
                    self.tim.ccr1.read().ccr().bits()
                }
                // Duty cycle in terms of clock cycles
                pub fn get_duty_cycle_clocks(&self) -> $bits {
                    // TODO: express in terms of % duty
                    self.tim.ccr2.read().ccr().bits()
                }

                pub fn get_duty_cycle(&self) -> f32 {
                    if self.get_period_clocks() == 0 {
                        return 0.0;
                    };
                    return (self.get_duty_cycle_clocks() as f32 / self.get_period_clocks() as f32) * 100f32;
                }
            } )+
}}



pub struct PwmInput<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

pub trait Pins<TIM> {}

// implement the `Pins` trait wherever PC1 implements PinC1 and PC2 implements PinC2 for the given TIMer
impl<TIM, PC1> Pins<TIM> for PC1 where PC1: PinC1<TIM> {}


#[cfg(any(
feature = "stm32f411",
))]
/* red group */
hal! {
    TIM4: (tim4, 2, 2, apb1enr, apb1rstr, u16),
    TIM3: (tim3, 1, 1, apb1enr, apb1rstr, u16),
    TIM2: (tim2, 0, 0, apb1enr, apb1rstr, u32),
}

/* orange group */
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
hal! {
    TIM8: (tim8, 1, 1, apb2enr, apb2rstr, u16),
    TIM2: (tim2, 0, 0, apb1enr, apb1rstr, u32),
    TIM3: (tim3, 1, 1, apb1enr, apb1rstr, u16),
    TIM4: (tim4, 2, 2, apb1enr, apb1rstr, u16),
}
/* green group */
#[cfg(any(
feature = "stm32f405",
feature = "stm32f407",
feature = "stm32f412",
feature = "stm32f413",
feature = "stm32f415",
feature = "stm32f417",
feature = "stm32f423",
feature = "stm32f427",
feature = "stm32f429",
feature = "stm32f437",
feature = "stm32f439",
feature = "stm32f446",
feature = "stm32f469",
feature = "stm32f479",
))]
hal!{
        TIM12: (tim12, 6, 6, apb1enr, apb1rstr, u16),
}


/* every chip across the series have these timers with support for this feature.
 .. except for the 410 which, while the timers support this feature, has a different configuration
    than the rest of the series.
 */
/* yellow group */
#[cfg(not(feature="stm32f410"))]
hal! {
    TIM1: (tim1, 0, 0, apb2enr, apb2rstr, u16),
    TIM5: (tim5, 3, 3, apb1enr, apb1rstr, u32),
    TIM9: (tim9, 16, 16, apb2enr, apb2rstr, u16),
}
