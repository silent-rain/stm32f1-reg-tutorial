//! 按键中断控制 LED 灯
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;
use core::mem::MaybeUninit;

use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, syst::delay_ms};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32f1::stm32f103::{interrupt, EXTI, GPIOA};
use stm32f1::stm32f103::{CorePeripherals, Interrupt, Peripherals};

static G_KEY: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));
static mut G_LED: MaybeUninit<GPIOA> = MaybeUninit::uninit();

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let gpioa = dp.GPIOA;
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

    // 使能 APB2 时钟
    rcc.apb2enr.modify(|_, w| {
        w.iopaen()
            .enabled() // GPIOA
            .iopben()
            .enabled() // GPIOB
            .afioen()
            .enabled() // AFIO
    });

    // LED
    // 配置引脚为推挽输出模式
    gpioa
        .crl
        .modify(|_, w| w.mode1().output50().cnf1().push_pull());
    gpioa.bsrr.write(|w| w.bs1().set_bit()); // 高电平

    let led = unsafe { &mut *G_LED.as_mut_ptr() };
    *led = gpioa;

    // KEY
    // 配置引脚为上拉输入模式
    gpiob
        .crl
        .modify(|_, w| w.mode1().input().cnf1().alt_push_pull());
    // 设置引脚为高电平
    gpiob.bsrr.write(|w| w.bs1().set_bit());

    // 选择 EXTI 的触发源
    // 这是因为 EXTI14 的触发源可以选择从 PA14 到 PG14 的任意一个引脚，而 AFIO 的 EXTICR4 寄存器的 EXTI14 位域用来配置这个选择。
    // EXTI14 位域的值为 0 表示选择 PA14，为 1 表示选择 PB14，以此类推，直到 6 表示选择 PG14。
    // 所以这里设置的是 1，就是选择了 PB14 引脚作为 EXTI14 的触发源。
    afio.exticr1.modify(|_, w| unsafe { w.exti1().bits(0b01) });

    // 从 EXTI 引脚启用外部中断
    exti.imr.modify(|_, w| w.mr1().set_bit());

    // 启用 EXTI 上升沿生成中断
    exti.rtsr.modify(|_, w| w.tr1().enabled());
    // 启用 EXTI 下升沿生成中断
    exti.ftsr.modify(|_, w| w.tr1().enabled());

    cortex_m::interrupt::free(|cs| G_KEY.borrow(cs).replace(Some(exti)));

    // 配置 NVIC 以使能 EXTI1 中断
    unsafe {
        NVIC::unmask(Interrupt::EXTI1);
        nvic.set_priority(Interrupt::EXTI1, 0x1);
    }

    println!("loop...");
    loop {
        delay_ms(&mut syst, 1000);
    }
}

#[interrupt]
fn EXTI1() {
    cortex_m::interrupt::free(|cs| {
        let mut binding = G_KEY.borrow(cs).borrow_mut();
        let key = match binding.as_mut() {
            Some(key) => key,
            None => return,
        };

        // 获取中断标识, 非中断标识退出
        if !key.pr.read().pr1().is_pending() {
            return;
        }
        println!("key...");

        let led = unsafe { &mut *G_LED.as_mut_ptr() };

        if led.idr.read().idr1().is_low() {
            led.bsrr.write(|w| w.bs1().set_bit()); // 高电平
        } else {
            led.bsrr.write(|w| w.br1().set_bit()); // 低电平
        }

        // 清除中断标志
        key.pr.modify(|_, w| w.pr1().set_bit());
    });
}
