use re_viewer::external::{
    egui::{self, emath::TSTransform},
    re_entity_db::InstancePath,
    re_log::external::log,
    re_log_types::EntityPath,
    re_types::SpaceViewClassIdentifier,
    re_ui::{self, UiExt},
    re_viewer_context::{
        IdentifiedViewSystem as _, Item, SpaceViewClass, SpaceViewClassLayoutPriority,
        SpaceViewClassRegistryError, SpaceViewId, SpaceViewSpawnHeuristics, SpaceViewState,
        SpaceViewStateExt as _, SpaceViewSystemExecutionError, SpaceViewSystemRegistrator,
        SystemExecutionOutput, ViewQuery, ViewerContext,
    },
};

use crate::{
    error::Error,
    graph::Graph,
    types::NodeIndex,
    ui::{self, GraphSpaceViewState},
    visualizers::{EdgesDirectedVisualizer, EdgesUndirectedVisualizer, NodeVisualizer},
};

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
        system_registry.register_visualizer::<NodeVisualizer>()?;
        system_registry.register_visualizer::<EdgesDirectedVisualizer>()?;
        system_registry.register_visualizer::<EdgesUndirectedVisualizer>()
    }

    fn new_state(&self) -> Box<dyn SpaceViewState> {
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
            .get(&NodeVisualizer::identifier())
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
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        _space_origin: &EntityPath,
        _space_view_id: SpaceViewId,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let state = state.downcast_mut::<GraphSpaceViewState>()?;

        ui.selection_grid("graph_settings_ui").show(ui, |ui| {
            state.bounding_box_ui(ui);
            state.debug_ui(ui);
            state.layout_provider_ui(ui);
        });

        Ok(())
    }

    /// The contents of the Space View window and all interaction within it.
    ///
    /// This is called with freshly created & executed context & part systems.
    fn ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        state: &mut dyn SpaceViewState,
        query: &ViewQuery<'_>,
        system_output: SystemExecutionOutput,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        let node_system = system_output.view_systems.get::<NodeVisualizer>()?;
        let directed_system = system_output
            .view_systems
            .get::<EdgesDirectedVisualizer>()?;
        let undirected_system = system_output
            .view_systems
            .get::<EdgesUndirectedVisualizer>()?;

        let Some(graph) = Graph::from_nodes_edges(
            &node_system.data,
            &directed_system.data,
            &undirected_system.data,
        ) else {
            log::warn!("No graph data available.");
            return Ok(());
        };

        let state = state.downcast_mut::<GraphSpaceViewState>()?;

        let (id, clip_rect_window) = ui.allocate_space(ui.available_size());
        state.clip_rect_window = clip_rect_window;

        // TODO(grtlr): Consider putting the layout in a option to better signal intent
            let initial_layout = state.layout.empty();

                state.layout.select(graph.all_nodes().map(|n| n.into()));

            // let layout =
            //     state
            //         .layout_provider
            //         .compute(node_sizes.into_iter(), undirected, directed)?;

            // TODO(grtlr): Do this at the end of the layout computation.

            // if let Some(bounding_box) = ui::bounding_rect_from_iter(layout.values()) {
            //     state.fit_to_screen(
            //         bounding_box.scale_from_center(1.05),
            //         clip_rect_window.size(),
            //     );
            // }


        let response = ui.interact(clip_rect_window, id, egui::Sense::click_and_drag());

        // Allow dragging the background as well.
        if response.dragged() {
            state.world_to_view.translation += response.drag_delta();
        }

        let view_to_window = TSTransform::from_translation(ui.min_rect().left_top().to_vec2());
        let world_to_window = view_to_window * state.world_to_view;

        #[cfg(debug_assertions)]
        if response.double_clicked() {
            if let Some(window) = response.interact_pointer_pos() {
                log::debug!(
                    "Click event! Window: {:?}, View: {:?} World: {:?}",
                    window,
                    view_to_window.inverse() * window,
                    world_to_window.inverse() * window,
                );
            }
        }

        if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            // Note: doesn't catch zooming / panning if a button in this PanZoom container is hovered.
            if response.hovered() {
                let pointer_in_world = world_to_window.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);

                // Zoom in on pointer:
                state.world_to_view = state.world_to_view
                    * TSTransform::from_translation(pointer_in_world.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_world.to_vec2());

                // Pan:
                state.world_to_view =
                    TSTransform::from_translation(pan_delta) * state.world_to_view;
            }
        }

        let clip_rect_world = world_to_window.inverse() * clip_rect_window;

        let window_layer = ui.layer_id();

        if state.show_debug {
            let debug_id = egui::LayerId::new(egui::Order::Debug, id.with("debug_layer"));
            ui.ctx().set_transform_layer(debug_id, world_to_window);

            // Paint the coordinate system.
            let painter = egui::Painter::new(ui.ctx().clone(), debug_id, clip_rect_world);

            // paint coordinate system at the world origin
            let origin = egui::Pos2::new(0.0, 0.0);
            let x_axis = egui::Pos2::new(100.0, 0.0);
            let y_axis = egui::Pos2::new(0.0, 100.0);

            painter.line_segment([origin, x_axis], egui::Stroke::new(1.0, egui::Color32::RED));
            painter.line_segment(
                [origin, y_axis],
                egui::Stroke::new(2.0, egui::Color32::GREEN),
            );

            if let Some(bounding_box) = state.layout.bounding_box() {
                painter.rect(
                    bounding_box,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(255, 0, 255, 8),
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 255)),
                );
            }
        }

        for data in node_system.data.iter() {
            let ent_highlight = query.highlights.entity_highlight(data.entity_path.hash());
            // We keep track of the size of the current entity.
            let mut entity_rect: Option<egui::Rect> = None;

            for node in data.nodes() {
                let index = NodeIndex::from(&node);
                let Some(current_extent) = state.layout.get(&index) else {
                    return Err(Error::MissingLayoutInformation(
                        data.entity_path.clone(),
                        node.node_id.clone(),
                    )
                    .into());
                };
                let response = egui::Area::new(id.with(&index))
                    .current_pos(current_extent.min)
                    .order(egui::Order::Middle)
                    .constrain(false)
                    .show(ui.ctx(), |ui| {
                        let highlight = ent_highlight.index_highlight(node.instance);
                        ui.set_clip_rect(clip_rect_world);
                        ui::draw_node(ui, &node, highlight)
                    })
                    .response;

                let instance = InstancePath::instance(data.entity_path.clone(), node.instance);
                ctx.select_hovered_on_click(
                    &response,
                    Item::DataResult(query.space_view_id, instance),
                );

                state.layout.update(index, response.rect);
                entity_rect =
                    entity_rect.map_or(Some(response.rect), |e| Some(e.union(response.rect)));

                let id = response.layer_id;
                ui.ctx().set_transform_layer(id, world_to_window);
                ui.ctx().set_sublayer(window_layer, id);
            }

            let entity_path = data.entity_path.clone();

            if let Some(entity_rect) = entity_rect {
                let entity_id = egui::LayerId::new(
                    egui::Order::Background,
                    id.with(("debug", entity_path.hash())),
                );
                ui.ctx().set_transform_layer(entity_id, world_to_window);
                ui::draw_entity(
                    ui,
                    clip_rect_world,
                    entity_id,
                    entity_rect,
                    &entity_path,
                    &query.highlights,
                );
            }
        }

        for dummy in graph.unknown_nodes() {
            let index = NodeIndex::from(&dummy);
            let current_extent = state.layout
                .get(&index)
                .expect("missing layout information for dummy node");
            let response = egui::Area::new(id.with(&index))
                .current_pos(current_extent.min)
                .order(egui::Order::Middle)
                .constrain(false)
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(clip_rect_world);
                    ui::draw_dummy(ui, &dummy)
                })
                .response;

            state.layout.update(index, response.rect);

            let id = response.layer_id;
            ui.ctx().set_transform_layer(id, world_to_window);
            ui.ctx().set_sublayer(window_layer, id);
        }

        for data in undirected_system.data.iter() {
            let ent_highlight = query.highlights.entity_highlight(data.entity_path.hash());

            for edge in data.edges() {
                let source_ix = NodeIndex::from(edge.source);
                let target_ix = NodeIndex::from(edge.target);
                let source_pos = state.layout
                    .get(&source_ix)
                    .ok_or_else(|| Error::EdgeUnknownNode(edge.source.to_string()))?;
                let target_pos = state.layout
                    .get(&target_ix)
                    .ok_or_else(|| Error::EdgeUnknownNode(edge.target.to_string()))?;

                let response = egui::Area::new(id.with((data.entity_path.hash(), edge.instance)))
                    .current_pos(source_pos.center())
                    .order(egui::Order::Background)
                    .constrain(false)
                    .show(ui.ctx(), |ui| {
                        ui.set_clip_rect(world_to_window.inverse() * clip_rect_window);
                        ui::draw_edge(
                            ui,
                            edge.color,
                            source_pos,
                            target_pos,
                            ent_highlight.index_highlight(edge.instance),
                            false,
                        );
                    })
                    .response;

                let id = response.layer_id;

                ui.ctx().set_transform_layer(id, world_to_window);
                ui.ctx().set_sublayer(window_layer, id);
            }
        }

        for data in directed_system.data.iter() {
            let ent_highlight = query.highlights.entity_highlight(data.entity_path.hash());

            for edge in data.edges() {
                let source_ix = NodeIndex::from(edge.source);
                let target_ix = NodeIndex::from(edge.target);
                let source_pos = state.layout
                    .get(&source_ix)
                    .ok_or_else(|| Error::EdgeUnknownNode(edge.source.to_string()))?;
                let target_pos = state.layout
                    .get(&target_ix)
                    .ok_or_else(|| Error::EdgeUnknownNode(edge.target.to_string()))?;

                let response = egui::Area::new(id.with((data.entity_path.hash(), edge.instance)))
                    .current_pos(source_pos.center())
                    .order(egui::Order::Background)
                    .constrain(false)
                    .show(ui.ctx(), |ui| {
                        ui.set_clip_rect(world_to_window.inverse() * clip_rect_window);
                        ui::draw_edge(
                            ui,
                            edge.color,
                            source_pos,
                            target_pos,
                            ent_highlight.index_highlight(edge.instance),
                            true,
                        );
                    })
                    .response;

                let id = response.layer_id;

                ui.ctx().set_transform_layer(id, world_to_window);
                ui.ctx().set_sublayer(window_layer, id);
            }
        }

         let undirected = undirected_system
                .data
                .iter()
                .flat_map(|d| d.edges().map(|e| (NodeIndex::from(e.source), NodeIndex::from(e.target))));

            let directed = directed_system
                .data
                .iter()
                .flat_map(|d| d.edges().map(|e| (NodeIndex::from(e.source), NodeIndex::from(e.target))));

        state.layout.compute(&mut state.layout_provider, directed, undirected)?;

        Ok(())
    }
}
