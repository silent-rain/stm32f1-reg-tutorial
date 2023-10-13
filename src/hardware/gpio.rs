//! 通用IO
//! ```rust
//! // into_alternate_open_drain(): 将PA0引脚配置为开漏输出的备用功能模式。
//! gpioa.crl.modify(|_, w| w.mode0().output50().cnf0().alt_open_drain());
//!
//! // into_alternate_push_pull(): 将PA0引脚配置为推挽输出的备用功能模式。
//! gpioa.crl.modify(|_, w| w.mode0().output50().cnf0().alt_push_pull());
//!
//! // into_analog(): 将PA0引脚配置为模拟输入模式。
//! // 将引脚配置为可以在不更改类型的情况下在输入和输出之间更改的引脚。它最初是一个浮动输入
//! gpioa.crl.modify(|_, w| w.mode0().input().cnf0().analog());
//! 
//! // 手动实现模拟输入模式
//! // Reset the configuration of PA0
//! gpioa.crl.modify(|r, w| unsafe { w.bits(r.bits() & !(0b1111)) });
//! // Set PA0 as analog input
//! gpioa.crl.modify(|r, w| unsafe { w.bits(r.bits() | 0b0000) });
//!
//! // into_dynamic(): 将PA0引脚配置为动态模式，可以在运行时更改引脚的功能和模式。
//!
//! // into_floating_input(): 将PA0引脚配置为浮空输入模式。
//! // gpioa.crl.modify(|_, w| w.mode0().input().cnf0().bits(0b01));
//! gpioa.crl.modify(|_, w| w.mode0().input().cnf0().open_drain());
//!
//! // into_open_drain_output(): 将PA0引脚配置为开漏输出模式。
//! gpioa.crl.modify(|_, w| w.mode0().output50().cnf0().open_drain());
//!
//! // into_open_drain_output_with_state(cr, initial_state): 将PA0引脚配置为带有初始状态的开漏输出模式。
//! gpioa.crl.modify(|_, w| w.mode0().output50().cnf0().open_drain());
//! // 根据 initial_state 设置初始状态
//! if initial_state {
//!     gpioa.bsrr.write(|w| w.bs0().set_bit());
//! } else {
//!     gpioa.brr.write(|w| w.br0().set_bit());
//! }
//!
//! // into_pull_down_input(): 将PA0引脚配置为下拉输入模式。
//! // gpioa.crl.modify(|_, w| w.mode0().input().cnf0().bits(0b10));
//! gpioa.crl.modify(|_, w| w.mode0().input().cnf0().alt_push_pull());
//! gpioa.brr.write(|w| w.br0().set_bit());
//!
//! // into_pull_up_input(cr): 将PA0引脚配置为上拉输入模式。
//! // gpioa.crl.modify(|_, w| w.mode0().input().cnf0().bits(0b10));
//! gpioa.crl.modify(|_, w| w.mode0().input().cnf0().alt_push_pull());
//! gpioa.bsrr.write(|w| w.bs0().set_bit());
//!
//! // into_push_pull_output(cr): 将PA0引脚配置为推挽输出模式。
//!  gpioa.crl.modify(|_, w| w.mode0().output50().cnf0().push_pull());
//!
//! // into_push_pull_output_with_state(cr, initial_state): 将PA0引脚配置为带有初始状态的推挽输出模式。
//! gpioa.crl.modify(|_, w| w.mode0().output50().cnf0().push_pull());
//! // 根据 initial_state 设置初始状态
//! if initial_state {
//!     gpioa.bsrr.write(|w| w.bs0().set_bit());
//! } else {
//!     gpioa.brr.write(|w| w.br0().set_bit());
//! }
//! ```

use stm32f1::stm32f103::{GPIOA, GPIOB};

pub struct Gpioa<'a> {
    gpio: &'a GPIOA,
    pin: u16,
}
impl<'a> Gpioa<'a> {
    pub fn new(gpioa: &'a GPIOA, pin: u16) -> Self {
        Gpioa { gpio: gpioa, pin }
    }
    /// 端口输出是否为低电平
    pub fn is_set_low(&self) -> bool {
        self.gpio.odr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输出是否为高电平
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// 端口输入是否为低电平
    pub fn is_low(&self) -> bool {
        self.gpio.idr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输入是否为高电平
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// 设置引脚为高电平
    pub fn set_high(&self) {
        self.gpio.bsrr.write(|w| unsafe { w.bits(1 << self.pin) });
    }

    /// 设置引脚为低电平
    pub fn set_low(&self) {
        self.gpio
            .bsrr
            .write(|w| unsafe { w.bits(1 << (16 + self.pin)) });
    }

    /// 翻转引脚电平
    pub fn toggle(&self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

pub struct Gpiob<'a> {
    gpio: &'a GPIOB,
    pin: u16,
}
impl<'a> Gpiob<'a> {
    pub fn new(gpiob: &'a GPIOB, pin: u16) -> Self {
        Gpiob { gpio: gpiob, pin }
    }
    /// 端口输出是否为低电平
    pub fn is_set_low(&self) -> bool {
        self.gpio.odr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输出是否为高电平
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// 端口输入是否为低电平
    pub fn is_low(&self) -> bool {
        self.gpio.idr.read().bits() & (1 << self.pin) == 0
    }

    /// 端口输入是否为高电平
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// 设置引脚为高电平
    pub fn set_high(&self) {
        self.gpio.bsrr.write(|w| unsafe { w.bits(1 << self.pin) });
    }

    /// 设置引脚为低电平
    pub fn set_low(&self) {
        self.gpio
            .bsrr
            .write(|w| unsafe { w.bits(1 << (16 + self.pin)) });
    }

    /// 翻转引脚电平
    pub fn toggle(&self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}
