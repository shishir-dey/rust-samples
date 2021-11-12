use std::mem;

fn main() {
    println!("----Unsigned Datatypes----");
    println!("size_of u8 = {}", mem::size_of::<u8>());
    println!("size_of u16 = {}", mem::size_of::<u16>());
    println!("size_of u32 = {}", mem::size_of::<u32>());
    println!("size_of u64 = {}", mem::size_of::<u64>());
    println!("size_of u128 = {}", mem::size_of::<u128>());
    println!("size_of usize = {}", mem::size_of::<usize>());
    
    println!("----Unsigned Datatypes----");
    println!("size_of i8 = {}", mem::size_of::<i8>());
    println!("size_of i16 = {}", mem::size_of::<i16>());
    println!("size_of i32 = {}", mem::size_of::<i32>());
    println!("size_of i64 = {}", mem::size_of::<i64>());
    println!("size_of i128 = {}", mem::size_of::<i128>());
    println!("size_of isize = {}", mem::size_of::<isize>());

    println!("----Others----");
    println!("size_of char = {}", mem::size_of::<char>());
    println!("size_of bool = {}", mem::size_of::<bool>());

}