fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let _number = if condition {5} else {6};
    // let number = if condition {5} else {"6"}; // error: because of data type mismatch
    let number = if condition {"5"} else {"6"};
    println!("value of number is: {number}");


    // loop {
    //     println!("again");
    // }

    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result: {result}");


}
