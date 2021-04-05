use std::io::stdin;

fn main() {
    let x = 10;
    let y : i8;
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).expect("Not a number?");
    y = input_string.trim().parse::<i8>().unwrap();
    if y != 0{
    print!("Lets see if this works!: {}", x/y);
    }
    else{
        print!("Really???");
    }
}

