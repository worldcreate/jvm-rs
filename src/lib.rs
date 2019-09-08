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
    cp_info: Vec<CpInfo>,
    access_flag: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldsInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>
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

#[derive(PartialEq, Debug)]
struct FieldsInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>
}

#[derive(PartialEq, Debug)]
struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>
}

#[derive(PartialEq, Debug)]
struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>
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

    let access_flag = reader.read_u16::<BigEndian>()?;

    let this_class = reader.read_u16::<BigEndian>()?;

    let super_class = reader.read_u16::<BigEndian>()?;

    let interfaces_count = reader.read_u16::<BigEndian>()?;

    let mut interfaces = Vec::new();

    for _ in 0..interfaces_count {
        let interface = reader.read_u16::<BigEndian>()?;

        interfaces.push(interface);
    }

    let fields_count = reader.read_u16::<BigEndian>()?;

    let fields = (0..fields_count).map(|_| {
        let access_flags = reader.read_u16::<BigEndian>().unwrap();
        let name_index = reader.read_u16::<BigEndian>().unwrap();
        let descriptor_index = reader.read_u16::<BigEndian>().unwrap();
        let attributes_count = reader.read_u16::<BigEndian>().unwrap();

        let attributes = (0..attributes_count).map(|_| {
            let attribute_name_index = reader.read_u16::<BigEndian>().unwrap();
            let attribute_length = reader.read_u32::<BigEndian>().unwrap();

            let info = (0..attribute_length).map(|_| {
                reader.read_u8().unwrap()
            }).collect();

            AttributeInfo {attribute_name_index, attribute_length, info}
        }).collect();

        FieldsInfo {access_flags, name_index, descriptor_index, attributes_count, attributes}
    }).collect();

    let methods_count = reader.read_u16::<BigEndian>()?;

    let methods = (0..methods_count).map(|_| {
        let access_flags = reader.read_u16::<BigEndian>().unwrap();
        let name_index = reader.read_u16::<BigEndian>().unwrap();
        let descriptor_index = reader.read_u16::<BigEndian>().unwrap();
        let attributes_count = reader.read_u16::<BigEndian>().unwrap();

        let attributes = (0..attributes_count).map(|_| {
            let attribute_name_index = reader.read_u16::<BigEndian>().unwrap();
            let attribute_length = reader.read_u32::<BigEndian>().unwrap();

            let info = (0..attribute_length).map(|_| {
                reader.read_u8().unwrap()
            }).collect();

            AttributeInfo {attribute_name_index, attribute_length, info}
        }).collect();

        MethodInfo {access_flags, name_index, descriptor_index, attributes_count, attributes}
    }).collect();

    let attributes_count = reader.read_u16::<BigEndian>()?;

    let attributes = (0..attributes_count).map(|_| {
        let attribute_name_index = reader.read_u16::<BigEndian>().unwrap();
            let attribute_length = reader.read_u32::<BigEndian>().unwrap();

            let info = (0..attribute_length).map(|_| {
                reader.read_u8().unwrap()
            }).collect();

            AttributeInfo {attribute_name_index, attribute_length, info}
    }).collect();

    Ok(ClassFile {
        magic,
        minor_version,
        major_version,
        constant_pool_count,
        cp_info: cp_info_vec,
        access_flag,
        this_class,
        super_class,
        interfaces_count,
        interfaces,
        fields_count,
        fields,
        methods_count,
        methods,
        attributes_count,
        attributes})
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
            CpInfo{tag: 12, info: Info::NameAndType {name_index: 4, descriptor_index: 5}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 4, bytes: vec![0x54, 0x65, 0x73, 0x74]}},
            CpInfo{tag: 1, info: Info::Utf8 {length: 0x10, bytes: vec![0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74]}}
            ],
        access_flag: 0x20,
        this_class: 0x02,
        super_class: 0x03,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 0x00,
                name_index: 0x0004,
                descriptor_index: 0x0005,
                attributes_count: 0x0001,
                attributes: vec![AttributeInfo {attribute_name_index: 0x0006, attribute_length: 0x0000001D, info: vec![0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2A, 0xB7, 0x00, 0x01, 0xB1, 0x00, 0x00, 0x00, 
                        0x01, 0x00, 0x07, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]}]
                },
            MethodInfo {
                access_flags: 0x09,
                name_index: 0x08,
                descriptor_index: 0x09,
                attributes_count: 0x01,
                attributes: vec![AttributeInfo {attribute_name_index: 0x06, attribute_length: 0x0000002D, info: vec![0x00, 0x02, 0x00, 0x04, 0x00, 0x00, 0x00, 0x09, 0x03, 0x3C, 0x06, 0x3D,
                        0x1B, 0x1C, 0x60, 0x3E, 0xB1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x07, 0x00, 0x00, 0x00, 0x12, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x02, 0x00, 0x05, 0x00, 0x04, 0x00, 0x07, 0x00, 0x08, 0x00, 0x08]}]
            }],
        attributes_count: 1,
        attributes: vec![AttributeInfo {attribute_name_index: 0x000A, attribute_length: 0x00000002, info: vec![0x00, 0x0B]}]
        };
    assert_eq!(read_from_class().unwrap(), class_file);
}