enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {
    match ip_type {
        IpAddrKind::V4 => println!("IP Address Type is IPv4"),
        IpAddrKind::V6 => println!("IP Address Type is IPv6"),
    }
}

// 例によって、列挙型は、値を持つことができる。
// > NOTE
// > IpAddrは標準ライブラリですでに使われているが、ここではそのスコープをuseしていないので使うことが可能
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enumを使ってstructのような定義をすることもできる。また、implを使ってメソッドを定義することもできる
// enumとstructの違いは、メンバの方が同じになるか、別々のままか。
enum MessageForEnum {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageForStruct {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Rustのnull
// Rustにはnullがない。代わりに、Option<T>にNoneが定義されている
// Option<T>は明示的にスコープに導入しなくても利用可能
// ------------------
// matchを使ったパターンマッチング
// Kotolinっぽい感じを覚えた
// Rustのマッチは包括的にする必要がある。網羅しつくさない限り有効にならない
fn pattern_matching(value: u8) {
    // // このような、候補が足りない場合はエラーになる
    // match value {
    //     1 => println!("one"),
    // }
    // _が、いわゆる各種switch文のdefault立ち位置になる
    match value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

// if let
// if letを使えば、if とletの冗長な書き方を省略できる
// if letはswiftにもあるらしい
fn if_let(value: Option<i32>) {
    if let Some(v) = value {
        println!("{}", v);
    } else {
        println!("None");
    }

    // これと同じ意味
    match value {
        Some(v) => println!("{}", v),
        None => println!("None"),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    println!("==========================");
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    pattern_matching(1);
    pattern_matching(2);
    pattern_matching(3);
    pattern_matching(4);
    pattern_matching(255);
    if_let(Some(1));
    if_let(None);
}
