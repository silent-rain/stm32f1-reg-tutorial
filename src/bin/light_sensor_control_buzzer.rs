//!光敏传感器控制蜂鸣器
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

    // 启用 APB2 GPIOB 的时钟
    rcc.apb2enr.modify(|_, w| w.iopben().set_bit());

    // 蜂鸣器
    // 配置引脚为推挽输出模式
    gpiob
        .crh
        .modify(|_, w| w.mode12().output50().cnf12().push_pull());
    let buzzer = Gpiob::new(gpiob, 12);

    // 光敏传感器
    // 配置引脚为上拉输入模式
    gpiob
        .crh
        .modify(|_, w| w.mode13().input().cnf13().alt_push_pull());
    // 设置引脚为高电平
    gpiob.bsrr.write(|w| w.bs13().set_bit());
    let light_sensor = Gpiob::new(gpiob, 13);

    println!("loop...");
    loop {
        println!("start...");
        if light_sensor.is_high() {
            buzzer.set_high();
        } else {
            buzzer.set_low();
        }
        // 检测间隔延时
        delay_ms(&mut syst, 200);
    }
}
