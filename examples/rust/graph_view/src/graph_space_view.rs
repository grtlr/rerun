use std::collections::HashMap;

use re_viewer::external::{
    egui::{self, Color32, Label},
    re_log::external::log,
    re_log_types::EntityPath,
    re_types::SpaceViewClassIdentifier,
    re_ui,
    re_viewer_context::{
        HoverHighlight, IdentifiedViewSystem as _, SelectionHighlight, SpaceViewClass, SpaceViewClassLayoutPriority, SpaceViewClassRegistryError, SpaceViewId, SpaceViewSpawnHeuristics, SpaceViewState, SpaceViewStateExt as _, SpaceViewSystemExecutionError, SpaceViewSystemRegistrator, SystemExecutionOutput, ViewQuery, ViewerContext
    },
};

use crate::node_visualizer_system::GraphNodeVisualizer;
use crate::{common::QualifiedNode, edge_visualizer_system::GraphEdgeVisualizer};

// We need to differentiate between regular nodes and nodes that belong to a different entity hierarchy.
enum NodeKind {
    Regular,
    Dummy,
}

/// Space view state for the custom space view.
///
/// This state is preserved between frames, but not across Viewer sessions.
#[derive(Default)]
pub struct GraphSpaceViewState {
    graph: petgraph::stable_graph::StableGraph<NodeKind, ()>,
    node_to_index: HashMap<QualifiedNode, petgraph::stable_graph::NodeIndex>,
}

impl SpaceViewState for GraphSpaceViewState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct GraphSpaceView;

impl SpaceViewClass for GraphSpaceView {
    // State type as described above.

    fn identifier() -> SpaceViewClassIdentifier {
        "Graph".into()
    }

