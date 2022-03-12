extern crate libc;
extern crate xcb;

use xcb::randr;

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let window_dummy = conn.generate_id();

    xcb::create_window(
        &conn,
        0,
        window_dummy,
        screen.root(),
        0,
        0,
        1,
        1,
        0,
        0,
        0,
        &[],
    );

    conn.flush();

    let screen_res_cookie = randr::get_screen_resources(&conn, window_dummy);
    let screen_res_reply = screen_res_cookie.get_reply().unwrap();
    let crtcs = screen_res_reply.crtcs();

    let mut crtc_cookies = Vec::with_capacity(crtcs.len());
    for crtc in crtcs {
        crtc_cookies.push(randr::get_crtc_info(&conn, *crtc, 0));
    }

    for (i, crtc_cookie) in crtc_cookies.into_iter().enumerate() {
        if let Ok(reply) = crtc_cookie.get_reply() {
            if i != 0 {
                println!("");
            }
            println!("CRTC[{}] INFO:", i);
            println!(" x-off\t: {}", reply.x());
            println!(" y-off\t: {}", reply.y());
            println!(" width\t: {}", reply.width());
            println!(" height\t: {}", reply.height());
        }
    }
}
