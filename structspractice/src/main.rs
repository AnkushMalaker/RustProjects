struct MyFriends{
    name: String,
    age: u32,
    favcolor: Color,
    alive: bool
}

struct Color(i32,i32,i32);

fn build_friend(name: String, favcolor: Color, age: u32) -> MyFriends{
    MyFriends {
        name,
        favcolor,
        age,
        alive: true,
    }
}

fn main() {
    let green = Color(32,32,12);

    let friend1 = build_friend(String::from("John Doe"), green, 21);
    println!("Friend1 name is {}", friend1.name);
}
