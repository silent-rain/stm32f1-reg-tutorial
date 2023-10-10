#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, syst::delay_ms};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1::stm32f103::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let gpioa = &dp.GPIOA;
    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    println!("配置时钟树");
    set_clock(rcc);

    // 配置为推挽输出模式
    println!("配置引脚");
    gpioa.crl.modify(|_, w| {
        w.mode1()
            // 设置输出速度为 50MHz
            // .bits(0b11)
            .output50()
            // 配置为推挽输出模式
            .cnf1()
            .push_pull()
    });

    // 如果LED灯是阳极接电源，阴极接IO口，那么当IO口输出低电平时，LED灯会导通，从而点亮；
    // 当IO口输出高电平时，LED灯会截止，从而熄灭。

    // 点亮 LED
    println!("点亮 LED");
    gpioa.brr.write(|w| w.br1().reset());

    // 延迟1s
    delay_ms(&mut syst, 1000);

    // 熄灭 LED
    println!("熄灭 LED");
    gpioa.bsrr.write(|w| w.bs1().set_bit());
    loop {}
}
