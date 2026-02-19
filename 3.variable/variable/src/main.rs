fn main() {

    // let mut age: u32 = 30;
    // age = 24;
    // println!("my age is {}", age);

    // constant
    // const PI: u8 = 10;
    // println!("PI value is {}", PI);
    
    // shadowing
    let apples: i32 = 10;
    println!("apples{apples}");

    let apples: i32 = apples + 20;
    println!("apples{apples}");
}
