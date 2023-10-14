//! TIM2 定时器外部时钟中断
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use cortex_m::{interrupt::Mutex, peripheral::NVIC};
use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, syst::delay_ms};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1::stm32f103::{interrupt, CorePeripherals, Peripherals, TIM2};

static G_TIM2: Mutex<RefCell<Option<TIM2>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let gpiob = &dp.GPIOB;
    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut nvic = cp.NVIC;
    let tim2 = dp.TIM2;
    let mut syst = cp.SYST;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    // 使能 APB1 时钟
    rcc.apb1enr.modify(|_, w| {
        w.tim2en().enabled() // TIM2
    });
    // 使能 APB2 时钟
    rcc.apb2enr.modify(|_, w| {
        w.iopben().enabled() // GPIOB
    });

    // 对射式红外传感器
    // 配置引脚为上拉输入模式
    gpiob
        .crh
        .modify(|_, w| w.mode14().input().cnf14().alt_push_pull());
    // 设置引脚为高电平
    gpiob.bsrr.write(|w| w.bs14().set_bit());

    println!("TIM2 External...");
    // 设置 TIM2 预分频器
    tim2.psc.write(|w| w.psc().bits(0));
    // 设置 TIM2 自动重装载值
    tim2.arr.write(|w| w.arr().bits(1));

    // Configure the external clock mode 1 on TIM2
    tim2.smcr.modify(|_, w| {
        w
            // 外部时钟触发模式
            .sms()
            .encoder_mode_1()
            // 主/从模式
            .msm()
            .set_bit()
            // Enable external clock mode 2
            .ece()
            .enabled()
            // Set external trigger polarity to non-inverted
            .etp()
            .not_inverted()
            // External trigger prescaler
            .etps()
            .div1()
            // Set external trigger filter to 0x0F
            .etf()
            .no_filter()
            // 触发器选择, External Trigger input (ETRF)
            .ts()
            .ti1f_ed()
    });

    // // 设置 TIM2 为外部时钟模式 1
    // tim2.smcr.modify(|_, w| w.sms().encoder_mode_1());
    // // 选择 TI1 作为外部时钟源
    // tim2.smcr.modify(|_, w| w.ts().ti1f_ed());

    // 使能 TIM2 更新中断
    tim2.dier.write(|w| w.uie().enabled());

    // 配置定时器为从模式，使用 TIM2_ETR 引脚作为输入源，使用外部触发模式
    tim2.cr1.modify(|_, w| w.urs().set_bit().cen().set_bit());

    // 本质：在调用中断前，中断状态寄存器不能有标志位
    // 避免刚一上电就立刻进入中断，在 Time Init 的后面和中断的前面（手动清除中断标志位）
    tim2.sr.write(|w| w.cc1if().clear_bit());

    cortex_m::interrupt::free(|cs| G_TIM2.borrow(cs).replace(Some(tim2)));

    unsafe {
        NVIC::unmask(interrupt::TIM2);
        nvic.set_priority(interrupt::TIM2, 1);
    }

    println!("loop...");
    loop {
        let count = get_count();
        println!("count: {:#?}", count);
        delay_ms(&mut syst, 1000);
    }
}

// 计数器
static mut COUNT: u32 = 0;

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        println!("G_TIM2");
        let mut binding = G_TIM2.borrow(cs).borrow_mut();
        let tim2 = match binding.as_mut() {
            Some(tim2) => tim2,
            None => return,
        };

        // 获取中断标识, 非中断标识退出
        if !tim2.sr.read().uif().bit_is_set() {
            return;
        }
        println!("tim2");

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
