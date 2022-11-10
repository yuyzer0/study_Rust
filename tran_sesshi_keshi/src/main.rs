use proconio::input;

fn main() {

    println!("摂氏を入力");
    input! {
        sesshi: f32,
    }

    let kashi = (sesshi * 9.0 / 5.0) + 32.0;
    println!("華氏: {}", kashi);

    println!("華氏を入力");
    input! {
        kashi: f32,
    }
    let sesshi = (kashi - 32.0) * 5.0 / 9.0;
    println!("摂氏: {}", sesshi);
    
}
