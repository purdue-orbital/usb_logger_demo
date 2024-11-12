#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::pac::pwm::Pwm;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{i2c, pwm};
use embassy_time::Timer;
use embassy_usb_logger;
use {defmt_rtt as _, panic_probe as _};

mod iis2mdc;
mod bma530;
mod adxl314;
mod bmp585;

bind_interrupts!(struct Irqs {
	USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
	embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_rp::init(Default::default());

	// USB
	let driver = Driver::new(p.USB, Irqs);
	spawner.spawn(logger_task(driver)).unwrap();

	// I2c
	let mut config = i2c::Config::default();
	config.frequency = 100000;
	let sda = p.PIN_8;
	let scl = p.PIN_9;
	let mut i2c_bus = i2c::I2c::new_blocking(p.I2C0, scl, sda, config);

	// PWM
	let mut config = pwm::Config::default();
	config.compare_a = 0x8000;
	config.compare_b = 8;
	let mut pin0 = pwm::Pwm::new_output_a(p.PWM_SLICE0, p.PIN_0, config);

	bmp585::set_power_mode(&mut i2c_bus, bmp585::PowerMode::Normal);
	bmp585::set_fifo_press(&mut i2c_bus);
	bmp585::set_osr_press(&mut i2c_bus);
	Timer::after_millis(1000).await;

	loop {
		
		// let hrmmmmm: f32 = bmp585::get_pressure(&mut i2c_bus);
		// log::info!("id: {}", hrmmmmm);


		// adxl314 testing
		let acceleration = adxl314::read_acceleration(&mut i2c_bus);
		log::info!("acceleration: {}", acceleration);

		Timer::after_millis(500).await;
	}
}