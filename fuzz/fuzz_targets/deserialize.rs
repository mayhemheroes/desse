use honggfuzz::fuzz;
use desse::{DesseStatic, DesseSized};
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, PartialEq, DesseStatic, DesseSized)]
struct All {
    bool: bool,
    char: char,
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    u8_32: [u8; 32],
    u16_32: [u16; 32],
    u32_32: [u32; 32],
    u64_32: [u64; 32],
    u128_32: [u128; 32],
    i8_32: [i8; 32],
    i16_32: [i16; 32],
    i32_32: [i32; 32],
    i64_32: [i64; 32],
    i128_32: [i128; 32],
}

#[derive(Arbitrary, Debug, PartialEq, DesseStatic, DesseSized)]
struct Primitives {
    bool: bool,
    char: char,
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
}

#[derive(Arbitrary, Debug, PartialEq, DesseStatic, DesseSized)]
struct Arrays {
    u8_32: [u8; 32],
    u16_32: [u16; 32],
    u32_32: [u32; 32],
    u64_32: [u64; 32],
    u128_32: [u128; 32],
    i8_32: [i8; 32],
    i16_32: [i16; 32],
    i32_32: [i32; 32],
    i64_32: [i64; 32],
    i128_32: [i128; 32],
}

fn main() {
    loop {
        fuzz!(|data: ([u8; 67], [u8; 1984], [u8; 2051])| {
            let (primitives_serialized, arrays_serialized, all_serialized) = data;
            let _primitives: Primitives = Primitives::deserialize_from(&primitives_serialized).unwrap();
            let _arrays: Arrays = Arrays::deserialize_from(&arrays_serialized).unwrap();
            let _all: All = All::deserialize_from(&all_serialized).unwrap();
        });
    }
}