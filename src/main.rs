#![no_std]
#![no_main]

use embedded_graphics::image::{ImageRaw, Image};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::{ Rgb565};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use longan_nano::hal::eclic::{EclicExt, Level, LevelPriorityBits, Priority, TriggerType};
use longan_nano::hal::gpio::{Input, Output, PullDown, PushPull, State};
use longan_nano::hal::timer::{Event, Timer};
use longan_nano::hal::{pac, prelude::*, gpio::gpioc::*, gpio::gpioa::*};
use longan_nano::{lcd_pins,lcd};
use riscv_rt::entry;
use panic_halt as _;



pub struct DataPins
{
    pin0 : PA2<Input<PullDown>>,
    pin1 : PA1<Input<PullDown>>,
    pin2 : PA0<Input<PullDown>>,
    pin3 : PC13<Input<PullDown>>,
    pin4 : PC14<Input<PullDown>>,
    pin5 : PC15<Input<PullDown>>,
    pin6 : PA3<Input<PullDown>>,
    pin7 : PA4<Input<PullDown>>

}

const NUMBER_OF_COLUMNS : u16 = 160;
const NUMBER_OF_ROWS : u16 = 80;
static mut CLK_PIN : Option<PA9<Output<PushPull>>> = None;
static mut CAMERA_CLOCK_STATE : bool = false;
static mut TIMER : Option<Timer<longan_nano::hal::pac::TIMER1>> = None;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
    .RCU
    .configure()
    .ext_hf_clock(30.mhz())
    .sysclk(108.mhz())
    .freeze();
    
    let mut afio = dp.AFIO.constrain(&mut rcu);
    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);    
    let gpioc = dp.GPIOC.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    lcd.clear(Rgb565::BLACK).unwrap();
     let data_pins = DataPins{
        pin0 : gpioa.pa2.into_pull_down_input(),
        pin1 : gpioa.pa1.into_pull_down_input(),
        pin2 : gpioa.pa0.into_pull_down_input(),
        pin3 : gpioc.pc13.into_pull_down_input(),
        pin4 : gpioc.pc14.into_pull_down_input(),
        pin5 : gpioc.pc15.into_pull_down_input(),
        pin6 : gpioa.pa3.into_pull_down_input(),
        pin7 : gpioa.pa4.into_pull_down_input(),
    };
    let clk_pin = gpioa.pa9.into_push_pull_output_with_state(State::Low);

    let href = gpioa.pa12;
    let pclk = gpiob.pb9;
    let mut byte_handled = false;

    let mut counter_row: u16 = 0;
    let mut counter_column: u16 = 0;
    let bytes = [0u8; 25600];
    let mut test_counter: usize = 0;
    let mut href_was_down = false;
  
    longan_nano::hal::pac::ECLIC::reset();
    longan_nano::hal::pac::ECLIC::set_level_priority_bits(LevelPriorityBits::L3P1);
    longan_nano::hal::pac::ECLIC::set_threshold_level(Level::L0);
    longan_nano::hal::pac::ECLIC::setup(pac::Interrupt::TIMER1, TriggerType::Level, Level::L1, Priority::P1);
     
    let mut timer = Timer::timer1(dp.TIMER1, 24.mhz(), &mut rcu);  
    timer.listen(Event::Update);

     unsafe{
        TIMER = Some(timer);
        CLK_PIN = Some(clk_pin);
        longan_nano::hal::pac::ECLIC::unmask(pac::Interrupt::TIMER1);
        riscv::interrupt::enable();
    }
    
    loop {
        unsafe
        {   
            riscv::asm::wfi();
        }
            if href.is_high().unwrap()
            {
                href_was_down = false;
                if counter_column <= NUMBER_OF_COLUMNS * 2 && !byte_handled && pclk.is_high().unwrap()
                {
                    handle_byte(&data_pins, bytes, test_counter);
                    byte_handled = true;
                    test_counter += 1;
                    counter_column += 1;
                }
                else if pclk.is_low().unwrap()
                {
                    byte_handled = false;
                }
            }
            else if !href_was_down
            {
                counter_row += 1;
                test_counter = 0;
                href_was_down = true;
            }
            if counter_row > NUMBER_OF_ROWS
            {
                let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&bytes, 80);
                Image::new(&raw_image, Point::new(0,0))
                    .draw(&mut lcd)
                    .unwrap();
            }

        
        
    }
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(unused_assignments)]
#[no_mangle]
fn TIMER1 ()
{
    unsafe{        
        if !CAMERA_CLOCK_STATE
        {
            CAMERA_CLOCK_STATE = true;
            CLK_PIN.as_mut().unwrap().set_high().unwrap();
        }
        else
        {
            CAMERA_CLOCK_STATE = false;
            CLK_PIN.as_mut().unwrap().set_low().unwrap();
        }
        TIMER.as_mut().unwrap().clear_update_interrupt_flag();           
    }
}

fn handle_byte(pins : &DataPins, mut rgb_array : [u8 ; 25600], i : usize)
{
    let mut byte : u8 = 0;
    byte |= pins.pin0.is_high().unwrap() as u8;
    byte  = byte << 1;
    byte |= pins.pin1.is_high().unwrap() as u8;
    byte  = byte << 1;
    byte |= pins.pin2.is_high().unwrap() as u8;
    byte  = byte << 1;
    byte |= pins.pin3.is_high().unwrap() as u8;
    byte  = byte << 1;
    byte |= pins.pin4.is_high().unwrap() as u8;
    byte  = byte << 1;
    byte |= pins.pin5.is_high().unwrap() as u8;
    byte  = byte << 1;
    byte |= pins.pin6.is_high().unwrap() as u8;
    byte  = byte << 1;
    byte |= pins.pin7.is_high().unwrap() as u8;
    byte  = byte << 1;
    rgb_array[i] = byte;
}
