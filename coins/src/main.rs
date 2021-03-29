enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter
}

fn valueincents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let myCoin = Coin::Quarter;
    println!("The value of mycoin is {}", valueincents(myCoin));
}
