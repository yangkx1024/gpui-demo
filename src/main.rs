use gpui::*;

use crate::common::{HEIGHT, setup_window, WIDTH};
use crate::list::Main;

mod common;
mod list;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let option = setup_window(WIDTH, HEIGHT, cx);
        cx.open_window(option, |cx| { Main::new(cx) });
    });
}
