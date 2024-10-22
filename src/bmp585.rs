use embedded_hal::i2c::I2c;

// const ADDR: u8 = 0x46; // alt of 0x47
const ADDR: u8 = 0x18; // alt of 0x47

pub fn get_ids(bus: &mut impl I2c) -> [u8; 2] {
	let mut buf = [0_u8; 1];

	let res = bus.write_read(ADDR, &[0x01], &mut buf);

	if res.is_err() {
		log::error!("{:?}", res);
	}

	[buf[0], 0]
}