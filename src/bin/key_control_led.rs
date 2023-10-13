//!按键控制 LED
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use stm32f1_core::hardware::{
    acr::set_flash,
    cfgr::set_clock,
    gpio::{Gpioa, Gpiob},
    syst::delay_ms,
};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1::stm32f103::{CorePeripherals, Peripherals, SYST};

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
    // 定义 LED 连接的引脚号
    let leds: [Gpioa; 3] = [
        Gpioa::new(gpioa, 0),
        Gpioa::new(gpioa, 1),
        Gpioa::new(gpioa, 2),
    ];
    // 默认熄灯
    for led in &leds {
        led.set_high();
    }

    // KEY
    // 配置引脚为上拉输入模式
    // 输入模式中速度是没有用的, 无需配置
    gpiob
        .crl
        .modify(|_, w| w.mode1().input().cnf1().alt_push_pull());
    // 设置引脚为高电平
    gpiob.bsrr.write(|w| w.bs1().set_bit());
    let key_pin = Gpiob::new(gpiob, 1);

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

/// 获取按键的状态
fn get_key_status(key: &Gpiob, syst: &mut SYST) -> bool {
    if !key.is_low() {
        return false;
    }

    // 按键按下抖动
    delay_ms(syst, 20_u32);
    // 按着不动, 松手后跳出循环
    while key.is_low() {}
    // 按键松开抖动
    delay_ms(syst, 20_u32);
    true
}
