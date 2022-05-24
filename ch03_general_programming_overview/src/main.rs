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

fn main() {
    about_variable();
    about_types();
    about_tuple_array();
}
