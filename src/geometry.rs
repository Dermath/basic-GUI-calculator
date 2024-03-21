use xcb::{x};
// we need to import the `Xid` trait for the `resource_id` call down there.
use xcb::VoidCookieChecked;

#[derive(Clone)]
pub struct Circle<'a> {
    pub connection: &'a xcb::Connection,
    pub window: x::Window,
    pub gc: x::Gcontext,
    pub x: i16,
    pub y: i16,
    pub radius: i32,
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
                    pixels.push(x::Point{x: x+self.x, y: y+self.y});
                }
            }
        }
        return draw_pix(&pixels, self.connection, self.window, self.gc);
    }

    // pub fn shift(&'a mut self, x_shift_value: i16, y_shift_value: i16) {
    //     self.x = self.x + x_shift_value;
    //     self.y = self.y + y_shift_value;
    // }
}

pub fn draw_pix<'a>(pixels: &Vec<x::Point>, connection: &'a xcb::Connection, window: x::Window, gc: x::Gcontext) -> VoidCookieChecked {
    let addition = connection.send_request_checked(&x::PolyPoint {
        coordinate_mode: x::CoordMode::Origin,
        drawable: x::Drawable::Window(window),
        gc: gc,
        points: &pixels //.collect::<Vec<x::Point>>().as_slice(),
    });
    return addition;
}

