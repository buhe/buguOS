#![no_std]
#![no_main]

#[macro_use]
extern crate user;

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello OS from app@v-mem");
    0
}