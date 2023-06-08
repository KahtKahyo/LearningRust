use std::io;

fn main() {
    let i: i32 = 1;

    let mut i: i32 = 1;

    let pair: (char, i32) = ('a', 1);

    let (some_char, some_int) = ('a', 17);
    assert!(some_char, 'a');
    assert!(some_int, 17);

    let (l, r) = slice.split_at(middle);
    let (_, right) = slice.split_at(middle);

    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    fn greet() {
        println!("Hello!");
    }

    fn fair_dice_roll() -> i32 {
        4
    }

    //interior variable
    {
        let x = "in";
        println!("{}", x);
    }

    let x = {
        let y = 1;
        let z = 2;
        y + z;
    };

    fn fair_dice_roll() -> i32 {
        if feelin_lucky {
            6
        } else {
            4
        }
    }

    let a = (10, 20);
    a.0; // == 10

    let amos = get_some_struct();
    amos.nickname;

    let nick = "fasterthanlime";
    nick.len();

    let least = std::cmp::min(3, 8);

    //crate::file::function

    let x = "amos".len(); // 4
    let x = str::len("amos"); // also 4
}
