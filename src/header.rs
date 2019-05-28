use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use std::io::{Read, Write, Error};

use crate::io::*;

pub struct PBOHeader {
    pub filename: String,
    pub method: u32,
    pub original: u32,
    pub reserved: u32,
    pub timestamp: u32,
    pub size: u32,
}

impl PBOHeader {
    pub fn read<I: Read>(input: &mut I) -> Result<PBOHeader, Error> {
        Ok(PBOHeader {
            filename: input.read_cstring()?,
            method: input.read_u32::<LittleEndian>()?,
            original: input.read_u32::<LittleEndian>()?,
            reserved: input.read_u32::<LittleEndian>()?,
            timestamp: input.read_u32::<LittleEndian>()?,
            size: input.read_u32::<LittleEndian>()?
        })
    }

    pub fn write<O: Write>(&self, output: &mut O) -> Result<(), Error> {
        output.write_cstring(&self.filename)?;
        output.write_u32::<LittleEndian>(self.method)?;
        output.write_u32::<LittleEndian>(self.original)?;
        output.write_u32::<LittleEndian>(self.reserved)?;
        output.write_u32::<LittleEndian>(self.timestamp)?;
        output.write_u32::<LittleEndian>(self.size)?;
        Ok(())
    }
}

#[test]
fn read() {
    use std::io::{Cursor};
    let header = crate::header::PBOHeader::read(&mut Cursor::new(String::from("images\\mission.jpg             ��*\\*W i"))).unwrap();
    assert_eq!(header.filename, "images\\mission.jpg");
    assert_eq!(header.size, 1546304959);
    assert_eq!(header.timestamp, 4022190063);
}
