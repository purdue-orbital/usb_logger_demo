use embedded_hal::i2c::I2c;

const ADDR: u8 = 0x53;
const REG_POWER_CTL: u8 = 0x2D;
const REG_DATAX0: u8 = 0x32;

fn read_acceleration(bus: &mut impl I2c) -> Result<(i16, i16, i16)> {
    let mut buf = [0_u8; 6];
    let res = bus.write_read(ADDR, &[REG_DATAX0], &mut buf);
    if res.is_err(){
        log::error!("{:?}", res);
    }
    let x = i16::from_le_bytes(buf[0..2]);
    x *= 48.83;
    let y = i16::from_le_bytes(buf[2..4]);
    y *= 48.83;
    let z = i16::from_le_bytes(buf[4..6]);
    z *= 48.83;
    Ok((x, y, z))
}
