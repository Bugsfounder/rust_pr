fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("value: {s1}, len: {len}");

    change(&mut s1);
    {
        let r2 = &mut s1;
        println!(" {r2}");
    }
    let r1 = &mut s1;
    // println!("{s1}");
    println!("{r1}");

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{r1}, {r2}");
    println!("{r1}, {r2}");

    let r3 = &mut s;
    println!("{r3}");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" added");
}
