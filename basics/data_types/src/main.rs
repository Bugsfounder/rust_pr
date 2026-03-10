
use std::io;
fn main(){
    let guess : u32 = "42".parse().expect("Not a number");

    println!("{guess}");

    let n : i8 = -12;
    println!("{n}");
    let n : u8 = 255;


    let n = 98_222;
    println!("{n}");
    let n = 0xff;
    let n = 0o77;
    let n = 0b1111_0000;
    let n = b'A';

    println!("Size of i8: {}", std::mem::size_of::<i8>());
    println!("Size of i16: {}", std::mem::size_of::<i16>());
    println!("Size of i32: {}", std::mem::size_of::<i32>());
    println!("Size of i64: {}", std::mem::size_of::<i64>());
    println!("Size of i128: {}", std::mem::size_of::<i128>());
    println!("Size of isize: {}", std::mem::size_of::<isize>());

    println!("Size of u8: {}", std::mem::size_of::<u8>());
    println!("Size of u16: {}", std::mem::size_of::<u16>());
    println!("Size of u32: {}", std::mem::size_of::<u32>());
    println!("Size of u64: {}", std::mem::size_of::<u64>());
    println!("Size of u128: {}", std::mem::size_of::<u128>());
    println!("Size of usize: {}", std::mem::size_of::<usize>());


    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");


    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let n = tup.0;
    println!("The value of y is: {y}");
    println!("{n}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a = [3; 5];
    // let a = a[0];
    // println!("{a}");
    for i in 0..5 {
        let b = a[i];
        println!("{b}");
    }



    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index-1];

    println!("The value of the element at index {index} is: {element}");

}