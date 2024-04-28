// we import the necessary modules (only the core X module in this application).
use xcb::{x};
// we need to import the `Xid` trait for the `resource_id` call down there.
use xcb::{Xid};
// use xcb::VoidCookieChecked;

mod geometry;
mod input;
mod logic;

// Many xcb functions return a `xcb::Result` or compatible result.
fn main() -> xcb::Result<()> {

    let mut backend = logic::Backend {
        new: true,
        num1: 0,
        num2: 0,
        operation: logic::Operation::Inactive,
    };
    // Connect to the X server.
    let (conn, screen_num) = xcb::Connection::connect(None)?;
    let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

    let env = geometry::Env {
        conn: &conn,
        window: conn.generate_id(),
        gc: conn.generate_id(),
        scale: 120,
    };

    let font = conn.generate_id();
    conn.check_request(conn.send_request_checked(&x::OpenFont{
        fid: font,
        name: b"rk24",
    }))?;

    let _cursor: x::Cursor = env.conn.generate_id();
    let _event_mask: x::EventMask;
    _event_mask = x::EventMask::empty();

    // event_mask.NO_EVENT = true;
    // let pointer = 

    // Generate an `Xid` for the client window.
    // The type inference is needed here.

    // We can now create a window. For this we pass a `Request`
    // object to the `send_request_checked` method. The method
    // returns a cookie that will be used to check for success.
    let cookie = conn.send_request_checked(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: env.window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 16 * env.scale,
        height: 10 * env.scale,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        // this list must be in same order than `Cw` enum order
        value_list: &[
            x::Cw::BackPixel(screen.black_pixel()),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS)
        ],
    });
    conn.check_request(cookie)?;

    let cookie = conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: env.window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: b"Calculator",
    });
    // And check for success again
    conn.check_request(cookie)?;

    let gc_cookie = conn.send_request_checked(&x::CreateGc {
        cid: env.gc,
        drawable: x::Drawable::Window(env.window),
        value_list: &[
            x::Gc::Foreground(screen.white_pixel()),
            x::Gc::Font(font),
        ],
    });
    conn.check_request(gc_cookie)?;
    conn.send_request(&x::CloseFont{
        font: font,
    });

    // We now check if the window creation worked.
    // A cookie can't be cloned; it is moved to the function.
    // conn.check_request(cookie)?;

    // We now show ("map" in X terminology) the window.
    // This time we do not check for success, so we discard the cookie.
    conn.send_request(&x::MapWindow {
        window: env.window,
    });

    let button_size: i16 = 300;

    let mut output_box = geometry::Panel {
        env: env,
        pos: geometry::Position {x: button_size * 4, y: 0},
        shape: geometry::Shape::Rect( geometry::Rect {
            env: &env,
            width: 700,
            height: 1200,
            thickness: 15.0
        }),
        text: "0".to_string(),
    };

    let base_button = geometry::Button {
        env: env,
        pos: geometry::Position{x: 0, y: 0},
        shape: geometry::Shape::Rect( geometry::Rect {
            env: &env,
            width: (button_size - 3) as i16,
            height: (button_size - 3) as i16,
            thickness: 3.0
        }),
        text: "test button",
        tag: logic::Tag::Test,
    };
    let base_circle = geometry::Shape::Circle( geometry::Circle {
            env: &env,
            radius: ((button_size/2) - 3) as i16,
            thickness: 3.0
        });

    let mut buttons: Vec<geometry::Button> = vec!(base_button; 16);
    for x in 0..15+1 {
        let y = x/4;
        buttons[x].pos = geometry::Position{x: ((x%4) as i16)*button_size, y: (y as i16)*button_size};
        buttons[x].tag = match x {
            0 => logic::Tag::Num(7),
            1 => logic::Tag::Num(8),
            2 => logic::Tag::Num(9),
            3 => logic::Tag::Op(logic::Operation::Addition),
            4 => logic::Tag::Num(4),
            5 => logic::Tag::Num(5),
            6 => logic::Tag::Num(6),
            7 => logic::Tag::Op(logic::Operation::Subtraction),
            8 => logic::Tag::Num(1),
            9 => logic::Tag::Num(2),
            10 => logic::Tag::Num(3),
            11 => logic::Tag::Op(logic::Operation::Multiplication),
            12 => logic::Tag::Clear,
            13 => logic::Tag::Num(0),
            14 => logic::Tag::Eq,
            15 => logic::Tag::Op(logic::Operation::Division),
            _ => logic ::Tag::Error,
        };
        buttons[x].shape = match x {
            0 => base_circle,
            1 => base_circle,
            2 => base_circle,
            4 => base_circle,
            5 => base_circle,
            6 => base_circle,
            8 => base_circle,
            9 => base_circle,
            10 => base_circle,
            13 => base_circle,
            _ => buttons[x].shape,
        };
        buttons[x].text = match x {
            0 => "7",
            1 => "8",
            2 => "9",
            3 => "+",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "-",
            8 => "1",
            9 => "2",
            10 => "3",
            11 => "x",
            12 => "Clear",
            13 => "0",
            14 => "=",
            15 => "/",
            _ => "",
        };
    }

    // We send a few requests in a row and wait for the replies after.
    let wm_del_window = {
        let cookie = conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"WM_DELETE_WINDOW",
            });
            conn.wait_for_reply(cookie)?.atom()
    };

    // We now activate the window close event by sending the following request.
    // If we don't do this we can still close the window by clicking on the "x" button,
    // but the event loop is notified through a connection shutdown error.
    //conn.check_request(conn.send_request_checked(&x::ChangeProperty {
    //    mode: x::PropMode::Replace,
    //    window: env.window,
    //    property: wm_protocols,
    //    r#type: x::ATOM_ATOM,
    //    data: &[wm_del_window],
    //}))?;

    // Previous request was checked, so a flush is not necessary in this case.
    // Otherwise, here is how to perform a connection flush.
    //conn.flush()?;
    
    loop {
        for i in buttons.iter() {
            let drawn = i.draw();
            i.env.conn.check_request(drawn)?;
        }
        output_box.update(&backend);
        // eprintln!("process: {}", output_box.text);
        conn.check_request(output_box.draw())?;

        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                if ev.detail() == 0x18 {
                    // Q (on qwerty)

                    // We exit the event loop (and the program)
                    break Ok(());
                }
                else if ev.detail() == 0x40 {
                    for i in geometry::update(&buttons)?.iter() {
                        i.click_action(&mut backend)
                    }
                }
            }
            xcb::Event::X(x::Event::ClientMessage(ev)) => {
                // We have received a message from the server
                if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                    if atom == wm_del_window.resource_id() {
                        // The received atom is "WM_DELETE_WINDOW".
                        // We can check here if the user needs to save before
                        // exit, or in our case, exit right away.
                        break Ok(());
                    }
                }
            }
           _ => {}
        }
    }
    // return Ok(0);
}
