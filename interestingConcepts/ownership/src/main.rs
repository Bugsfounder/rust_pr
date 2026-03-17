
fn main() {
    let x = 5;
    let y = x;

    println!("{x}, {y}");

    let s1 = String::from("hello");
    println!("{s1}");
    let s2 = s1;

    println!("{s2}");
    // println!("{s1}"); // error

    let mut s = String::from("Hello");
    s.push('a');
    s.push_str(" world");

    println!("{s}");

    let s: &str = "hello";
    println!("{s}");


    // Ownership and Functions
    println!("Ownership and Functions");
    let s = String::from("hello");
    println!("{s}");

    takes_ownership(s);
    // println!("{s}"); // error

    let x = 5;
    make_copy(x);
    println!("{x}");

    // Return Values and Scope
    println!("Return Values and Scope");
    let s1 = gives_ownership();
    println!("{s1}");
    let s2 = String::from("hello");
    println!("{s2}");
    let s3 = takes_and_gives_back(s2);
    // println!("{s2}"); // ownership has been moved to takes_and_gives_back() function, it will produce error. no reachable
    println!("{s3}");


}
fn gives_ownership() -> String{
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
fn takes_ownership(some_string: String){
    println!("inside function: {some_string}");
}

fn make_copy(some_integer: i32){
    println!("{some_integer}");
}