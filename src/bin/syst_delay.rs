//!系统定时器延迟
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1::stm32f103::{CorePeripherals, Peripherals};
use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, syst::delay_ms};

#[entry]
fn main() -> ! {
    // 从外围设备访问机箱访问设备特定的外围设备
    let dp = Peripherals::take().unwrap();
    // 从 cortex-m 机箱访问核心外围设备
    let cp = CorePeripherals::take().unwrap();

    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    println!("配置时钟树");
    set_clock(rcc);

    loop {
        for i in 0..10 {
            println!("i={:?}", i);
            // 延时一秒
            delay_ms(&mut syst, 1000);
        }
    }
}
