use xcb::{x};
// we need to import the `Xid` trait for the `resource_id` call down there.
use xcb::VoidCookieChecked;
use crate::logic;

#[derive(Clone)]
pub struct Button<'a, 'b> {
    shape: Shape<'a>,
    text: &'b str,
    tag: logic::Tag
}

#[derive(Clone)]
pub enum Shape<'a> {
    Circle(Circle<'a>),
    Rect(Rect<'a>)
}

#[derive(Clone)]
pub struct Env<'a> {
    pub conn: &'a xcb::Connection,
    pub window: x::Window,
    pub gc: x::Gcontext,
    pub scale: u16,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

#[derive(Clone)]
pub struct Circle<'a> {
    pub env: &'a Env<'a>,
    pub pos: Position,
    pub radius: i32,
    pub thickness: f32
}

#[derive(Clone)]
pub struct Rect<'a> {
    pub env: Env<'a>,
    pub pos: Position,
    pub width: i32,
    pub height: i32,
    pub thickness: f32
}

impl<'a> Circle<'a> {
    pub fn draw(&'a self) -> VoidCookieChecked {
        let mut pixels: Vec<x::Point> = vec![];
        // draw circle
        let outer_radius: f32 = self.radius as f32 + self.thickness;
        let in_sq: f32 = self.radius.pow(2) as f32;
        let out_sq: f32 = outer_radius.powf(2.0);
        let mut distance: f32;
        for x in 1..2*(outer_radius as i16) {
            for y in 1.. 2*(outer_radius as i16) {
                distance = (x as f32 - outer_radius).powf(2.0) + (y as f32 - outer_radius).powf(2.0);

                if distance > in_sq && distance < out_sq {
                    pixels.push(x::Point{x: x+self.pos.x as i16, y: y+self.pos.y as i16});
                }
            }
        }
        return draw_pix(&self.env, &pixels);
    }

    // pub fn shift(&'a mut self, x_shift_value: i16, y_shift_value: i16) {
    //     self.x = self.x + x_shift_value;
    //     self.y = self.y + y_shift_value;
    // }
}

impl<'a> Rect<'a> {
    pub fn draw(&'a self) -> VoidCookieChecked {
        let mut pixels: Vec<x::Point> = vec![];

        for x in 1..self.width as u16 {
            for y in 1..self.thickness as u16 {
                pixels.push(x::Point{x: (x+self.pos.x) as i16, y: (y+self.pos.y) as i16})
            }
            for y in (self.height as u16 - self.thickness as u16)..self.height as u16 {
                pixels.push(x::Point{x: (x+self.pos.x) as i16, y: (y+self.pos.y) as i16})
            }
        }
        for x in (self.width - self.thickness as i32) as u16..self.width as u16 {
            for y in 1..self.thickness as u16 {
                pixels.push(x::Point{x: (x+self.pos.x) as i16, y: (y+self.pos.y) as i16})
            }
            for y in (self.height as u16 - self.thickness as u16)..self.height as u16 {
                pixels.push(x::Point{x: (x+self.pos.x) as i16, y: (y+self.pos.y) as i16})
            }
        }
        return draw_pix(&self.env, &pixels);
    }
}

impl <'a> Env <'a> {
    pub fn pointer_pos(&self) -> Result<Position, xcb::Error> {
        let pointer_cookie: x::QueryPointerCookie = self.conn.send_request(&x::QueryPointer{window: self.window});
        let pointer = self.conn.wait_for_reply(pointer_cookie)?;
        Ok(Position{
            x: pointer.win_x() as u16,
            y: pointer.win_y() as u16
        })
    }
}

pub fn draw_pix<'a>(env: &Env, pixels: &Vec<x::Point>) -> VoidCookieChecked {
    let addition = env.conn.send_request_checked(&x::PolyPoint {
        coordinate_mode: x::CoordMode::Origin,
        drawable: x::Drawable::Window(env.window),
        gc: env.gc,
        points: &pixels //.collect::<Vec<x::Point>>().as_slice(),
    });
    return addition;
}

