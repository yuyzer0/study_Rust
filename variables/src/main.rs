
// const で定数
// グローバルスコープ含めてどんなスコープでも定義できる
// 定数は値の型も必ず注釈する
// 定数は定数式にしかセットできない
// 関数呼び出し結果や、実行次に評価される値にはセットできない
const MAX_POINTS: u32 = 100_00;

fn main() {
    // mut で可変
    let mut x = 5;

    println!("The value of x is: {}" , x);
    x = 6;
    println!("The value of x is: {}" , x);

    println!("The value of MAX_OINTS: {}", MAX_POINTS);

    let y = 5;

    println!("The value of y is: {}", y);

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    let mut spaces = "   ";
    println!("{}", spaces);
    // シャドーイングではなく、変数で可変にすると
    spaces = "    ";
    // spaces = spaces.len() これは型不一致でエラー
    
    // シャドーイングなら、異なる型が許される
    let spaces = spaces.len();
    println!("{}", spaces);
    
}
