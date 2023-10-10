//!SysTick: System Timer

use cortex_m::peripheral::{syst::SystClkSource, SYST};

/// 毫秒延时
/// 使用 SysTick 定时器来实现延时
pub fn delay_ms(syst: &mut SYST, ms: u32) {
    // 配置 SysTick
    // 将 SysTick 定时器设置为每 1ms（1000μs）回绕一次。因为系统时钟为 72MHz，所以每 1ms 有 72,000 个时钟周期。
    // 码设置了 SysTick 的重载值。当 SysTick 的计数值达到这个值时，它会自动清零并设置 "已经回绕" 标志

    // 内部时钟源，时钟频率为72MHz
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(72_000_000 / 1000);

    // 设置外部定时器为处理器时钟驱动
    // syst.set_clock_source(SystClkSource::External);
    // 外部时钟源，时钟频率为72MHz / 8 = 9MHz
    // syst.set_reload(9_000_000 / 1000);

    // 清除 SysTick 的当前计数值
    syst.clear_current();
    // 启用 SysTick 的计数器
    syst.enable_counter();
    // 启用 SysTick 的中断。我们并没有使用中断处理函数，所以这行代码实际上是不必要的。
    // syst.enable_interrupt();

    // 等待指定的毫秒数
    // 等待 SysTick 计数器回绕指定的次数
    for _ in 0..ms {
        while !syst.has_wrapped() {}
    }

    // 关闭 SysTick 的计数器
    syst.disable_counter();
}

/// 定义一个简单的延时函数，使用忙等待的方式
/// 延时大约一秒钟
pub fn delay(cycles: u32) {
    for _ in 0..cycles * 8_000_000 {
        cortex_m::asm::nop();
    }
}
