// fn main() {

//     // In Rust, you can specify the type of a variable using a colon followed by the type. This is called type annotation. For example, to declare an integer variable, you can do it like this:
//     // let a: i32 = 10;
//     // println!("The value of a is: {}", a);

//     // Rust has several integer types, including i8, i16, i32, i64, and isize. The isize type is a pointer-sized integer type that can hold any value that can be represented as a pointer. It is typically used for indexing and pointer arithmetic. The size of isize depends on the architecture of the machine (e.g., 32 bits on a 32-bit system and 64 bits on a 64-bit system). Here’s how you can declare an isize variable:
//     // let a: isize = 10;
//     // println!("The value of a is: {}", a);

//     // let a: i8 = 200; // This will cause a compile-time error because 200 is out of the range of i8 (-128 to 127)
//     // println!("The value of a is: {}", a);


//     let a: u8 = 100; // This is valid because 100 is within the range of u8 (0 to 255)
//     println!("The value of a is: {}", a);

//     let a = 33;
//     println!("The value of a is: {}", a);

//     let a: u8 = 254_u8;

//     let s: i32 = 0b1010;
//     println!("value of s is {s}");

//     let h: i32 = 0xff;                  // hex
//     println!("value of hex is {h} ")
// }

// for floating 
// fn main() {
//     let my_f64 = 2.123456789;
//     let my_f32: f32= 2.123456789;

//     println!("my_64 value is: {}", my_f64);
//     println!("my_32 value is: {}", my_f32);

// }


//     fn main() {

//         let priya: (i8, u32, f64, bool) = (21, 2027, 23.32, true);

//         // 1. way to access tuple
//         // let (x: i8, y: u32, z: f64, a: bool) = priya;

//         // 2. way to access tuple
        
//         // 3. ek yasa tuple jiske andr kch v value ni hota h, empty tuple bolte h.

// }

// fn main() {
//     let a: [i32; 6] = [1,2,3,4,5,6];

//     println!("value is {}" , a[0]);

    

// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
