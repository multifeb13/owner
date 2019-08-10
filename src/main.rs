fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("string = {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("integer = {}", some_integer);
}
