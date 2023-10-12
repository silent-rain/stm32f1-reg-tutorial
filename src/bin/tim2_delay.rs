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
    let tim2 = dp.TIM2;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    println!("配置时钟树");
    set_clock(rcc);

    // 使能定时器2时钟
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());

    // 封装定时器
    let mut delay = Delay { tim: tim2 };

    loop {
        for i in 0..10 {
            println!("i={:?}", i);
            // 延时一秒
            delay.delay_ms(1000);
        }
    }
}

struct Delay {
    tim: TIM2,
}

impl Delay {
    fn delay_ms(&mut self, ms: u16) {
        // 设置 TIM2 的预分频器和自动重装载寄存器
        self.tim.psc.write(|w| w.psc().bits(7999)); // 预分频器
        self.tim.arr.write(|w| w.arr().bits(1000)); // 自动重装载寄存器

        self.tim.cr1.modify(|_, w| w.cen().set_bit()); // 启动 TIM2

        while self.tim.sr.read().uif().bit_is_clear() {} // 等待更新事件标志

        self.tim.sr.modify(|_, w| w.uif().clear_bit()); // 清除更新事件标志
    }
}
