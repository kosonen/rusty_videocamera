# rusty_videocamera

Goal of the project was to stream video from OV7670 camera module to 160x80 lcd display of longan nano. Software side was supposed to write in Rust using longan nano BSP and 
embbeded graphics libraries. As we learned more about our hardware we find out that our solution would not work because OV7670 need atleast 10 MHz external clock and we did not 
find way to solve this problem with longan nano which is also pretty slow.

## Harware and wiring

OV7670 needs external clock so we used pin PA9 to produce clock from longan nano to camera. PCLK pin is wired to PB9 pin in longan nano. In falling edge of this pin output pins
changes data to next byte. HREF pin is wired to PA12 pin in longan nano. HREF is up when camera is sending bytes from row and goes down when changing row and up when first byte 
from next row is available. Actual data is sended one byte per pclk cycle. Camera has output pins D0-D7 where D0 is lsb and D7 msb. These are wired D0 - PA2, D1 - PA1, D2 - PA0,
D3 - PC13, D4 - PC14, D5 - PC15, D6 - PA3, D7 - PA4.

## Problem

OV7670 external clock needs to oscillate in 10 - 48 MHz frequency and duty cycle need to be between 45 - 55 %. Our idea was to set timer to interrupt 24 MHz frequency and change 
the state of clock pin. This way camera would have 12 MHz frequency clock signal. The problem is that now we would have about 10 clock cycle to handle one byte because maximum 
frequency for longan nano processor clock is 108 MHz. We analyzed assembler dump of our code (without optimization) and the amount of instructions in our byte handling was about
100. Even if execution of one instruction would take only 1 clock cycle we still dont have a chance to be able to handle one byte in decent time. Also longan nano has very limited
memory so we can only read data from pixles we want to draw. For this we needed counters too keep track in which pixels we are. This increases a lot the amount of instructions.

## How it should work

Our idea was to read data one byte at a time and save those to array. Due to lack of memory we can only read pixels we want to show in lcd screen. Thats why we needed a lot of
counters to keep track where we are and when row changes. When all needed bytes are read we draw image by using embbedded graphics library which can draw picture from array of
RGB565 or RGB555 bytes.

## Solutions

Solution for this is to use chip which has better performance. Increasing clock speed gives us more clock cycles to handle bytes. Other solution could be increase memory of
chip. This way we could just save all the bytes camera sends and draw image based on these.
