//!LED 流水灯
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(unused)]

use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, syst::delay_ms};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f1::stm32f103::{CorePeripherals, Peripherals, GPIOA, SYST};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let gpioa = &dp.GPIOA;
    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    // 配置引脚为推挽输出模式
    gpioa.crl.modify(|_, w| {
        w.mode0().output50().cnf0().push_pull(); // LED 0
        w.mode1().output50().cnf1().push_pull(); // LED 1
        w.mode2().output50().cnf2().push_pull() // LED 2
    });

    // 定义 LED 连接的引脚号（PA0、PA1）
    let leds: [u16; 3] = [0, 1, 2];

    loop {
        println!("start...");
        for led in leds.into_iter() {
            set_high(gpioa, led);
            delay_ms(&mut syst, 500);
        }
        for led in leds.into_iter() {
            set_low(gpioa, led);
            delay_ms(&mut syst, 500);
        }
    }
}

/// 是否为低电平
fn is_pin_low(gpioa: &GPIOA, pin: u16) -> bool {
    let mask = 1 << pin;
    let bit = gpioa.idr.read().bits() & mask;
    bit == 0
}

/// 是否为高电平
fn is_pin_high(gpioa: &GPIOA, pin: u16) -> bool {
    let mask = 1 << pin;
    let bit = gpioa.idr.read().bits() & mask;
    bit == 1
}

/// 设置引脚为高电平
fn set_high(gpioa: &GPIOA, pin: u16) {
    gpioa.bsrr.write(|w| unsafe { w.bits(1 << pin) });
}

/// 设置引脚为低电平
fn set_low(gpioa: &GPIOA, pin: u16) {
    gpioa.bsrr.write(|w| unsafe { w.bits(1 << (16 + pin)) });
}

/// 翻转引脚电平
fn toggle(gpioa: &GPIOA, pin: u16) {
    if is_pin_low(gpioa, pin) {
        set_high(gpioa, pin)
    } else {
        set_low(gpioa, pin)
    }
}
