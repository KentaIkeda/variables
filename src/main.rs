fn main() {
    // xに5を束縛
    let x = 5; // x = 5
    // xを再宣言し、x + 1を束縛
    let x = x + 1; // x = 6

    { // 新しいスコープを作成
        // 新しいスコープ内でxを再宣言し、x * 2を束縛
        let x = x * 2; // x = 12
        println!("The value of x in the inner scope is: {}", x);
    }

    // xの値は、スコープが作成される前の値である6のまま
    println!("The value of x is: {}", x);

    // シャドーイングを使って、文字列の長さを取得する
    let spaces = "     ";
    let spaces = spaces.len();
    // 変数名を考える手間が省けている
    println!("The value of spaces is: {}", spaces);

    let test = 98_222;
    println!("The value of test is: {}", test);
    let test = 0xff;
    println!("The value of test is: {}", test);
    let test = 0o77;
    println!("The value of test is: {}", test);
    let test = 0b1111_0000;
    println!("The value of test is: {}", test);
    let test = b'A';
    println!("The value of test is: {}", test);
}