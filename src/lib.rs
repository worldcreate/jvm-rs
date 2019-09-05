use std::{
    io::{Result, BufReader},
    fs::File
};
use byteorder::{BigEndian, ReadBytesExt};

fn read_from_class() -> Result<(u32)> {
    let mut reader = BufReader::new(File::open("res/Test.class")?);
    
    let magic = reader.read_u32::<BigEndian>();

    magic
}

#[test]
fn test_read_from_class() {
    assert_eq!(read_from_class().unwrap(), 3_405_691_582);
}