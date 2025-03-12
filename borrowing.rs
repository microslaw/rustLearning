fn subtract(mut x: i32) -> i32 {
    println!("Pre subtraction {}", x);
    x -= 1;
    println!("Post subtraction {}", x);
    return x;
}

fn main() {
    let x = 5;
    let x2 = subtract(x);
    println!("In main {}",x)
}
