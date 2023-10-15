//! RTC 告警中断
//! 这是一个失败的案例
#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::cell::RefCell;

use stm32f1_core::hardware::{acr::set_flash, cfgr::set_clock, syst::delay_ms};

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::{interrupt::Mutex, peripheral::NVIC};
use cortex_m_rt::entry;
use stm32f1::stm32f103::{interrupt, CorePeripherals, Interrupt, Peripherals, EXTI, RTC};

static G_RTC: Mutex<RefCell<Option<RTC>>> = Mutex::new(RefCell::new(None));
static G_EXTI: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let rcc = &dp.RCC;
    let flash = &dp.FLASH;
    let mut syst = cp.SYST;

    let pwr = &dp.PWR;
    let rtc = dp.RTC;
    let mut nvic = cp.NVIC;

    // 设置 Flash
    set_flash(flash);

    // 设置时钟
    set_clock(rcc);

    // 使能 APB2 时钟
    rcc.apb2enr.modify(|_, w| {
        w.afioen().enabled() // AFIO
    });
    // 使能 PWR 和 BKP 时钟
    rcc.apb1enr
        .modify(|_, w| w.pwren().set_bit().bkpen().set_bit());

    println!("PWR ...");
    // 解除 RTC 寄存器写保护
    pwr.cr.modify(|_, w| w.dbp().set_bit());

    // 使能 LSE 时钟
    rcc.bdcr.modify(|_, w| w.lseon().set_bit());

    // 等待 LSE 时钟就绪
    while rcc.bdcr.read().lserdy().bit_is_clear() {}

    // 选择 LSE 作为 RTC 时钟源
    rcc.bdcr.modify(|_, w| w.rtcsel().lse());

    // 使能 RTC 时钟
    rcc.bdcr.modify(|_, w| w.rtcen().set_bit());

    // 等待 RTC 寄存器同步
    while rtc.crl.read().rtoff().bit_is_clear() {}

    // 进入 RTC 配置模式
    rtc.crl.modify(|_, w| w.cnf().set_bit());

    // 设置 RTC 分频器
    // 设置 RTC 预分频器的低 16 位为 0x7FFF，即 32767
    // RTC 的时钟周期为 1 秒，即 RTCCLK/RTC_PR = (32.768 KHz)/(32767+1)
    rtc.prll.write(|w| w.prll().bits(0x7FFF));

    // 设置 RTC 预分频器的高 16 位为 0x7F，即 127
    rtc.prlh.write(|w| w.prlh().bits(0x7F));

    // 设置 RTC 子秒寄存器为 0
    rtc.cnth.write(|w| w.cnth().bits(0));
    rtc.cntl.write(|w| w.cntl().bits(0));

    // 设置闹钟值，单位秒
    rtc.alrl.write(|w| w.alrl().bits(1));

    // 退出 RTC 配置模式
    rtc.crl.modify(|_, w| w.cnf().clear_bit());

    // 等待 RTC 寄存器同步
    while rtc.crl.read().rtoff().bit_is_clear() {}

    // 使能 RTC 闹钟中断
    rtc.crh.modify(|_, w| w.alrie().set_bit());

    cortex_m::interrupt::free(|cs| G_RTC.borrow(cs).replace(Some(rtc)));

    // 配置 NVIC 以使能 RTCALARM 中断
    unsafe {
        NVIC::unmask(Interrupt::RTCALARM);
        nvic.set_priority(Interrupt::RTCALARM, 0x1);
    }

    // 设置EXTI
    let exti = dp.EXTI;
    // 下降触发器选择寄存器
    exti.ftsr.write(|w| w.tr17().set_bit());
    // 中断屏蔽寄存器
    exti.imr.write(|w| w.mr17().set_bit());
    cortex_m::interrupt::free(|cs| G_EXTI.borrow(cs).replace(Some(exti)));

    println!("loop...");
    loop {
        let count = get_count();
        println!("count: {:#?}", count);
        delay_ms(&mut syst, 500);
    }
}

// 计数器
static mut COUNT: u32 = 0;

#[interrupt]
fn RTCALARM() {
    static mut RTC: Option<RTC> = None;
    static mut EXTI: Option<EXTI> = None;

    println!("RTCALARM...");

    let rtc = RTC.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_RTC.borrow(cs).replace(None).unwrap())
    });
    let exti = EXTI.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_EXTI.borrow(cs).replace(None).unwrap())
    });

    // 挂起寄存器
    exti.pr.write(|w| w.pr17().set_bit());

    // Wait for the APB1 interface to be ready
    while !rtc.crl.read().rsf().bit() {}

    let current_time = rtc.cnth.read().bits() << 16 | rtc.cntl.read().bits();
    println!("current_time: {:?}", current_time);

    unsafe {
        COUNT += 1;
    }

    // 再次设置告警时间
    rtc.alrl.write(|w| w.alrl().bits(1));

    // 清除 RTC 闹钟中断标志
    rtc.crl.modify(|_, w| w.alrf().clear_bit());
}

/// 获取计数
fn get_count() -> u32 {
    unsafe { COUNT }
}
