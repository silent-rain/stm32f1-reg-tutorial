//!TIM2 定时器延迟
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1::stm32f103::{CorePeripherals, Peripherals, TIM2};
use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock};

#[entry]
fn main() -> ! {
    // 从外围设备访问机箱访问设备特定的外围设备
    let dp = Peripherals::take().unwrap();
    // 从 cortex-m 机箱访问核心外围设备
    let cp = CorePeripherals::take().unwrap();

    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let tim2 = &dp.TIM2;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    println!("配置时钟树");
    set_clock(rcc);

    // 使能定时器2时钟
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());

    // 设置定时器2的重装值为72M/1000，即每毫秒计数一次
    tim2.arr.write(|w| w.arr().bits(65535));

    // 设置定时器2的预分频值为0，即不分频
    tim2.psc.write(|w| w.psc().bits(0));

    // 清除定时器2的更新事件标志
    tim2.sr.modify(|_, w| w.uif().clear_bit());

    // 启动定时器2
    tim2.cr1.modify(|_, w| w.cen().set_bit());

    loop {
        for i in 0..10 {
            println!("i={:?}", i);
            // 延时一秒
            delay_ms(tim2, 1000);
        }
    }
}

// 定义一个延时函数，参数为毫秒数
fn delay_ms(tim2: &TIM2, ms: u32) {
    // 循环ms次
    for _ in 0..ms {
        // 等待定时器2的更新事件标志置位，即一毫秒过去
        while tim2.sr.read().uif().bit_is_clear() {}
        // 清除定时器2的更新事件标志
        tim2.sr.modify(|_, w| w.uif().clear_bit());
    }
}
