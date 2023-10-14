//!对射式红外传感器触发中断计数
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, syst::delay_ms};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32f1::stm32f103::{interrupt, EXTI};
use stm32f1::stm32f103::{CorePeripherals, Interrupt, Peripherals};

static G_EXTI: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let gpiob = &dp.GPIOB;
    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    let exti = dp.EXTI;
    let afio = &dp.AFIO;
    let mut nvic = cp.NVIC;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    // 使能 APB2 GPIOB 和 AFIO 时钟
    rcc.apb2enr
        .modify(|_, w| w.iopben().enabled().afioen().enabled());

    // 光敏传感器
    // 配置引脚为上拉输入模式
    gpiob
        .crh
        .modify(|_, w| w.mode14().input().cnf14().alt_push_pull());
    // 设置引脚为高电平
    gpiob.bsrr.write(|w| w.bs14().set_bit());

    // 选择 EXTI 的触发源
    // 这是因为 EXTI14 的触发源可以选择从 PA14 到 PG14 的任意一个引脚，而 AFIO 的 EXTICR4 寄存器的 EXTI14 位域用来配置这个选择。
    // EXTI14 位域的值为 0 表示选择 PA14，为 1 表示选择 PB14，以此类推，直到 6 表示选择 PG14。
    // 所以这里设置的是 1，就是选择了 PB14 引脚作为 EXTI14 的触发源。
    afio.exticr4.modify(|_, w| unsafe { w.exti14().bits(0b01) });
    // afio.exticr4
    //     .modify(|_, w| w.exti14().variant(1));

    // 从 EXTI 引脚启用外部中断
    // exti.imr.modify(|_, w| w.mr14().masked().mr14().set_bit());
    exti.imr.modify(|_, w| w.mr14().set_bit());

    // Event mask register
    // exti.emr.modify(|_, w| w.mr14().set_bit());

    // 在 EXTI 上升沿生成中断
    exti.rtsr.modify(|_, w| w.tr14().enabled());
    // 禁用 EXTI 下降沿生成中断
    // exti.ftsr
    // .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 14)) });
    exti.ftsr.modify(|_, w| w.tr14().disabled());

    cortex_m::interrupt::free(|cs| G_EXTI.borrow(cs).replace(Some(exti)));

    // 配置 NVIC 以使能 EXTI15_10 中断
    unsafe {
        NVIC::unmask(Interrupt::EXTI15_10);
        nvic.set_priority(Interrupt::EXTI15_10, 0x1);
    }

    println!("loop...");
    loop {
        let count = get_sensor_count();
        println!("count: {:#?}", count);
        delay_ms(&mut syst, 1000);
    }
}

// 计数器
static mut COUNT: u32 = 0;

#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let mut binding = G_EXTI.borrow(cs).borrow_mut();
        let exti = match binding.as_mut() {
            Some(exti) => exti,
            None => return,
        };

        // 获取中断标识, 非中断标识退出
        if !exti.pr.read().pr14().is_pending() {
            return;
        }

        unsafe {
            COUNT += 1;
        }

        // 清除中断标志
        exti.pr.modify(|_, w| w.pr14().set_bit());
    });
}

/// 获取传感器计数
fn get_sensor_count() -> u32 {
    unsafe { COUNT }
}
