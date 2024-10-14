use re_viewer::external::egui::{self, ahash::HashMap};

use crate::types::NodeIndex;



trait Drawable {
    // Decorations don't influence the extent of an object an are not considered during a measurement path.
    type Decoration;

    fn draw(&self, ui: &mut egui::Ui, decorations: Self::Decoration) -> egui::Response;
}

