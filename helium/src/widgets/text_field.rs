use super::{Modifiers, Widget};
use crate::{
    colors::tailwind_colors::{NEUTRAL100, NEUTRAL200},
    impl_modifiers,
};
use crystal::{BoxSizing, EmptyLayout};
use helium_core::color::Color;
use helium_renderer::Rect;

/// Contains editable text
pub struct TextField {
    id: String,
    text: String,
    focused: bool,
    /// The background color when this widget is focused.
    pub focus_background_color: Color,
    pub background_color: Color,
    pub corner_radius: u32,
    modifiers: Modifiers,
}

impl TextField {
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(),
            text: String::default(),
            focused: false,
            focus_background_color: NEUTRAL200,
            background_color: NEUTRAL100,
            corner_radius: 0,
            modifiers: Modifiers::new(),
        }
    }

    pub fn on_click(mut self, f: impl FnMut() + 'static) -> Self {
        self
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    /// Set the background color of the [`TextField`] when it is focused.
    pub fn focus_background_color(mut self, focus_background_color: Color) -> Self {
        self.focus_background_color = focus_background_color;
        self
    }

    /// Set the background color of the [`TextField`].
    pub fn background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    fn on_input(&mut self, f: impl FnMut(&str) + 'static) {}

    impl_modifiers!();
}

impl Widget for TextField {
    fn id(&self) -> &str {
        &self.id
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn click(&mut self) {
        self.focused = true;
    }

    fn process_key(&mut self, key: &winit::keyboard::Key) {
        if !self.focused {
            return;
        }

        match key {
            winit::keyboard::Key::Character(character) => {
                self.text.push_str(&character);
            }
            winit::keyboard::Key::Named(named_key) => match named_key {
                winit::keyboard::NamedKey::Backspace => {
                    self.text.pop();
                }
                winit::keyboard::NamedKey::Space => {
                    self.text.push_str(" ");
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn crystal::Layout> {
        let mut layout = EmptyLayout::new();
        layout.id = self.id.clone();
        layout.intrinsic_size = self.modifiers.intrinsic_size;
        Box::new(layout)
    }

    fn draw(&self, layout: &dyn crystal::Layout, renderer: &mut helium_renderer::Renderer) {
        let background_color = match self.focused {
            true => self.focus_background_color,
            false => self.background_color,
        };

        renderer.draw([Rect::from(layout)
            .color(background_color)
            .corner_radius(self.corner_radius as f32)]);

        // Empty text causes panics
        if self.text.is_empty() {
            return;
        }

        renderer.draw([
            helium_renderer::Text::new(&self.text)
                .position(layout.position().x + 16.0, layout.position().y + 16.0), // TODO replace this with a layout
        ]);

        // self.text.draw(&*layout.children()[0], renderer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::{Key, NamedKey, SmolStr};

    #[test]
    fn does_not_update_when_not_focused() {
        let mut text_field = TextField::new();

        let keys = [
            Key::Character(SmolStr::new("H")),
            Key::Character(SmolStr::new("!")),
            Key::Character(SmolStr::new("!")),
        ];

        for key in keys {
            text_field.process_key(&key);
        }

        assert_eq!(text_field.text, "")
    }

    #[test]
    fn text_updates_on_key_input() {
        let mut text_field = TextField::new();
        text_field.focused = true;

        let keys = [
            Key::Character(SmolStr::new("H")),
            Key::Character(SmolStr::new("i")),
            Key::Character(SmolStr::new(" ")),
            Key::Character(SmolStr::new("m")),
            Key::Character(SmolStr::new("o")),
            Key::Character(SmolStr::new("m")),
            Key::Character(SmolStr::new("!")),
        ];

        for key in keys {
            text_field.process_key(&key);
        }

        assert_eq!(text_field.text, "Hi mom!")
    }

    #[test]
    fn backspace_deletes_text() {
        let mut text_field = TextField::new();
        text_field.text = String::from("Hello");
        text_field.focused = true;

        let keys = [
            Key::Named(NamedKey::Backspace),
            Key::Named(NamedKey::Backspace),
        ];

        for key in keys {
            text_field.process_key(&key);
        }

        assert_eq!(text_field.text, "Hel")
    }

    #[test]
    fn space_key_adds_space() {
        let mut text_field = TextField::new();
        text_field.text = String::from("Hello");
        text_field.focused = true;

        let keys = [Key::Named(NamedKey::Backspace), Key::Named(NamedKey::Space)];

        for key in keys {
            text_field.process_key(&key);
        }

        assert_eq!(text_field.text, "Hell ")
    }
}
