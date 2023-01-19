use fltk::{
    app, button::Button, dialog, enums::Event, frame::Frame, group::Flex, prelude::*,
    text::TextBuffer, window::Window,
};

use crate::editor::Editor;

mod calc_time;
mod editor;

pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

#[derive(Copy, Clone)]
pub enum Message {
    Changed,
    New,
    Open,
    Quit,
    Cut,
    Copy,
    Paste,
    About,
}

fn main() {
    println!("Hello, world!");

    let app = app::App::default();
    let (s, r) = app::channel::<Message>();

    let mut wind = Window::default().with_size(290, 290).with_label("Counter");
    let flex = Flex::default()
        .with_size(280, 280)
        .center_of_parent()
        .column();

    let buf = TextBuffer::default();
    let mut editor = Editor::new(buf.clone());

    let mut buf_formatted = TextBuffer::default();
    let mut _editor2 = Editor::new(buf_formatted.clone());
    // (*editor2).set_readonly(true); Can't for some reason?

    flex.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    editor.emit(s, Message::Changed);

    while app.wait() {
        if let Some(msg) = r.recv() {
            use calc_time::calc_time;
            use Message::*;

            match msg {
                Changed => buf_formatted.set_text(&calc_time(&buf.text())),
                _ => (),
            }
        }
    }
}
