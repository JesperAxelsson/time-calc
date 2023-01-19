use std::ops::{Deref, DerefMut};

use fltk::{text, enums::{Font, Color, CallbackTrigger}, prelude::{WidgetBase, DisplayExt, WidgetExt, InputExt}};


pub struct Editor {
    editor: text::TextEditor,
}

impl Editor {
    pub fn new(buf: text::TextBuffer) -> Self {
        let mut editor = text::TextEditor::new(5, 35, 790, 560, "");
        editor.set_buffer(Some(buf));

        #[cfg(target_os = "macos")]
        editor.resize(5, 5, 790, 590);

        editor.set_scrollbar_size(15);
        editor.set_text_font(Font::Courier);
        editor.set_linenumber_width(32);
        editor.set_linenumber_fgcolor(Color::from_u32(0x008b_8386));
        editor.set_trigger(CallbackTrigger::Changed);

        Self { editor }
    }
}

impl Deref for Editor {
    type Target = text::TextEditor;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for Editor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}