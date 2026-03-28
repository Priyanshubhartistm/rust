fn main() {
    let mut number:i32 = 0;
    loop {
        println!("value of number is {number}")
    }
    if number == 10 {
        break;
    }
    number = number + 1;

    println!("the value of number is {number}")
}