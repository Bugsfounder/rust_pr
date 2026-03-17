fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..10).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");

    // Convert temperatures between Fahrenheit and Celsius.
    // (°F − 32) × 5/9 = 0°C --> fahrenheit to celsius
    // (0°C × 9/5) + 32 = 32°F --> celsius to fahrenheit
    let fehN = 10;
    let celN = 14;

    let feh_to_cel = (fehN - 32) * 5 / 9;
    let cel_to_feh = (celN * 9 / 5) + 32;

    println!("{fehN} fahrenheit into celsius is: {feh_to_cel}");
    println!("{celN} celsius into fahrenheit is: {cel_to_feh}");

    // Generate the nth Fibonacci number
    // n = 5 --> 0 1 1 2 3 5 8 13...

    let n = 5;
    let mut prev = 0;
    let mut curr = 1;
    let mut seq = -1;
    if n == 0 {
        seq = 0
    }

    if n == 1 || n == 2 {
        seq = 1
    }

    println!("{prev}");
    println!("{curr}");
    for i in (1..=n) {
        let next = prev + curr;
        seq = next;
        println!("{seq}");

        prev = curr;
        curr = seq;
    }
}
