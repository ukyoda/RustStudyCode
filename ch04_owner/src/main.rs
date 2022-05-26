fn takes_ownership(s: String) {
    println!("s in takes_ownership = {}", s);
}

fn takes_ownership_return(s: String) -> String {
    println!("s in takes_ownership_return = {}", s);
    return s;
}

fn main() {
    // ::はString型直下のfrom関数を特定する働きをする演算子 (5章と7章で詳しく議論)
    let s = String::from("hello");
    // Rustの変数はリソースの書き込み自体もダメっぽい。なのでこれはエラーになる。
    // s.push_str(", world!");
    println!("s = {}", s);

    let mut s = String::from("hello");
    // mutableであれば、書き換えOK
    s.push_str(", world!");
    println!("{}", s);

    // 文字列リテラルは静的確保、String型は動的確保
    // Rustのメモリ管理はRAII(Resource Acquisition Is Initialization)。
    // よって、スコープから抜けて不要になった変数は初期化される(drop関数が実行される))。
    // ※GCは使っていない。

    // Rustの代入の基本はコピー。
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Stringの場合の代入は？
    // → リソース（helloという文字列自体)はコピーされない(ポインタがコピーされる)。
    //   一方でそのほかの値(length, capacity)はコピーされる
    //   普通に考えると、s1とs2どちらでもdropで解放されるが、
    //   s1とs2は同じリソースを参照しているので2重開放が発生する
    //   よって、String型の代入はmove(移動)になり、代入元の変数は利用できなくなる
    // → Rustは自動的にデータのDeep Copyが行われることはない
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {}", s2);

    // Rustで文字列コピーしたいならcloneメソッドを使う
    // i32などのコンパイル時に既知のサイズを持つ型、もしくはCopyトレイトに適合した型は
    // 代入後も代入元の変数が利用できる
    //
    // コピー可能な型
    // * あらゆる整数型。u32など。
    // * 論理値型であるbool。trueとfalseという値がある。
    // * あらゆる浮動小数点型、f64など。
    // * 文字型であるchar。
    // * タプル。ただ、Copyの型だけを含む場合。例えば、(i32, i32)はCopyだが、 (i32, String)は違う。
    let s3 = s2.clone();
    println!("s2, s3 = {}, {}", s2, s3);

    // 関数へ値を渡すときは注意しないと。
    takes_ownership(s3);
    // s3は、take_ownershipに値を譲渡しているので、もうs3は使えなくなっている。
    // なので、下記はエラーになる。
    // println!("{}", s3); // エラーになる

    // 関数が値を返すときももちろん所有権が移動する。なのでこれはOK
    let s4 = String::from("hello");
    let s4 = takes_ownership_return(s4);
    println!("s4 = {}", s4);
}
