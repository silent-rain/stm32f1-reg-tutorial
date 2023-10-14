//! 系统定时器中断
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use cortex_m::peripheral::syst::SystClkSource;
use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::{entry, exception};
use stm32f1::stm32f103::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    println!("SysTick...");
    // 设置 SysTick 时钟源
    syst.set_clock_source(SystClkSource::Core);

    // 设置 SysTick 重载值
    syst.set_reload(72_000_000);

    // 清除当前 SysTick 值
    syst.clear_current();

    // 使能 SysTick 中断
    syst.enable_interrupt();

    // 使能 SysTick
    syst.enable_counter();

    println!("loop...");
    loop {
        let count = get_count();
        println!("count: {:#?}", count);
    }
}

// 计数器
static mut COUNT: u32 = 0;

#[exception]
fn SysTick() {
    unsafe {
        COUNT += 1;
    }
}

/// 获取计数
fn get_count() -> u32 {
    unsafe { COUNT }
}
