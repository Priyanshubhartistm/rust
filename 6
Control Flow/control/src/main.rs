// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

fn main() {
    let y: bool = is_even(5);
    println!("value of y is {y}");

}

fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        return true;

    }
    println!("The number is odd");
    false

}