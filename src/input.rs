// use xcb::x::*;

// pub struct Cursor {
//     x: i32,
//     y: i32,
// }

pub fn input(text: &str) -> String{
    println!("{text}");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    return input;
}

