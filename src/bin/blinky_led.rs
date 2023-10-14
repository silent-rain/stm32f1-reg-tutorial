//!闪烁 LED 灯
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

// 定义 LED 连接的引脚号（PA1）
const LED_PIN: u16 = 1;

#[entry]
fn main() -> ! {
    // 从外围设备访问机箱访问设备特定的外围设备
    let dp = Peripherals::take().unwrap();
    // 从 cortex-m 机箱访问核心外围设备
    let cp = CorePeripherals::take().unwrap();

    let gpioa = &dp.GPIOA;
    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    println!("配置时钟树");
    set_clock(rcc);

    // 启用 APB2 GPIOB 的时钟
    rcc.apb2enr.modify(|_, w| w.iopben().set_bit());

    // 启用 APB2 GPIOA 的时钟
    rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

    // 配置引脚为推挽输出模式
    gpioa.crl.modify(|_, w| {
        w
            // Port n.1 mode bits
            .mode1()
            // Output mode 50 MHz
            .output50()
            // Port n.1 configuration bits
            .cnf1()
            // Analog mode / Push-Pull mode
            .push_pull()
    });

    loop {
        println!("start...");

        // 翻转 LED 的状态
        toggle(gpioa, LED_PIN);

        // 延时2秒钟
        delay_ms(&mut syst, 2000);
    }
}

/// 是否为低电平
fn is_low(gpioa: &GPIOA) -> bool {
    gpioa.idr.read().idr1().is_low()
}

/// 是否为低电平
fn is_pin_low(gpioa: &GPIOA, pin: u16) -> bool {
    // 获取 PA1 引脚对应的 IDR 寄存器中的位的掩码
    let mask = 1 << pin;
    // 读取 IDR 寄存器中对应的位的值
    // 如果该位的值为 0，那么说明该引脚为低电平；如果该位的值为 1，那么说明该引脚为高电平。
    let bit = gpioa.idr.read().bits() & mask;
    // 判断该位的值是否为 0
    bit == 0
}

/// 是否为高电平
fn is_high(gpioa: &GPIOA) -> bool {
    gpioa.idr.read().idr1().is_high()
}

/// 是否为高电平
fn is_pin_high(gpioa: &GPIOA, pin: u16) -> bool {
    // 获取 PA1 引脚对应的 IDR 寄存器中的位的掩码
    let mask = 1 << pin;
    // 读取 IDR 寄存器中对应的位的值
    // 如果该位的值为 0，那么说明该引脚为低电平；如果该位的值为 1，那么说明该引脚为高电平。
    let bit = gpioa.idr.read().bits() & mask;
    // 判断该位的值是否为 1
    bit == 1
}

/// 设置引脚为高电平
fn set_high(gpioa: &GPIOA, pin: u16) {
    // 设置 PA1 引脚为高电平
    // 将 GPIOA 的 LED_PIN 引脚设置为高电平（1）；
    // 使用左移运算符（<<）将数字 1 左移 LED_PIN 位，得到一个二进制数，其中只有第 LED_PIN 位为 1，其余位为 0。
    // 例如，如果 LED_PIN 为 5，则得到的二进制数为 0b00100000。
    gpioa.bsrr.write(|w| unsafe { w.bits(1 << pin) });
    // gpioa.bsrr.write(|w| w.bs1().set_bit()); // 高电平
    // gpioa.bsrr.write(|w| w.bs1().clear_bit()); // 低电平
}

/// 设置引脚为低电平
fn set_low(gpioa: &GPIOA, pin: u16) {
    // 由于 BSRR 寄存器的高 16 位用来清除 GPIOA 的输出状态，
    // 所以这样就相当于将第 LED_PIN 引脚清除为低电平（0），而不影响其他引脚的状态。
    gpioa.bsrr.write(|w| unsafe { w.bits(1 << (16 + pin)) });
    // gpioa.bsrr.write(|w| w.br1().set_bit()); // 低电平
    // gpioa.bsrr.write(|w| w.br1().reset()); // 低电平
    // gpioa.bsrr.write(|w| w.br1().clear_bit()); // 高电平
}

/// 翻转引脚电平
fn toggle(gpioa: &GPIOA, pin: u16) {
    if is_pin_low(gpioa, pin) {
        set_high(gpioa, pin)
    } else {
        set_low(gpioa, pin)
    }
}
