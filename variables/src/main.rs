fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    // let x = "Six"; // this is a shadow version of variable x without using the mut keyword
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
}
