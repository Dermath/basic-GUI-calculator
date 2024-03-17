// we import the necessary modules (only the core X module in this application).
use xcb::{x};
// we need to import the `Xid` trait for the `resource_id` call down there.
use xcb::{Xid};
// use xcb::VoidCookieChecked;

mod geometry;

// Many xcb functions return a `xcb::Result` or compatible result.
fn main() -> xcb::Result<()> {
    let mut name = input("what would you like to name your window? ");
    name.remove(name.len() - 1);
    // name[name.len()] = 0;

    return create_win(&name.as_bytes());
}

// fn draw_circle(connection: &xcb::Connection ,window: x::Window, g_context: x::Gcontext, x_offset: i16, y_offset: i16, radius: i32, border_width: f32) -> VoidCookieChecked {
//     let mut pixels: Vec<x::Point> = vec![];
//     // draw circle
//     let outer_radius: f32 = radius as f32 + border_width;
//     let in_sq: f32 = radius.pow(2) as f32;
//     let out_sq: f32 = outer_radius.powf(2.0);
//     let mut distance: f32;
//     for x in 1..2*(outer_radius as i16) {
//         for y in 1.. 2*(outer_radius as i16) {
//             distance = (x as f32 - outer_radius).powf(2.0) + (y as f32 - outer_radius).powf(2.0);
//
//             if distance > in_sq && distance < out_sq {
//                 pixels.push(x::Point{x: x+x_offset, y: y+y_offset});
//             }
//         }
//     }
//
//     let addition = connection.send_request_checked(&x::PolyPoint {
//         coordinate_mode: x::CoordMode::Origin,
//         drawable: x::Drawable::Window(window),
//         gc: g_context,
//         points: &pixels //.collect::<Vec<x::Point>>().as_slice(),
//     });
//     return addition;
// }

fn input(text: &str) -> String{
    println!("{text}");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    return input;
}

fn create_win(name: &[u8]) -> xcb::Result<()> {
    let scale: u16 = 120;

    // Connect to the X server.
    let (conn, screen_num) = xcb::Connection::connect(None)?;

    // Fetch the `x::Setup` and get the main `x::Screen` object.
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    // Generate an `Xid` for the client window.
    // The type inference is needed here.
    let window: x::Window = conn.generate_id();
    let buffer: x::Pixmap = conn.generate_id();
    let _gc: x::Gc = x::Gc::Foreground(screen.white_pixel());
    // gc = x::Gc::Function(x::Gx::And);
    let g_context: x::Gcontext = conn.generate_id();

    // We can now create a window. For this we pass a `Request`
    // object to the `send_request_checked` method. The method
    // returns a cookie that will be used to check for success.
    let cookie = conn.send_request_checked(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 16 * scale,
        height: 10 * scale,
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

    let _buff_cookie = conn.send_request_checked(&x::CreatePixmap {
        depth: 0,
        pid: buffer,
        drawable: x::Drawable::Pixmap(buffer),
        width: 16 * scale,
        height: 10 * scale,
    });

    let gc_cookie = conn.send_request_checked(&x::CreateGc {
        cid: g_context,
        drawable: x::Drawable::Window(window),
        value_list: &[
            x::Gc::Foreground(screen.white_pixel()),
            // x::Gc::Background(screen.white_pixel()),
            x::Gc::GraphicsExposures(false),
        ],
    });
    conn.check_request(gc_cookie)?;

    // We now check if the window creation worked.
    // A cookie can't be cloned; it is moved to the function.
    // conn.check_request(cookie)?;

    // Let's change the window title
    let _cookie = conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: name,
    });
    // And check for success again
    conn.check_request(_cookie)?;

    // We now show ("map" in X terminology) the window.
    // This time we do not check for success, so we discard the cookie.
    conn.send_request(&x::MapWindow {
        window,
    });

    // We send a few requests in a row and wait for the replies after.
    let (wm_protocols, wm_del_window, wm_state, wm_state_maxv, wm_state_maxh) = {
        let cookies = (
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"WM_PROTOCOLS",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"WM_DELETE_WINDOW",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"_NET_WM_STATE",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"_NET_WM_STATE_MAXIMIZED_VERT",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: true,
                name: b"_NET_WM_STATE_MAXIMIZED_HORZ",
            }),
        );
        (
            conn.wait_for_reply(cookies.0)?.atom(),
            conn.wait_for_reply(cookies.1)?.atom(),
            conn.wait_for_reply(cookies.2)?.atom(),
            conn.wait_for_reply(cookies.3)?.atom(),
            conn.wait_for_reply(cookies.4)?.atom(),
        )
    };

    // We now activate the window close event by sending the following request.
    // If we don't do this we can still close the window by clicking on the "x" button,
    // but the event loop is notified through a connection shutdown error.
    conn.check_request(conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[wm_del_window],
    }))?;

    // Previous request was checked, so a flush is not necessary in this case.
    // Otherwise, here is how to perform a connection flush.
    conn.flush()?;

    let mut maximized = false;
    
    // let mut pos:i16 = 0;
    // We enter the main event loop
    loop {
        
        geometry::Circle{
            connection: &conn,
            window: window,
            gc: g_context,
            x: 50,
            y: 200,
            radius: 300,
            thickness: 4.0
        }.draw();
        // draw my circles
        // draw_circle(&conn, window, g_context, 0, 0, 300, 5.0);
        // draw_circle(&conn, window, g_context, 1000, 500, 200, 20.0);
        // draw_circle(&conn, window, g_context, 602, pos, 300, 5.0);
        // pos = pos + 1;

        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                if ev.detail() == 0x3a {
                    // The M key was pressed
                    // (M only on qwerty keyboards. Keymap support is done
                    // with the `xkb` extension and the `xkbcommon-rs` crate)

                    // We toggle maximized state, for this we send a message
                    // by building a `x::ClientMessageEvent` with the proper
                    // atoms and send it to the server.

                    let data = x::ClientMessageData::Data32([
                        if maximized { 0 } else { 1 },
                        wm_state_maxv.resource_id(),
                        wm_state_maxh.resource_id(),
                        0,
                        0,
                    ]);
                    let event = x::ClientMessageEvent::new(window, wm_state, data);
                    let cookie = conn.send_request_checked(&x::SendEvent {
                        propagate: false,
                        destination: x::SendEventDest::Window(screen.root()),
                        event_mask: x::EventMask::STRUCTURE_NOTIFY,
                        event: &event,
                    });
                    conn.check_request(cookie)?;

                    // Same as before, if we don't check for error, we have to flush
                    // the connection.
                    // conn.flush()?;

                    maximized = !maximized;
                } else if ev.detail() == 0x18 {
                    // Q (on qwerty)

                    // We exit the event loop (and the program)
                    break Ok(());
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
