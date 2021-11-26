#![no_std]
#![no_main]


use core::convert::{Infallible};
use core::pin;

use longan_nano::hal::gpio::{Floating, Input, gpioa, gpioc};
use riscv_rt::entry;
use panic_halt as _;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{Rectangle, PrimitiveStyle};
use longan_nano::hal::{pac, prelude::*, gpio::gpioc::*, gpio::gpioa::*};
use longan_nano::{lcd_pins,lcd};
use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
use longan_nano::hal::delay::McycleDelay;

pub struct DataPins
{
    pin0 : PA2<Input<Floating>>,
    pin1 : PA1<Input<Floating>>,
    pin2 : PA0<Input<Floating>>,
    pin3 : PC13<Input<Floating>>,
    pin4 : PC14<Input<Floating>>,
    pin5 : PC15<Input<Floating>>,
    pin6 : PA3<Input<Floating>>,
    pin7 : PA4<Input<Floating>>

}


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
    let gpioc = dp.GPIOC.split(&mut rcu);
    //let mut dataPins= create_datapins(gpioc, gpioa);
    let mut dataPins = DataPins{
        pin0 : gpioa.pa2,
        pin1 : gpioa.pa1,
        pin2 : gpioa.pa0,
        pin3 : gpioc.pc13,
        pin4 : gpioc.pc14,
        pin5 : gpioc.pc15,
        pin6 : gpioa.pa3,
        pin7 : gpioa.pa4,
    };
    let mut clkPin = gpiob.pb9.into_push_pull_output();
    let mut delay = McycleDelay::new(&rcu.clocks);
    let mut rgb_bytes = [[0u8 ; 160]; 70];
    loop 
    {
        clkPin.set_high();
        clkPin.set_low();
        
        let datapin7Status = match dataPins.pin5.is_high()
        {
            Ok(f) =>  f,
            Err(e) => false,
        };
        let datapin6Status = match dataPins.pin4.is_high()
        {
            Ok(f) =>  f,
            Err(e) => false,
        };
        let mut val = read_pixel(&dataPins, &rgb_bytes, (1,1));
        if(!datapin7Status)
        {
            Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut lcd)
            .unwrap();
        }
        else 
        {
            Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(&mut lcd)
            .unwrap();
        }
        delay.delay_ms(20000);

    }
}

fn read_pixel(pins : &DataPins, array: &[[u8 ; 160]; 70], (row,column): (u8, u8) ) -> u8
{
    let mut rgb_value: u8 = 0; 
    rgb_value &= read_pin(pins.pin7.is_high()) as u8;
    rgb_value = rgb_value << 1;
    rgb_value &= read_pin(pins.pin6.is_high()) as u8;
    rgb_value = rgb_value << 1;
    rgb_value &= read_pin(pins.pin5.is_high()) as u8;
    rgb_value = rgb_value << 1;
    rgb_value &= read_pin(pins.pin4.is_high()) as u8;
    rgb_value = rgb_value << 1;
    rgb_value &= read_pin(pins.pin3.is_high()) as u8;
    rgb_value = rgb_value << 1;
    rgb_value &= read_pin(pins.pin2.is_high()) as u8;
    rgb_value = rgb_value << 1;
    rgb_value &= read_pin(pins.pin1.is_high()) as u8;
    rgb_value = rgb_value << 1;
    rgb_value &= read_pin(pins.pin0.is_high()) as u8;
    rgb_value
}

fn read_pin(is_pin_high : Result<bool, Infallible>) -> bool
{
    let mut retVal = match is_pin_high {
        Ok(f) =>  f,
        Err(e) => false,
    };
    retVal
}


