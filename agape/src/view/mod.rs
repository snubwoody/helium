//! [`View`]'s are responsible for drawing widgets to the screen, as such
//! they hold rendering information such as size, position and color.
//! Rendering is done using `tiny_skia`.
//!

use agape_core::{Color, GlobalId, Position, Rgba, Size};
use tiny_skia::Pixmap;
mod rect;
mod text;

use crate::Resources;
pub use rect::RectView;
pub use text::*;

/// A [`View`] is a primitive object that performs the rendering to the screen.
pub trait View {
    /// Get the view's id
    fn id(&self) -> GlobalId;
    /// Get the view's color
    fn color(&self) -> &Color<Rgba>;

    fn set_id(&mut self, id: GlobalId);
    fn set_size(&mut self, size: Size);
    fn set_position(&mut self, position: Position);

    /// Render the view to the screen.
    fn render(&self, pixmap: &mut Pixmap, resources: &Resources);
}
