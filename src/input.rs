use xcb::{x};

use crate::geometry; 

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

impl <'a> geometry::Env <'a> {
    pub fn pointer_pos(&self) -> Result<geometry::Position, xcb::Error> {
        let pointer_cookie: x::QueryPointerCookie = self.conn.send_request(&x::QueryPointer{window: self.window});
        let pointer = self.conn.wait_for_reply(pointer_cookie)?;
        Ok(geometry::Position{
            x: pointer.win_x() as u16,
            y: pointer.win_y() as u16
        })
    }
}

