enum IpAddrKind {
    V4,
    V6,
}
enum IpAddrKindTagged {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// the standard library has a definition we can use
struct Ip4Addr {}
struct Ip6Addr {}

enum IpAddrStd {
    V4(Ip4Addr),
    V6(Ip6Addr),
}
enum Messsage {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Messsage {
    fn call(&self) {
        println!("Call method called");
    }
}
fn route(ip_kind: IpAddrKind) {}

enum Option<T> {
    None,
    Some(T),
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // IpAddrKindTagged
    let home = IpAddrKindTagged::V4(String::from("192.168.0.0.1"));
    let loopback = IpAddrKindTagged::V6(String::from("::1"));

    let m = Messsage::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = None;
}
