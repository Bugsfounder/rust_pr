fn main() {
    let s = String::from("hello");
    let h = &s[0..5];

    println!("{h}");

    let mut s1 = String::from("hello");

    let s2 = &s1;
    let s3 = &s1;
    println!("{} {}", s2, s3);

    let s4 = &mut s1;
    println!("{}", s4);

    let mut s1 = String::from("hello");
    let r1 = &mut s1;
    r1.push_str(" world");
    let r2 = &s1;
    println!("{}", r2);
}
