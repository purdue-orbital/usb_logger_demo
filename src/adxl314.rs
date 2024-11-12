use embedded_hal::i2c::I2c;

const ADDR: u8 = 0x53;
const REG_POWER_CTL: u8 = 0x2D;
const REG_DATAX0: u8 = 0x32;

pub fn get_ids(bus: &mut impl I2c) -> [u8; 2] {
    let mut buf = [0_u8; 1];

    let res = bus.write_read(ADDR, &[0x00], &mut buf);

    if res.is_err() {
        log::error!("{:?}", res);
    }
    
    buf[0];

}

pub fn read_acceleration(bus: &mut impl I2c) -> Result<(f32, f32, f32)> {
    let mut buf = [0_u8; 6];
    let res = bus.write_read(ADDR, &[REG_DATAX0], &mut buf);
    if res.is_err(){
        log::error!("{:?}", res);
    }
    let x:f32 = i16::from_le_bytes(buf[0..2]) as f32;
    x *= 48.83;
    let y:f32 = i16::from_le_bytes(buf[2..4]) as f32;
    y *= 48.83;
    let z:f32 = i16::from_le_bytes(buf[4..6]) as f32;
    z *= 48.83;
    Ok((x, y, z))
}
