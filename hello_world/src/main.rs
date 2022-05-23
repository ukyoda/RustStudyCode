// rustfmtを使うとprettierみたいにコードスタイルを統一できる
// `cargo build` でデバッグビルド
// `cargo build -r` でリリースビルド
// `cargo run` でビルド＆実行
// `cargo`のおすすめパッケージ (https://zenn.dev/toru3/articles/14312f4dbf18b6)
// `cargo check`はビルドのdry-runっぽい
// 基本、cargoを使って開発するほうがよさそう

fn main() {
    // printlnの`!`は何？
    // Rustはセミコロンなくてもいいっぽい
    println!("Hello, World!!");
}
