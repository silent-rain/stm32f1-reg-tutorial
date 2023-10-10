#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use embedded_hal::digital::{
    ErrorKind, ErrorType, OutputPin, PinState, StatefulOutputPin, ToggleableOutputPin,
};
use stm32f1::stm32f103::{Peripherals, GPIOC};

// 定义 LED 连接的引脚号（PC13）
const LED_PIN: u16 = 13;

#[entry]
fn main() -> ! {
    // 从外围设备访问机箱访问设备特定的外围设备
    let peripherals = Peripherals::take().unwrap();
    let gpio_c = &peripherals.GPIOC;

    // 配置 PC13 引脚为推挽输出模式
    gpio_c
        .crh
        .modify(|_, w| w.mode13().output().cnf13().push_pull());

    // 获取 PC13 引脚的输出控制对象
    let mut led = Pin::new(gpio_c, LED_PIN);

    loop {
        // 翻转 LED 的状态
        led.toggle().unwrap();
        // 延时大约一秒钟
        delay(8_000_000);
    }
}

// 定义一个简单的输出引脚结构体，实现 OutputPin 和 ToggleableOutputPin 特征
struct Pin<'a> {
    gpio: &'a GPIOC,
    pin: u16,
}

impl<'a> Pin<'a> {
    fn new(gpio: &'a GPIOC, pin: u16) -> Self {
        Self { gpio, pin }
    }
}

impl<'a> ErrorType for Pin<'a> {
    type Error = ErrorKind;
}
impl<'a> StatefulOutputPin for &mut Pin<'a> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl<'a> OutputPin for Pin<'a> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.gpio
            .bsrr
            .write(|w| unsafe { w.bits(1 << (16 + self.pin)) });
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.gpio.bsrr.write(|w| unsafe { w.bits(1 << self.pin) });
        Ok(())
    }

    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        let _ = match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        };
        Ok(())
    }
}

impl<'a> ToggleableOutputPin for Pin<'a> {
    fn toggle(&mut self) -> Result<(), Self::Error> {
        if self.is_set_low().unwrap() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

// 定义一个简单的延时函数，使用忙等待的方式
fn delay(cycles: u32) {
    for _ in 0..cycles {
        cortex_m::asm::nop();
    }
}
