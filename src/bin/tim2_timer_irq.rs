//! TIM2 中断
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::{interrupt::Mutex, peripheral::NVIC};
use cortex_m_rt::entry;
use stm32f1::stm32f103::{interrupt, CorePeripherals, Interrupt, Peripherals, TIM2};

static G_TIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let tim2 = dp.TIM2;
    let mut nvic = cp.NVIC;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    // 使能 APB2 时钟
    rcc.apb2enr.modify(|_, w| {
        w.afioen().enabled() // AFIO
    });
    // 使能 APB1 时钟
    rcc.apb1enr.modify(|_, w| {
        w.tim2en().enabled() // TIM2
    });

    println!("tim ...");
    // 设置 TIM2 预分频器
    tim2.psc.write(|w| w.psc().bits(36_000 - 1));
    // 设置 TIM2 自动重装寄存器
    tim2.arr.write(|w| w.arr().bits(1000 - 1));

    // 使能 TIM2 更新中断
    tim2.dier.write(|w| w.uie().enabled());

    // 使能 TIM2
    tim2.cr1.modify(|_, w| w.cen().enabled());

    cortex_m::interrupt::free(|cs| G_TIM2.borrow(cs).replace(Some(tim2)));

    // 配置 NVIC 以使能中断
    unsafe {
        NVIC::unmask(Interrupt::TIM2);
        nvic.set_priority(Interrupt::TIM2, 0x1);
    }

    println!("loop...");
    loop {
        let count = get_count();
        println!("count: {:#?}", count);
    }
}

// 计数器
static mut COUNT: u32 = 0;

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        let mut binding = G_TIM2.borrow(cs).borrow_mut();
        let tim2 = match binding.as_mut() {
            Some(tim2) => tim2,
            None => return,
        };

        // 获取中断标识, 非中断标识退出
        if !tim2.sr.read().uif().bit_is_set() {
            return;
        }

        unsafe {
            COUNT += 1;
        }

        // 清除中断标志
        tim2.sr.modify(|_, w| w.uif().clear_bit());
    });
}

/// 获取计数
fn get_count() -> u32 {
    unsafe { COUNT }
}
