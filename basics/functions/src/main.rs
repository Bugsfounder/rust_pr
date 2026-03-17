fn main() {
    // println!("Hello, world!");
    // another_function(2, 'h');

    // let x = 6;
    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("value of y {y}");

    let _x = five();
    let x = plus_one(3);
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn another_function(x: i32, unit_label: char) {
    println!("Another function {x} {unit_label}");
}
