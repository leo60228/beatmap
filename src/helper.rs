use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, prelude::*};
use std::convert::TryInto;

pub fn write_aligned_str(writer: &mut impl Write, s: &str) -> io::Result<()> {
    writer.write_u32::<LittleEndian>(s.len().try_into().unwrap())?;
    writer.write_all(s.as_ref())?;
    for _ in 0..(4 - (s.len() % 4)) {
        writer.write_all(&[0])?;
    }
    Ok(())
}

pub fn read_aligned_str(reader: &mut impl Read) -> io::Result<String> {
    let len = reader.read_u32::<LittleEndian>()?;
    let mut bytes: Vec<u8> = Vec::new();

    for _ in 0..len {
        bytes.push(reader.read_u8()?);
    }

    for _ in 0..((4 - len) % 4) {
        let _ = reader.read_u8();
    }

    String::from_utf8(bytes).map_err(|_| io::ErrorKind::InvalidData.into())
}
