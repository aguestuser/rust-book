fn main() {
    // enumerable kinds

    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_type: IpAddrKind) {}

    route(four);
    route(six);

    // structs of kind

    struct IpAddr_ {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr_ {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr_ {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // terser!

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // ADT-o-rama!!!

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // meet pattern-matching...

    // ...with coins

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    enum UsState {
        Alabama,
        Alaska,
        NoState,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25,
        }
    }

    // ... with Option

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    println!("plus_one(Some(5)): {:?}", plus_one(Some(5)));
    println!("plus_one(None): {:?}", plus_one(None));

    // tersen up with higher-order-functions!
    fn increment(x: Option<i32>) -> Option<i32> {
        x.map(|n| n + 1)
    }

    println!("increment(Some(5)): {:?}", increment(Some(5)));
    println!("increment(None): {:?}", increment(None));

    // if let

    let some_u8_num = some(0u8);

    // this...
    match some_u8_num {
        Some(3) => println!("it's three!"),
        _ => (),
    }

    //... is equivalent to this:

    if let Some(3) = some_u8_num {
        println!("it's three!");
    } // but we lose exhaustive checking for all patterns!
}
