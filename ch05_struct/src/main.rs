// #[derive(debug)]はトレイトというものである。これをつけることでprintlnで構造体を出力できる。
// derive注釈で使える鳥とはほかにもいろいろある
#[derive(Debug)]
struct User {
    // &strではなく、所有権のあるString型を使用している。
    // これは、構造体に全データの所有権を持たせる必要があるため。
    // 構造体全体が有効な間はずっと有効である必要がある。
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // TypeScriptと同じ感じで、変数とフィールドの名前が一致していたら省略して書ける
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッドを定義するにはimplブロックを使う
impl Rectangle {
    // メソッドは自信を受け取るために&selfをつける。この辺りはPythonっぽい
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数はselfを引数にとらない(static methodみたいなもの)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    println!("Hello, world!");
    let user = User {
        email: String::from("aaa"),
        username: String::from("bbb"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("ccc"), String::from("ddd"));
    println!("User = {:?}", user);
    println!("User = {:#?}", user2); // {:#?}だったら開業もされて見やすい

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 area = {}", area(&rect1));
    println!("rect1 area = {}", rect1.area());
    println!("rect1 can_hold = {}", rect1.can_hold(&rect1));
    println!("generate rect = {:#?}", Rectangle::square(10));
}
