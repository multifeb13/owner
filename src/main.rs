fn main() {
    let mut s = String::from("Hello");

    s.push_str("!, world!");

    println!("{}", s);
/* -------------------- */
    let x = 5;
    let y = x;
    println!("x = {} y = {}", x, y);

    /* Can't build */
    /*
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}, world!", s1);
    */
}
