use re_types::blueprint::components::VisualBounds2D;
use re_viewer_context::{SpaceViewStateExt as _, TypedComponentFallbackProvider};

use crate::{ui::{bounding_rect_from_iter, GraphSpaceViewState}, GraphSpaceView};

fn valid_bound(rect: &egui::Rect) -> bool {
    rect.is_finite() && rect.is_positive()
}

impl TypedComponentFallbackProvider<VisualBounds2D> for GraphSpaceView {
    fn fallback_for(&self, ctx: &re_viewer_context::QueryContext<'_>) -> VisualBounds2D {
        let Ok(state) = ctx.view_state.downcast_ref::<GraphSpaceViewState>() else {
            return VisualBounds2D::default();
        };

        let default_scene_rect = bounding_rect_from_iter(state.layout.values());

        if valid_bound(&default_scene_rect) {
            default_scene_rect.into()
        } else {
            // Nothing in scene, probably.
            VisualBounds2D::default()
        }
    }
}

re_viewer_context::impl_component_fallback_provider!(GraphSpaceView => [VisualBounds2D]);
