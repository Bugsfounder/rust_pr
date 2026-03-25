fn main() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1}, {s2}");

    let mut s = String::from("hello");
    // take(s);
    // println!("{s}d");

    borrow(&s);
    println!("{s} d");

    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);

    // can create multiple immutable references
    // can't create mutable and immutable references both at a time
    // can't create multiple mutable references of a variable at a time.
}

fn take(s: String) {
    println!("{s}");
}

fn borrow(x: &String) {
    println!("{}", x);
}
