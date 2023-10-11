//!汇编延迟
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1::stm32f103::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let _dp = Peripherals::take().unwrap();
    let _cp = CorePeripherals::take().unwrap();

    loop {
        for i in 0..10 {
            println!("i={:?}", i);
            // 延时一秒
            delay_s(1);
        }
    }
}

/// 定义一个简单的延时函数，使用忙等待的方式
/// 延时大约一秒钟
pub fn delay_s(cycles: u32) {
    for _ in 0..cycles * 8_000_000 {
        cortex_m::asm::nop();
    }
}
