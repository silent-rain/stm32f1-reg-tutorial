//! 通用IO

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
