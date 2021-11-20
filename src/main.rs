#![no_std]
#![no_main]


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
    };
    let mut clkPin = gpiob.pb9.into_push_pull_output();
    let mut clk = false;
    let mut delay = McycleDelay::new(&rcu.clocks);

    loop 
    {
        if(!clk)
        {
            clk = true;
            clkPin.set_high();
        }
        else {
            clk = false;
            clkPin.set_low();
        }

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
        let value = evaluate_data(datapin7Status, datapin6Status);

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
  /*  
        match value {
            0 => Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut lcd)
            .unwrap(),
            1 => Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::MAGENTA))
            .draw(&mut lcd)
            .unwrap(),
            2 => Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
            .draw(&mut lcd)
            .unwrap(),
            3 => Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(&mut lcd)
            .unwrap(),
            _ => Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
            .draw(&mut lcd)
            .unwrap(),
        }; 
        */

        delay.delay_ms(500);

    }
}

fn evaluate_data(pin1: bool, pin2: bool) -> u8
{
    if(!pin1 && !pin2)
    {
        return 0;
    }
    else if(pin1 && !pin2)
    {
        return 1;
    }
    else if(!pin1 && pin2)
    {
        return 2;
    }
    else {
        return 3;
    }
}   

pub struct DataPins
{
    pin0 : PA2<Input<Floating>>,
    pin1 : PA1<Input<Floating>>,
    pin2 : PA0<Input<Floating>>,
    pin3 : PC13<Input<Floating>>,
    pin4 : PC14<Input<Floating>>,
    pin5 : PC15<Input<Floating>>,

}

