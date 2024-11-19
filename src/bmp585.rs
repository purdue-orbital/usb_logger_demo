use embedded_hal::i2c::I2c;

// const ADDR: u8 = 0x46; // alt of 0x47
const ADDR: u8 = 0x47;

// decide if want to add Low Power Normal mode - apply
// determine OSR rate higher means less error but more power consumed

#[derive(Debug)]
pub enum PowerMode {
	Standby = 0b00,
	Normal = 0b01,
	Forced = 0b10,
	NonStop = 0b11
}

impl PowerMode {
	pub const MASK: u8 = 0b0000_0011;
}

pub fn get_ids(bus: &mut impl I2c) -> u8 {
	let mut buf = [0_u8; 1];

	let res = bus.write_read(ADDR, &[0x01], &mut buf); // check if issues

	if res.is_err() {
		log::error!("{:?}", res);
	}

	buf[0]
}

pub fn get_status(bus: &mut impl I2c) -> u8 {
	let mut buf = [0_u8; 1];

	let res = bus.write_read(ADDR, &[0x28], &mut buf); // check if issues

	if res.is_err() {
		log::error!("{:?}", res);
	}

	let mut nvm_error = buf[0] << 5; // eliminate left digits
	nvm_error = nvm_error >> 7; // eliminate right digits

	let mut nvm_rdy = buf[0] << 6; // eliminate left digits
	nvm_rdy = nvm_rdy >> 7; // eliminate right digits

	if !nvm_error == nvm_rdy {0} else {1} // return 0 if no issues, 1 if issues
}

pub fn get_pressure(bus: &mut impl I2c) -> f32 { //issue getting pressure here, refer to 4.4.1 config (pg20) write 1 to 0x36
	let mut buf = [0_u8; 3];

	let res = bus.write_read(ADDR, &[0x20], &mut buf); // get pressure
	if res.is_err() { //error handling
		log::error!("{:?}", res);
	}

	let output = u32::from_le_bytes([buf[0], buf[1], buf[2], 0]);
	// let div_thingy: f32 = (1_u32 << 6).into();
	let div_thingy: f32 = (1_u32 << 16) as f32;
	output as f32 / div_thingy
}

pub fn get_temperature(bus: &mut impl I2c) -> f32 {
	let mut buf = [0_u8; 3];

	let res = bus.write_read(ADDR, &[0x1D], &mut buf); // get temperature
	if res.is_err() { //error handling
		log::error!("{:?}", res);
	}

	let output = u32::from_le_bytes([buf[0], buf[1], buf[2], 0]);

	//let div_thingy: f32 = (1_u32 << 16).into();
	let div_thingy: f32 = (1_u32 << 16) as f32;
	output as f32 / div_thingy
}

// TODO later check status or something
pub fn set_power_mode(bus: &mut impl I2c, power_mode: PowerMode) {
	let mut buf = [0];

	bus.write_read(ADDR, &[0x37], &mut buf).unwrap();

	buf[0] &= !PowerMode::MASK; // set to 0
	buf[0] |= power_mode as u8; // Add powerMode setting to buffer

	bus.write(ADDR, &[0x37, buf[0]]).unwrap();
}

pub fn set_osr_press(bus: &mut impl I2c) { // Enable pressure reading from OSR
	let mut buf = [0b01111011];

	bus.write_read(ADDR, &[0x36], &mut buf).unwrap();
}

pub fn get_osr_press(bus: &mut impl I2c) -> u8 {
	let mut buf = [0_u8; 1];

	let res = bus.write_read(ADDR, &[0x36], &mut buf); // check if issues

	if res.is_err() {
		log::error!("{:?}", res);
	}

	buf[0]
}

pub fn set_fifo_press(bus: &mut impl I2c) { // Enable pressure reading from FIFO
	let mut buf = [0];

	bus.write_read(ADDR, &[0x18], &mut buf).unwrap();

	buf[0] &= !0b0000_0011; // set to 0
	buf[0] |= 0b11; // add mode setting to buffer (0b11 harder to change than 0b10)
}