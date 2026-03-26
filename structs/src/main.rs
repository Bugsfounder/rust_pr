use std::iter::Enumerate;

struct User {
    active: bool, // fields
    username: String,
    email: String,
    sign_in_count: u64,
}

fn buildUser(username: String, email: String) -> User {
    User {
        active: true,
        // username: username,
        // email: email,
        // instead of using above two lines we can use field init shorthands, shown below
        username, // field init shorthand
        email,    // field init shorthand
        sign_in_count: 1,
    }
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct AlwaysEqual;
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("bugsfounder"),
        email: String::from("bugsfounder2021@gmail.com"),
        sign_in_count: 1,
    };
    let username = user1.username;
    println!("username: {username}");

    user1.username = String::from("manisha");

    let username = user1.username;
    println!("username: {username}");

    // build user
    let user = buildUser(
        String::from("manishakumari"),
        String::from("manisha@gmail.com"),
    );
    let username = user.username;
    let email = user.email;
    let active = user.active;
    let sign_in_count = user.sign_in_count;
    println!("{username} {email} {active} {sign_in_count}");

    // Struct Update Syntax
    user1.username = String::from("manisha");
    // without struct update syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: user1.email,
    //     sign_in_count: user1.sign_in_count
    // };

    // with struct update syntax
    let user2 = User {
        email: user1.email,
        ..user1
    };

    // use tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // destructure
    let Point(x, y, z) = origin;

    // Unit-Like Structs
    let subject  = AlwaysEqual;
}
