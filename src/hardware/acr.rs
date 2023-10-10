//!Flash access control register

use stm32f1::stm32f103::FLASH;

/// 设置 Flash
pub fn set_flash(flash: &FLASH) {
    // 设置 Flash 访问延迟。
    // 因为我们将要设置系统时钟为 72MHz，所以需要增加 Flash 访问延迟。
    // 这里设置了两个等待周期。
    flash.acr.modify(|_, w| {
        w.latency()
            // .bits(0b010)
            .ws2()
    });
}