    fn display_name(&self) -> &'static str {
        "Graph"
    }

    fn icon(&self) -> &'static re_ui::Icon {
        &re_ui::icons::SPACE_VIEW_GENERIC
    }

    fn help_markdown(&self, _egui_ctx: &egui::Context) -> String {
        "A space view that shows a graph as a node link diagram.".to_owned()
    }

    /// Register all systems (contexts & parts) that the space view needs.
    fn on_register(
        &self,
        system_registry: &mut SpaceViewSystemRegistrator<'_>,
    ) -> Result<(), SpaceViewClassRegistryError> {
        system_registry.register_visualizer::<GraphNodeVisualizer>()?;
        system_registry.register_visualizer::<GraphEdgeVisualizer>()
    }

    fn new_state(&self) -> Box<dyn SpaceViewState> {
        log::debug!("Creating new GraphSpaceViewState");
        Box::<GraphSpaceViewState>::default()
    }

    fn preferred_tile_aspect_ratio(&self, _state: &dyn SpaceViewState) -> Option<f32> {
        // Prefer a square tile if possible.
        Some(1.0)
    }

    fn layout_priority(&self) -> SpaceViewClassLayoutPriority {
        Default::default()
    }

    fn spawn_heuristics(&self, ctx: &ViewerContext<'_>) -> SpaceViewSpawnHeuristics {
        // By default spawn a single view at the root if there's anything the visualizer is applicable to.
        if ctx
            .applicable_entities_per_visualizer
            .get(&GraphNodeVisualizer::identifier())
            .map_or(true, |entities| entities.is_empty())
        {
            SpaceViewSpawnHeuristics::default()
        } else {
            SpaceViewSpawnHeuristics::root()
        }
    }

    /// Additional UI displayed when the space view is selected.
    ///
    /// In this sample we show a combo box to select the color coordinates mode.
    fn selection_ui(
        &self,
        _ctx: &ViewerContext<'_>,
        _ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        _space_origin: &EntityPath,
        _space_view_id: SpaceViewId,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let _state = state.downcast_mut::<GraphSpaceViewState>()?;

        // ui.horizontal(|ui| {
        //     ui.label("Coordinates mode");
        //     egui::ComboBox::from_id_salt("color_coordinates_mode")
        //         .selected_text(state.mode.to_string())
        //         .show_ui(ui, |ui| {
        //             for mode in &ColorCoordinatesMode::ALL {
        //                 ui.selectable_value(&mut state.mode, *mode, mode.to_string());
        //             }
        //         });
        // });

        Ok(())
    }

    /// The contents of the Space View window and all interaction within it.
    ///
    /// This is called with freshly created & executed context & part systems.
    fn ui(
        &self,
        _ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        query: &ViewQuery<'_>,
        system_output: SystemExecutionOutput,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let node_system = system_output.view_systems.get::<GraphNodeVisualizer>()?;
        let edge_system = system_output.view_systems.get::<GraphEdgeVisualizer>()?;

        let state = state.downcast_mut::<GraphSpaceViewState>()?;

        // TODO(grtlr): Once we settle on a design, we should update the graph instead of constructing it from scratch.
        state.graph.clear();
        state.node_to_index.clear();

        for data in node_system.data.iter() {
            for (node_id, _, _) in data.nodes() {
                let node_index = state.graph.add_node(NodeKind::Regular);
                state.node_to_index.insert(node_id, node_index);
            }
        }

        for data in edge_system.data.iter() {
            for (edge, _, _) in data.edges() {
                let source_index = *state
                    .node_to_index
                    .entry(edge.source)
                    .or_insert(state.graph.add_node(NodeKind::Dummy));
                let target_index = *state
                    .node_to_index
                    .entry(edge.target)
                    .or_insert(state.graph.add_node(NodeKind::Dummy));
                state.graph.add_edge(source_index, target_index, ());
            }
        }

        egui::Frame {
            inner_margin: re_ui::DesignTokens::view_padding().into(),
            ..egui::Frame::default()
        }
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.label(egui::RichText::new("Nodes").underline());

                    for data in node_system.data.iter() {
                        let ent_highlight = query.highlights.entity_highlight(data.entity_path.hash());

                        for (node, instance, maybe_color) in data.nodes() {
                            let highlight = ent_highlight.index_highlight(instance);

                            let hcolor = match (
                                highlight.hover,
                                highlight.selection != SelectionHighlight::None,
                            ) {
                                (HoverHighlight::None, false) => egui::Color32::BLACK,
                                (HoverHighlight::None, true) => ui.style().visuals.selection.bg_fill,
                                (HoverHighlight::Hovered, ..) => ui.style().visuals.widgets.hovered.bg_fill,
                            };

                            let text = egui::RichText::new(format!(
                                "{}: {}",
                                node.entity_path, node.node_id,
                            ));

                            if let Some(color) = maybe_color {
                                let c = Color32::from(color.0);
                                ui.add(Label::new(text.color(c).background_color(hcolor)));
                            } else {
                                ui.add(Label::new(text.background_color(hcolor)));
                            }
                        }
                    }

                    ui.label(egui::RichText::new("Edges").underline());

                    for data in edge_system.data.iter() {
                        let ent_highlight = query.highlights.entity_highlight(data.entity_path.hash());
                        for (edge, instance, maybe_color) in data.edges() {
                            let highlight = ent_highlight.index_highlight(instance);

                            let hcolor = match (
                                highlight.hover,
                                highlight.selection != SelectionHighlight::None,
                            ) {
                                (HoverHighlight::None, false) => egui::Color32::BLACK,
                                (HoverHighlight::None, true) => ui.style().visuals.selection.bg_fill,
                                (HoverHighlight::Hovered, ..) => ui.style().visuals.widgets.hovered.bg_fill,
                            };

                            let text = egui::RichText::new(format!(
                                "{}: {:?}:{} -> {:?}:{}",
                                data.entity_path,
                                edge.source.entity_path,
                                edge.source.node_id,
                                edge.target.entity_path,
                                edge.target.node_id,
                            ));

                            if let Some(color) = maybe_color {
                                let c = Color32::from(color.0);
                                ui.add(Label::new(text.color(c).background_color(hcolor)));
                            } else {
                                ui.add(Label::new(text.background_color(hcolor)));
                            }
                        }
                    }
                })
            });
        });

        Ok(())
    }
}
