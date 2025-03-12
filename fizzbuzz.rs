fn main() {
    println!("");

    let mut i = 0;
    while i < 10 {
        i += 1;
        if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i)
        }
    }

    println!("");
}
