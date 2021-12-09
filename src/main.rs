#![no_std]
#![no_main]

use longan_nano::hal::eclic::{EclicExt, Level, LevelPriorityBits, Priority, TriggerType};
use longan_nano::hal::gpio::gpiob::PB9;
use longan_nano::hal::gpio::{Floating, Input, OpenDrain, Output, PullDown, PushPull, State};
use longan_nano::hal::timer::{Event, Timer};
use longan_nano::lcd::Lcd;
use riscv_rt::entry;
use panic_halt as _;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::{ Rgb565};
use longan_nano::hal::{pac, prelude::*, gpio::gpioc::*, gpio::gpioa::*};
use longan_nano::{lcd_pins,lcd};
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};
use longan_nano::hal::delay::McycleDelay;

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

const NUMBER_OF_ROWS: u8 = 160;
const NUMBER_OF_COLUMNS : u8 = 80;
const NUMBER_OF_PIXELS : u16 = 12800;
static mut CLK_PIN : Option<PA9<Output<PushPull>>> = None;
static mut CAMERA_CLOCK_STATE : bool = false;
static mut TIMER : Option<Timer<longan_nano::hal::pac::TIMER1>> = None;

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
    
    let gpioc = dp.GPIOC.split(&mut rcu);


    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);
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
    let mut clk_pin = gpioa.pa9.into_push_pull_output_with_state(State::Low);
    let mut delay = McycleDelay::new(&rcu.clocks);

    let href = gpioa.pa12;
    let read_clk = gpiob.pb9;

    let mut rgb_bytes = [[0u8 ; NUMBER_OF_ROWS as usize]; NUMBER_OF_COLUMNS as usize];
    let mut counter_row: u16 = 0;
    let mut counter_column: u16 = 0;
    let mut test_bytes = [0u8; 255];
    let mut test_counter: usize = 0;
 
    
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
            // Clock rises   
            if CAMERA_CLOCK_STATE
            {
            }
            // Clock lowers
            else 
            {
            }
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
        TIMER.as_mut().unwrap().clear_update_interrupt_flag();
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
         

        
        
    }
    

}
