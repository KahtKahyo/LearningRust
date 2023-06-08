#![allow(unused)]
//struct can be declared with the struct keyword
struct Number {
    odd: bool,
    value: i32,
}

// struct can be generic too
struct Pair<T> {
    a: T,
    b: T,
}

fn print_number(n: Number) {
    match n.value {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("{}", n.value),
    }
}

//functions can be generic
fn foobar<T>(arg: T) {
    //work with `arg`
}

fn secondFoobar<L, R>(left: L, right: R) {
    //work with `left` and `right`
}
// they can be initialised using iterals

fn main() {
    // Option is not a struct its an enum with 2 variences
    enum Option<T> {
        None,
        Some(T),
    }

    impl<T> Option<T> {}
    fn unwrap(self) -> T {
        match self {
            Self::Some(t) => t,
            Self::None => panic!(...),
        }
    }

    // result is also an enum
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine
    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!

    let p1 = Pair { a: 3, b: 4 }; // Pair<i32>
    let p2 = Pair { a: true, b: false }; // Pair<bool>

    //standard library type of vec is generic
    let mut v1 = Vec::new();
    v1.push(1); // v1 == Vec<i32>
    let mut v2 = Vec::new();
    v2.push(false); // v2 == Vec<bool>

    // can be mutable
    let mut n = Number {
        odd: false,
        value: 1,
    };

    let x = Number {
        odd: false,
        value: 2,
    };
    let y = Number {
        odd: true,
        value: 3,
    };

    //from utf8
    let str1 = str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", str1);

    // this wont work
    //let str2 = str::from_utf8(&[195, 40]);
    let str2 = str::from_utf8(&[195, 40]).unwrap();
    // incase of failure you can unwrap
    // or u can use .expect("custom error") for custom error message
    println!("{:?}", str2);

    let melon = &[240, 159, 141, 137];
    match str::from_utf8(melon) {
        Ok(s) => println!("{}", s),
        Error(e) => panic!(e),
    }

    // safely destructure the inner value
    let melon2 = &[240, 159, 141, 137];
    if let Ok(s) = str::from_utf8(melon2) {
        println!("{}", s);
    }
    Ok(());

    //bubble up the error
    match std::str::from_utf8(melon) {
        Ok(s) => println!("{}", s),
        Err(e) => println!(e),
    }
    Ok(());

    //happy path
    // the shorter version is
    let s = str::from_utf8(melon)?;
    println!("{}", s);
    Ok(());

    // iterators //computed lazily on-demand
    // let natural_num = 1..;
    // 0 or greater
    // (0..).contains(&100); // true
    // 20 or less than 20
    // (..=20).contains(&20); // true
    // only 3, 4, 5
    // (3..6).contains(&4); // true

    for i in vec![52, 49, 21] {
        println!("I like number {}", i);
    }

    for c in "rust".chars() {
        println!("Gibe me a {}", c);
    }

    for c in "SuRPriSe INboUND"
        .chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase())
    {
        print!("{}", c);
    }
    // output URIEBO
}

// macros
// identified by `!` at the end
// example: println!()
