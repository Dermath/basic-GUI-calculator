// use xcb::x::*;
// mod geometry;

pub fn input(text: &str) -> String{
    println!("{text}");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    return input;
}

pub fn input_num(text: &str) -> i32{
    println!("{text}");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    
    let num_input: i32 = input.trim().parse().unwrap();

    return num_input;
}
