#![no_std]
#![no_main]

use riscv_rt::entry;
use panic_halt as _;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{Rectangle, PrimitiveStyle};
use longan_nano::hal::{pac, prelude::*};
use longan_nano::{lcd_pins,lcd};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
    .RCU
    .configure()
    .ext_hf_clock(8.mhz())
    .sysclk(108.mhz())
    .freeze();

    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
    .into_styled(PrimitiveStyle::with_fill(Rgb565::MAGENTA))
    .draw(&mut lcd)
    .unwrap();
    loop 
    {

    }
}

