fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let mut x = 10;
    println!("The value of x is {x}");
    x = 20;
    println!("The updated value of x is {x}");

    // shadowing
    let x = 10;
    let x = x +10;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is :{x}");
    }

    println!("The value of x is {x}");
    let  spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}")

}
