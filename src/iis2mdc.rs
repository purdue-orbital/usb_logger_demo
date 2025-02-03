use embedded_hal::i2c::I2c;

const ADDR: u8 = 0b0011110;

pub fn get_ids(bus: &mut impl I2c) -> u8 {
	let mut buf = [0_u8; 1];

	let res = bus.write_read(ADDR, &[0x4F], &mut buf); // check if issues

	if res.is_err() {
		log::error!("{:?}", res);
	}

	buf[0]
}

pub fn setup(bus: &mut impl I2c) {
  let mut buf = [0x80]; // temperature compensation (p21 datasheet)

  bus.write_read(ADDR, &[0x36], &mut buf).unwrap();

  buf[0] &= !0b0100_0000; // set target to 0
  buf[0] |= 0b10 << 5 as u8; // Add setting to buffer

  bus.write(ADDR, &[0x36, buf[0]]).unwrap();
}

pub fn get_x(bus: &mut impl I2c) -> f32 { //issue getting pressure here, refer to 4.4.1 config (pg20) write 1 to 0x36
	let mut buf = [0_u8; 2];

	let res = bus.write_read(ADDR, &[0x68], &mut buf); // get pressure
	if res.is_err() { //error handling
		log::error!("{:?}", res);
	}

	let output = u16::from_le_bytes([buf[0], buf[1]]);

	output as f32
}

pub fn get_y(bus: &mut impl I2c) -> f32 { //issue getting pressure here, refer to 4.4.1 config (pg20) write 1 to 0x36
	let mut buf = [0_u8; 2];

	let res = bus.write_read(ADDR, &[0x6A], &mut buf); // get pressure
	if res.is_err() { //error handling
		log::error!("{:?}", res);
	}

	let output = u16::from_le_bytes([buf[0], buf[1]]);

	output as f32 
}

pub fn get_z(bus: &mut impl I2c) -> f32 { //issue getting pressure here, refer to 4.4.1 config (pg20) write 1 to 0x36
	let mut buf = [0_u8; 2];

	let res = bus.write_read(ADDR, &[0x6C], &mut buf); // get pressure
	if res.is_err() { //error handling
		log::error!("{:?}", res);
	}

	let output = u16::from_le_bytes([buf[0], buf[1]]);

	output as f32
}

// implement get temperature