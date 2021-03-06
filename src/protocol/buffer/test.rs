use super::*;

use super::writer::*;
use super::reader::*;


#[test]
pub fn test_buffer_empty() {
    let buffer = Buffer::empty();

    assert_eq!(0, buffer.length())
}

#[test]
pub fn test_buffer_from_data() {
    let data: &[u8] = &[0, 0, 0, 0];
    let buffer = Buffer::from_bytes(data);

    assert_eq!(4, buffer.length());
}

#[test]
pub fn test_buffer_read_write_i32() {
    let mut buffer = Buffer::empty()
        .write_i32_be(9001)
        .write_i32_be(1009);

    let first_num = if let Ok(num) = buffer.read_i32_be() {
        num
    } else {
        panic!("failed to read first number");
    };

    let second_num = if let Ok(num) = buffer.read_i32_be() {
        num
    } else {
        panic!("failed to read second number");
    };

    assert_eq!(9001, first_num);
    assert_eq!(1009, second_num);
}

#[test]
pub fn test_buffer_read_write_i16() {
    let mut buffer = Buffer::empty()
        .write_i16_be(9001 as i16)
        .write_i16_be(1009 as i16);

    let first_num = if let Ok(num) = buffer.read_i16_be() {
        num
    } else {
        panic!("failed to read first number");
    };

    let second_num = if let Ok(num) = buffer.read_i16_be() {
        num
    } else {
        panic!("failed to read second number");
    };

    assert_eq!(9001 as i16, first_num);
    assert_eq!(1009 as i16, second_num);
}


#[test]
pub fn test_buffer_read_write_i16_i32() {
    let mut buffer = Buffer::empty()
        .write_i16_be(9001 as i16)
        .write_i32_be(91910)
        .write_i32_be(81810)
        .write_i16_be(1009 as i16);

    let first_num = if let Ok(num) = buffer.read_i16_be() {
        num
    } else {
        panic!("failed to read first number");
    };

    let second_num = if let Ok(num) = buffer.read_i32_be() {
        num
    } else {
        panic!("failed to read second number");
    };

    let third_num = if let Ok(num) = buffer.read_i32_be() {
        num
    } else {
        panic!("failed to read third number");
    };

    let fourth_num = if let Ok(num) = buffer.read_i16_be() {
        num
    } else {
        panic!("failed to read fourth number");
    };

    assert_eq!(9001 as i16, first_num);
    assert_eq!(91910, second_num);
    assert_eq!(81810, third_num);
    assert_eq!(1009 as i16, fourth_num);
}


#[test]
pub fn test_buffer_read_write_str_null() {
    let mut buffer = Buffer::empty()
        .write_str_null(&"string str string".to_string());

    let first_str = if let Ok(string) = buffer.read_str_null() {
        string
    } else {
        panic!("failed to read string");
    };

    assert_eq!("string str string", first_str);
}

#[test]
pub fn test_buffer_read_write_str() {
    let mut buffer = Buffer::empty()
        .write_str(&"string str string".to_string());

    let first_str = if let Ok(string) = buffer.read_str() {
        string
    } else {
        panic!("failed to read string");
    };

    assert_eq!("string str string", first_str);
}

#[test]
pub fn test_buffer_read_str_len() {
    let mut buffer = Buffer::empty()
        .write_str_no_len(&"string".to_string());

    let first_str = if let Ok(string) = buffer.read_str_len(6) {
        string
    } else {
        panic!("failed to read string");
    };

    assert_eq!("string", first_str);
}

#[test]
pub fn test_buffer_read_write_numbers() {
    let mut buffer = Buffer::empty()
        .write_i32(9001, 3)
        .write_i64(1232, 4)
        .write_i32(1818, 2);

    let first_num = if let Ok(num) = buffer.read_i32(3) {
        num
    } else {
        panic!("failed to read first num");
    };

    let second_num = if let Ok(num) = buffer.read_i64(4) {
        num
    } else {
        panic!("failed to read second num");
    };

    let third_num = if let Ok(num) = buffer.read_i32(2) {
        num
    } else {
        panic!("failed to read third num");
    };

    assert_eq!(first_num, 9001);
    assert_eq!(second_num, 1232);
    assert_eq!(third_num, 1818);
}