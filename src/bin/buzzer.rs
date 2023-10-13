//!蜂鸣器
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, gpio::Gpiob, syst::delay_ms};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1::stm32f103::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let gpiob = &dp.GPIOB;
    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    // 蜂鸣器
    // 配置引脚为推挽输出模式
    gpiob
        .crh
        .modify(|_, w| w.mode12().output50().cnf12().push_pull());
    let buzzer = Gpiob::new(gpiob, 12);

    println!("loop...");
    loop {
        println!("start...");
        // 响
        buzzer.set_low();
        delay_ms(&mut syst, 500);
        // 静
        buzzer.set_high();
        delay_ms(&mut syst, 1000);
    }
}
