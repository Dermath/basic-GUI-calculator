use std::fs;
use xcb::{x};
// we need to import the `Xid` trait for the `resource_id` call down there.
use xcb::VoidCookieChecked;
use crate::logic;

#[derive(Clone, Copy)]
pub struct Button<'a, 'b> {
    pub env: Env<'a>,
    pub pos: Position,
    pub shape: Shape<'a>,
    pub text: &'b str,
    pub tag: logic::Tag
}

#[derive(Clone, Copy)]
pub enum Shape<'a> {
    Circle(Circle<'a>),
    Rect(Rect<'a>)
}

#[derive(Clone, Copy)]
pub struct Env<'a> {
    pub conn: &'a xcb::Connection,
    pub window: x::Window,
    pub gc: x::Gcontext,
    pub scale: u16,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: i16,
    pub y: i16 }

#[derive(Clone, Copy)]
pub struct Circle<'a> {
    pub env: &'a Env<'a>,
    pub radius: i16,
    pub thickness: f32
}

#[derive(Clone, Copy)]
pub struct Rect<'a> {
    pub env: &'a Env<'a>,
    pub width: i16,
    pub height: i16,
    pub thickness: f32
}

impl<'a> Circle<'a> {
    pub fn draw(&'a self, pos: Position) -> VoidCookieChecked {
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
                    pixels.push(x::Point{x: x+pos.x as i16, y: y+pos.y as i16});
                }
            }
        }
        return draw_pix(&self.env, &pixels);
    }
    pub fn center(&'a self) -> Position {
        Position{
            x: self.radius/2,
            y: self.radius/2,
        }
    }
    pub fn check_inside(&'a self, pos: Position, check_pos: Position) -> bool {
        let center = Position{x: pos.x + self.radius, y: pos.y + self.radius};
        let distance = (center.x - check_pos.x).pow(2) + (center.y - check_pos.y).pow(2);
        if distance.pow(1/2) <= self.radius {
            return true;
        }
        else {return false};
    }
}

impl<'a> Rect<'a> {
    pub fn draw(&'a self, pos: Position) -> VoidCookieChecked {
        let mut pixels: Vec<x::Point> = vec![];

        for x in 0..=self.width {
            for y in 0..=self.thickness as i16 {
                pixels.push(x::Point{x: x+pos.x, y: y+pos.y})
            }
            for y in (self.height - self.thickness as i16)..=self.height {
                pixels.push(x::Point{x: x+pos.x, y: y+pos.y})
            }
        }
        for y in 0..=self.height {
            for x in 0..=self.thickness as i16 {
                pixels.push(x::Point{x: x+pos.x, y: y+pos.y})
            }
            for x in (self.width - self.thickness as i16)..=self.width {
                pixels.push(x::Point{x: x+pos.x, y: y+pos.y})
            }
        }
        return draw_pix(&self.env, &pixels);
    }
    pub fn center(&'a self) -> Position {
        Position{
            x: self.width/2,
            y: self.height/2,
        }
    }
    pub fn check_inside(&'a self, pos: Position, check_pos: Position) -> bool {
        if pos.x <= check_pos.x && pos.x + self.width >= check_pos.x
        && pos.y <= check_pos.y && pos.y + self.height >= check_pos.y {
            return true;
        }
        else {return false};
    }
}

impl<'a> Shape<'a> {
    pub fn draw(&'a self, pos: Position) -> VoidCookieChecked{
        return match self {
            Shape::Circle(inner) => inner.draw(pos),
            Shape::Rect(inner) => inner.draw(pos),
        }
    }
    pub fn center(&'a self) -> Position {
        match self{
            Shape::Circle(inner) => inner.center(),
            Shape::Rect(inner) => inner.center(),
        }
    }
    pub fn check_inside(&'a self, pos: Position, check_pos: Position) -> bool {
        return match self {
            Shape::Circle(inner) => inner.check_inside(pos, check_pos),
            Shape::Rect(inner) => inner.check_inside(pos, check_pos),
        }
    }
}

impl<'a, 'b> Button<'a, 'b> {
    pub fn draw(&'a self) -> VoidCookieChecked {
        self.shape.draw(self.pos);
        let pos = Position {
            x: self.shape.center().x + self.pos.x,
            y: self.shape.center().y + self.pos.y,
        };
        render_text(&self.env, pos, self.text)
    }
    pub fn check(&'a self) -> Result<bool, xcb::Error> {
        let click = match self.shape{
            Shape::Circle(inner) => inner.env,
            Shape::Rect(inner) => inner.env
        }.pointer_pos()?;
        Ok(self.shape.check_inside(self.pos, click))
    }
} 

impl <'a> Env <'a> {
    pub fn pointer_pos(&self) -> Result<Position, xcb::Error> {
        let pointer_cookie: x::QueryPointerCookie = self.conn.send_request(&x::QueryPointer{window: self.window});
        let pointer = self.conn.wait_for_reply(pointer_cookie)?;
        Ok(Position{
            x: pointer.win_x(),
            y: pointer.win_y()
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

pub fn update(buttons: &Vec<Button>) -> Result<Vec<logic::Tag>, xcb::Error> {
    let mut tags: Vec<logic::Tag> = vec!();

    for i in buttons.iter() {
        match i.check()? {
            true => tags.push(i.tag),
            false => ()
        }
        let drawn = i.draw();
        i.env.conn.check_request(drawn)?;
    }
        return Ok(tags);
}

fn render_text(env: &Env, pos: Position, text: &str) -> VoidCookieChecked {
    let text = text.as_bytes();
    env.conn.send_request_checked(&x::ImageText8{
        drawable: x::Drawable::Window(env.window),
        gc: env.gc,
        x: pos.x,
        y: pos.y,
        string: text,
    })
}
