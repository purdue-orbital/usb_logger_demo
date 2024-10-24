use embedded_hal::i2c::I2c;
// const ADDR: u8 = 0x46; // alt of 0x47
const ADDR: u8 = 0x46; // alt of 0x47

//add 4.3.9 post power up procedure (pg 19)

pub fn get_ids(bus: &mut impl I2c) -> [u8; 2] {
	let mut buf = [0_u8; 1];

	let res = bus.write_read(ADDR, &[0x01], &mut buf); // check if issues

	if res.is_err() {
		log::error!("{:?}", res);
	}

	[buf[0], 0]
}

pub fn get_pressure(bus: &mut impl I2c) -> u32 {
	let mut buf = [0_u8; 3];

	let res = bus.write_read(ADDR, &[0x20], &mut buf); // get pressure
	if res.is_err() { //error handling
		log::error!("{:?}", res);
	}

	let output = u32::from_le_bytes([buf[0], buf[1], buf[2], 0]);
	output = output >> 6
}



pub fn get_temperature(bus: &mut impl I2c) -> f32 {
	let mut buf = [0_u8; 3];

	let res = bus.write_read(ADDR, &[0x1D], &mut buf); // get temperature
	if res.is_err() { //error handling
		log::error!("{:?}", res);
	}

	let output = u32::from_le_bytes([buf[0], buf[1], buf[2], 0]);
	log::info!("temp: {}", output);
	let div_thingy: f32 = (1_u32 << 16).into();
	output as f32 / div_thingy
}