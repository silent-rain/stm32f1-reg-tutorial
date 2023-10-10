//!Clock configuration register

use stm32f1::stm32f103::RCC;

/// 设置时钟
pub fn set_clock(rcc: &RCC) {
    // 启用高速外部时钟（HSE）
    rcc.cr.modify(|_, w| w.hseon().set_bit());
    // 等待 HSERDY 位被设置，表示 HSE 已经稳定。
    while rcc.cr.read().hserdy().bit_is_clear() {}

    // 配置时钟树
    rcc.cfgr.modify(|_, w| {
        // 设置 AHB 总线时钟分频系数为 1（不分频）
        // 使 AHB 总线时钟频率等于系统时钟频率
        w.hpre().div1();
        // APB1 总线时钟分频系数为 2（36MHz）
        w.ppre1().div2();
        // APB2 总线时钟分频系数为 1（72MHz）
        w.ppre2().div1();
        // 设置 PLL 时钟源为 HSE，并将 HSE 时钟分频后作为 PLL 输入
        w.pllsrc().hse_div_prediv();
        // 将 HSE 预分频器的分频系数设置为 1。
        // HSE 的频率将除以 1 作为 PLL 的输入时钟频率。
        // HSE 的默认频率通常是 8MHz
        w.pllxtpre().div1();
        // 设置 PLL 倍频系数为 9，使 PLL 的输出频率为输入时钟频率的 9 倍
        // PLL 的输出时钟频率: 8MHz / 1 * 9 = 72MHz。
        w.pllmul().mul9();
        // 将时钟源选择为PLL
        w.sw().pll();
        // 设置了 USB 时钟分频系数为 1.5，使得 USB 时钟为 48MHz
        w.usbpre().div1_5()
    });

    // 启用 PLL（Phase-Locked Loop，锁相环）
    rcc.cr.modify(|_, w| w.pllon().set_bit());
    // 等待 PLL 稳定
    while rcc.cr.read().pllrdy().bit_is_clear() {}

    // 启用 APB2 的时钟
    rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
}
