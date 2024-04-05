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
    let mut name = input::input("what would you like to name your window? ");
    name.remove(name.len() - 1);
    let num = input::input_num("how large would you like to make the radius of your circles? ");

    return create_win(&name.as_bytes(), num);
}

fn create_win(name: &[u8], size: i32) -> xcb::Result<()> {
    let scale: u16 = 120;

    // Connect to the X server.
    let (conn, screen_num) = xcb::Connection::connect(None)?;

    // Fetch the `x::Setup` and get the main `x::Screen` object.
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();


    let cursor: x::Cursor = conn.generate_id();
    let event_mask: x::EventMask;
    event_mask = x::EventMask::empty();

    // event_mask.NO_EVENT = true;
    // let pointer = 

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
    
    loop {

        let pointer_cookie = conn.send_request(&x::QueryPointer{window: window});
        let pointer_pos = conn.wait_for_reply(pointer_cookie)?;
        let pointer_x = pointer_pos.win_x();
        let pointer_y = pointer_pos.win_y();

        let new = geometry::Circle{
            connection: &conn,
            window: window,
            gc: g_context,
            pos: geometry::Position{x: (pointer_x -11) as u16, y: (pointer_y -11) as u16},
            radius: 10,
            thickness: 2.0
        };
        new.draw();


        let _pointer = x::GrabPointer{
            owner_events: false,
            grab_window: window,
            event_mask: event_mask,
            pointer_mode: x::GrabMode::Sync,
            keyboard_mode: x::GrabMode::Sync,
            confine_to: window,
            cursor: cursor,
            time: 0,
        };

        let button_size: i32 = size;
        let button_border: f32 = 3.0;
        for x in 0..3+1 {
            for y in 0..3+1 {
                let addition = geometry::Circle{
                    connection: &conn,
                    window: window,
                    gc: g_context,
                    pos: geometry::Position{x: (x*button_size*2) as u16, y: (y*button_size*2) as u16},
                    radius: button_size - button_border as i32,
                    thickness: button_border
                }.draw();
                conn.check_request(addition)?;
            }
        }
        
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
