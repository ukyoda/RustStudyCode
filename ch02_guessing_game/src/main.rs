// 標準ライブラリ
// https://doc.rust-lang.org/std/prelude/index.html
// preludeについて
//     -> Rustの標準ライブラリは、よく使うものをpreludeという仕組みで自動でインポートされる
//     -> 本来のpreludeはstd::prelude
//     -> それ以外にother preludeというものがある。std::io::preludeがそれ
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut guess_count = 0;

    loop {
        // let mutはmutable, letだけだったらimmutable
        // どうもJavaScriptと違って、letは通常immutable variableっぽい
        // たぶんconstは本当に定数扱いなんじゃないかな？
        // Stringとstrは違うっぽいね。たぶん、strはプリミティブ型なんだろう
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rustは途中で変数を再宣言できるっぽいな
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        guess_count += 1;
    }
    println!("You took {} guesses", guess_count);
}
