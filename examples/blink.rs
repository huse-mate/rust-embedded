#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(dead_code)]


use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_probe as _;
use stm32f4xx_hal::{gpio::GpioExt, pac::Peripherals, prelude::{_stm32f4xx_hal_rcc_RccExt, _stm32f4xx_hal_timer_SysCounterExt}};
use switch_hal::{IntoSwitch, OutputSwitch};



#[entry]
fn main() -> ! {
    let d_periphs = Peripherals::take().unwrap();
    let gpioa = d_periphs.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output().into_active_high_switch();
    
    let cortex_p = cortex_m::peripheral::Peripherals::take().unwrap();
    let clocks = d_periphs.RCC.constrain().cfgr.freeze();
    let mut delay = cortex_p.SYST.delay(&clocks);

    
    loop {
        led.on().ok();
        delay.delay_ms(500_u32);
        led.off().ok();
        delay.delay_ms(500_u32);
    }
}
