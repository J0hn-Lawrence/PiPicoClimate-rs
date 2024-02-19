//! This example shows how to use SPI (Serial Peripheral Interface) in the RP2040 chip.
//!
//! Example written for a display using the ST7789 chip. Possibly the Waveshare Pico-ResTouch
//! (https://www.waveshare.com/wiki/Pico-ResTouch-LCD-2.8)

#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt::*;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::Delay;
use embedded_graphics::image::{Image, ImageRawLE};
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
//use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Text;
use mipidsi::Builder;
use {defmt_rtt as _, panic_probe as _};
use display_interface_spi::SPIInterface;





#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Hello World!");



    const BUTTON_A: u8 = 5;
    const BUTTON_B: u8 = 6;
    const BUTTON_X: u8 = 16;
    const BUTTON_Y: u8 = 24;

    const LED_R: u8 = 17;
    const LED_G: u8 = 27;
    const LED_B: u8 = 22;

    const DISPLAY_FREQ: u32 = 64_000_000;
    


    let rst = p.PIN_1;  // 10

    let bl = p.PIN_20;
    let mosi = p.PIN_19;
    let clk = p.PIN_18;
    let display_cs = p.PIN_17;
    let dcx = p.PIN_16;

    let _miso = p.PIN_0; // not used


    // create SPI
    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    let spi: Spi<'_, _, Blocking> = Spi::new_blocking(p.SPI0, clk, mosi, _miso, display_config.clone());
    //let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    //let display_spi: SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, embassy_rp::peripherals::SPI0, Blocking>, Output<'_, embassy_rp::peripherals::PIN_17>> = SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);

    let dcx = Output::new(dcx, Level::Low);
    let rst = Output::new(rst, Level::Low);
    // dcx: 0 = command, 1 = data

    // Enable LCD backlight
    let _bl = Output::new(bl, Level::High);

    // display interface abstraction from SPI and DC
    let di = SPIInterface::new(spi, dcx, Output::new(display_cs, Level::High));

    // create driver
    
    let mut display = Builder::st7789(di)
        // width and height are switched on purpose because of the orientation
        .with_display_size(240, 240)
        // this orientation applies for the Display HAT Mini by Pimoroni
        .with_orientation(mipidsi::Orientation::Portrait(false))
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut Delay, Some(rst))
        .unwrap();



    display.clear(Rgb565::BLACK).unwrap();

    let raw_image_data = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(34, 68));

    // Display the image
    ferris.draw(&mut display).unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
    Text::new(
        "Hello embedded_graphics \n + embassy + RP2040!",
        Point::new(20, 200),
        style,
    )
    .draw(&mut display)
    .unwrap();

    loop {
        
    }
}

