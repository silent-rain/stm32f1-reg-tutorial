//!按键控制 LED
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
use stm32f1::{
    stm32f103::{
        flash::KEYR, gpioa::idr::IDR_SPEC, CorePeripherals, Peripherals, GPIOA, GPIOB, SYST,
    },
    Reg,
};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let gpioa = &dp.GPIOA;
    let gpiob = &dp.GPIOB;
    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    // LED
    // 配置引脚为推挽输出模式
    gpioa.crl.modify(|_, w| {
        w.mode0().output50().cnf0().push_pull(); // LED 0
        w.mode1().output50().cnf1().push_pull(); // LED 1
        w.mode2().output50().cnf2().push_pull() // LED 2
    });
    // KEY
    // 配置引脚为上拉输入模式
    // 输入模式中速度是没有用的, 无需配置
    // 设置引脚为高电平
    gpiob.bsrr.write(|w| unsafe { w.bits(1 << 1) });
    gpiob
        .crl
        .modify(|_, w| w.mode1().input().cnf1().alt_push_pull());
    // gpiob.bsrr.write(|w| unsafe { w.bits(1 << 1) });

    // 设置 PB1 为输入模式 (00)
    // gpiob.crl.modify(|_, w| w.mode1().bits(0b00));

    // // 设置 PB1 为上拉模式 (10)
    // gpiob.crl.modify(|_, w| w.cnf1().bits(0b10));

    // 定义 LED 连接的引脚号（PA0、PA1）
    let leds: [Gpioa; 3] = [
        Gpioa::new(gpioa, 0),
        Gpioa::new(gpioa, 1),
        Gpioa::new(gpioa, 2),
    ];
    let key_pin = Gpiob::new(gpiob, 1);
    // 默认熄灯
    for led in &leds {
        led.set_high();
    }

    loop {
        println!("start...");
        if get_key_status(&key_pin, &mut syst) {
            for led in &leds {
                led.toggle();
            }
        }
        delay_ms(&mut syst, 500);
    }
}

struct Gpioa<'a> {
    gpio: &'a GPIOA,
    pin: u16,
}
impl<'a> Gpioa<'a> {
    fn new(gpioa: &'a GPIOA, pin: u16) -> Self {
        Gpioa { gpio: gpioa, pin }
    }
    /// 端口输出是否为低电平
    fn is_set_low(&self) -> bool {
        self.gpio.odr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输出是否为高电平
    fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// 端口输入是否为低电平
    fn is_low(&self) -> bool {
        self.gpio.idr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输入是否为高电平
    fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// 设置引脚为高电平
    fn set_high(&self) {
        self.gpio.bsrr.write(|w| unsafe { w.bits(1 << self.pin) });
    }

    /// 设置引脚为低电平
    fn set_low(&self) {
        self.gpio
            .bsrr
            .write(|w| unsafe { w.bits(1 << (16 + self.pin)) });
    }

    /// 翻转引脚电平
    fn toggle(&self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

struct Gpiob<'a> {
    gpio: &'a GPIOB,
    pin: u16,
}
impl<'a> Gpiob<'a> {
    fn new(gpiob: &'a GPIOB, pin: u16) -> Self {
        Gpiob { gpio: gpiob, pin }
    }
    /// 端口输出是否为低电平
    fn is_set_low(&self) -> bool {
        self.gpio.odr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输出是否为高电平
    fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// 端口输入是否为低电平
    fn is_low(&self) -> bool {
        self.gpio.idr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输入是否为高电平
    fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// 设置引脚为高电平
    fn set_high(&self) {
        self.gpio.bsrr.write(|w| unsafe { w.bits(1 << self.pin) });
    }

    /// 设置引脚为低电平
    fn set_low(&self) {
        self.gpio
            .bsrr
            .write(|w| unsafe { w.bits(1 << (16 + self.pin)) });
    }

    /// 翻转引脚电平
    fn toggle(&self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

/// 获取按键的状态
fn get_key_status(key: &Gpiob, syst: &mut SYST) -> bool {
    println!("key...");
    if !key.is_low() {
        return false;
    }
    println!("{:?}", key.is_low());
    println!("{:?}", key.is_high());
    println!("{:?}", key.is_set_low());
    println!("{:?}", key.is_set_high());
    println!("key2...");
    // 按键按下抖动
    delay_ms(syst, 20_u32);
    // 按着不动, 松手后跳出循环
    while key.is_low() {}
    // 按键松开抖动
    delay_ms(syst, 20_u32);
    println!("key3...");
    true
}
