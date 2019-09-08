use std::{
    io::{Result, BufReader, Read},
    fs::File
};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(PartialEq, Debug)]
struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    cp_info: Vec<CpInfo>
}

#[derive(PartialEq, Debug, Clone)]
enum Info {
    Class {name_index: u16},
    Methodref {class_index: u16, name_and_type_index: u16},
    Utf8 {length: u16, bytes: Vec<u8>},
    NameAndType {name_index: u16, descriptor_index: u16}
}

#[derive(PartialEq, Debug, Clone)]
struct CpInfo {
    tag: u8,
    info: Info
}

trait CustomRead {
    fn read_limit(&mut self, limit: u16) -> Vec<u8>;
}

impl CustomRead for BufReader<File> {
    fn read_limit(&mut self, limit: u16) -> Vec<u8> {
        let mut handle = self.take(limit as u64);
        let mut vec = vec![];
        let _ = handle.read_to_end(&mut vec);
        vec
    }
}

#[test]
fn test_read_limit() {
    let mut reader = BufReader::new(File::open("res/Test.class").unwrap());
    let vec = reader.read_limit(4);
    assert_eq!(vec, vec![202, 254, 186, 190]);

    let vec = reader.read_limit(4);
    assert_eq!(vec, vec![0, 0, 0, 55]);
}

fn read_from_class() -> Result<ClassFile> {
    let mut reader = BufReader::new(File::open("res/Test.class")?);
    
    let magic = reader.read_u32::<BigEndian>()?;

    let minor_version = reader.read_u16::<BigEndian>()?;

    let major_version = reader.read_u16::<BigEndian>()?;

    let constant_pool_count = reader.read_u16::<BigEndian>()?;

    let mut cp_info_vec = Vec::new();
    for _ in 0..(constant_pool_count - 1) {
        let tag = reader.read_u8()?;
        match tag {
            1 => {
                let length = reader.read_u16::<BigEndian>()?;
                let vec = reader.read_limit(length);
                cp_info_vec.push(CpInfo {tag, info: Info::Utf8 {length, bytes: vec}})
            }
            7 => {
                let name_index = reader.read_u16::<BigEndian>()?;
                cp_info_vec.push(CpInfo {tag, info: Info::Class {name_index}})
            }
            10 => {
                let class_index = reader.read_u16::<BigEndian>()?;
                let name_and_type_index = reader.read_u16::<BigEndian>()?;
                cp_info_vec.push(CpInfo {tag, info: Info::Methodref {class_index, name_and_type_index}});
            }
            12 => {
                let name_index = reader.read_u16::<BigEndian>()?;
                let descriptor_index = reader.read_u16::<BigEndian>()?;

                cp_info_vec.push(CpInfo {tag, info: Info::NameAndType {name_index, descriptor_index}})
            }
            n => {
                println!("{}: unimplmeneted!", n);
                break;
            }
        }
    }

    Ok(ClassFile {magic, minor_version, major_version, constant_pool_count, cp_info: cp_info_vec})
}

#[test]
fn test_read_from_class() {
    let class_file = ClassFile{
        magic: 3_405_691_582,
        minor_version: 0,
        major_version: 55,
        constant_pool_count: 15,
        cp_info: vec![
            CpInfo{tag: 10, info: Info::Methodref {class_index: 3, name_and_type_index: 12}},
            CpInfo{tag: 7, info: Info::Class {name_index: 13}},
            CpInfo{tag: 7, info: Info::Class {name_index: 14}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 6, bytes: vec![0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 3, bytes: vec![0x28, 0x29, 0x56]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 4, bytes: vec![0x43, 0x6f, 0x64, 0x65]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 15, bytes: vec![0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 4, bytes: vec![0x6d, 0x61, 0x69, 0x6e]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 0x16, bytes: vec![0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 0x0a, bytes: vec![0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 0x09, bytes: vec![0x54, 0x65, 0x73, 0x74, 0x2e, 0x6a, 0x61, 0x76, 0x61]}},
            CpInfo{tag: 12, info: Info::NameAndType {name_index: 4, descriptor_index: 5}}
            ]
        };
    assert_eq!(read_from_class().unwrap(), class_file);
}