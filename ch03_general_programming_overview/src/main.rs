// Rustの関数名、変数名はスネークケースが慣例
fn about_variable() {
    // 普通にletで変数を宣言すると再代入不可になる (imutable)
    let x = 5;
    // x = 6; // このように、xに再代入ができない
    println!("The value of x is: {}", x);

    // xを再度宣言することはできるっぽい。こういう機能をシャドーイングというっぽい。
    let x = "ABC";
    println!("The value of x is: {}", x);
    // スコープを抜けたら元の変数に戻る（変数は覆い隠された状態になるので、新しい変数が解放されたら元の変数になる
    {
        let x = 1234;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    // 定数はconst
    // constは必ず型の定義が必要っぽい。
    // 定数は定数式以外セットできない。C/C++のconstと同じ。
    // 定数の命名規則も例によって同じく大文字と_
    const XXX: &str = "constant value";
    println!("The value of XXX is: {}", XXX);
    const PRICE: u32 = 100_000;
    println!("The value of PRICE is: {}", PRICE);

    // もし変数が初期化されていないのであれば、その変数を使うとエラーになる！null安全で非常に良き！
    // let y: i32;
    // println!("The value of y is: {}", y);
}

fn about_types() {
    // データ型について
    // u32, i32: unsigned int, int (その他、8,16,32,64がある)
    // isize: CPUのアーキテクチャが64ビットなら64, 32なら32
    // f32, f64: 浮動小数点 (基準型はf64)
    //
    // 文字列 => 数値の変換はparseを使う. キャストは戻り値の変数の型がわかっている必要がある
    //
    let x: u32 = "42".parse().expect("Not a number!");
    println!("The value of x is: {}", x);
    let x: f32 = "42.5".parse().expect("Not a number!");
    println!("The value of x is: {}", x);

    // Rustでは型が違うと計算できないっぽい?
    // integer と float の計算はできないけど、u32とi32では計算できるみたい
    // ちなみに型のキャストはasを使う
    let x = -32;
    let y = 42.195;
    let z = x + (y as i32);
    println!("{} + {} = {}", x, y, z);
}

fn about_tuple_array() {
    // Rustでもタプルと配列あるよ
    let tup = (1, "ABC", 1.3);
    println!("tuple is: {}, {}", tup.0, tup.1); // tupleの要素にアクセスする時は`編数名.番号`にする
                                                // 配列の場合、要素は全て同じ型にする必要があるみたい。また、Rustの型は固定長の配列（C/C++の配列的な感じ)
                                                // 可変長の配列はvectorらしい (標準ライブラリにある)
    let arr = [1, 2, 3];
    println!("tuple is: {}, {}", arr[0], arr[1]); // 配列はいつも通り`変数[index]`
                                                  // 同じ値で初期化
    let arr = [1; 4]; // これで[1, 1, 1, 1]になる
                      // タプル、配列はこんな感じで書けば中身確認できる。長いものはできないらしい
    println!("tuple is: {:?}", tup);
    println!("tuple is: {:?}", arr);
}

fn add(x: i32, y: i32) -> i32 {
    // 最後の式の結果が暗黙的に戻り値になるっぽい。
    // 文はNG（式でも、セミコロンをつけると文になるのでNG）。
    // ちなみに、Rustは x = y = 12 みたいな書き方はNG
    x + y
}

fn lets_use_scope() {
    // {} でスコープを表せる。スコープの最後の行を式にすればそのスコープは式として扱われる
    // 即時関数的な取り扱いと考えてよさそう (returnは使えないけどね)
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn control_flow(x: i32) {
    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }

    // ifは式としても使える(if 式)
    let is_even = if x % 2 == 0 { true } else { false };
    if is_even {
        println!("x is even");
    } else {
        println!("x is odd");
    }

    // loopは無限ループ。breakで抜けることができる
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > x {
            break;
        }
        println!("counter is: {}", counter);
    }
    // while文も使えるよ
    // whileは式で書けるのか？ → 書けなかった
    counter = 0;
    while counter < x {
        counter += 1;
        println!("counter is: {}", counter);
    }

    // forはPythonっぽい感じ
    for number in 1..10 {
        println!("{}!", number);
    }

    for number in (1..10).rev() {
        println!("{}!", number);
    }
}

fn main() {
    about_variable();
    about_types();
    about_tuple_array();
    println!("add: 2 + 1 = {}", add(2, 1));
    lets_use_scope();
    control_flow(10)
}
