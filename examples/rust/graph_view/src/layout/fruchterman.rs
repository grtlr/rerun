use std::collections::HashMap;

use fdg::{nalgebra::Point2, Force as _};
use rand::distributions::Distribution as _;
use re_viewer::external::egui;

use crate::{error::Error, types::NodeIndex};

#[derive(Debug, Default)]
pub struct FruchtermanReingoldLayout {
    node_force: fdg::fruchterman_reingold::FruchtermanReingold<f32, 2>,
    center_force: fdg::simple::Center,
    graph: fdg::ForceGraph<f32, 2, (NodeIndex, egui::Vec2), ()>,
    node_to_index: HashMap<NodeIndex, petgraph::prelude::NodeIndex>,
}

impl FruchtermanReingoldLayout {
    pub fn compute(
        &mut self,
        nodes: impl IntoIterator<Item = (NodeIndex, egui::Vec2)>,
        directed: impl IntoIterator<Item = (NodeIndex, NodeIndex)>,
        undirected: impl IntoIterator<Item = (NodeIndex, NodeIndex)>,
    ) -> Result<HashMap<NodeIndex, egui::Rect>, Error> {

        // TODO(grtlr): This should not be initialized on every compute iteration.
        let dist = fdg::rand_distributions::Uniform::new(-10.0, 10.0);

        let new_nodes = nodes.into_iter().collect::<HashMap<_, _>>();

        for (node_id, ix) in self.node_to_index {
            if !new_nodes.contains_key(&node_id) {
                self.graph.remove_node(ix);
                self.node_to_index.remove(&node_id);
            }
        }

        for (node_id, size) in new_nodes {
            self.node_to_index.entry(node_id.clone()).or_insert_with(|| {
                self.graph.add_node((
                    (node_id.clone(), size),
                    Point2::new(
                        dist.sample(&mut rand::thread_rng()),
                        dist.sample(&mut rand::thread_rng()),
                    ),
                ))
            });
        }

        // We rebuild the list of edges for every iteration because they don't hold any state.
        self.graph.clear_edges();
        for (source, target) in directed.into_iter().chain(undirected) {
            let source_ix = self.node_to_index
                .get(&source)
                .ok_or_else(|| Error::EdgeUnknownNode(source.to_string()))?;
            let target_ix = self.node_to_index
                .get(&target)
                .ok_or_else(|| Error::EdgeUnknownNode(source.to_string()))?;
            self.graph.add_edge(*source_ix, *target_ix, ());
        }

        // TODO(grtlr): This is incorrect because it makes the forces frame dependent. Ideally we would have a `dt` parameter.
        self.node_force.apply_many(&mut self.graph, 1);
        self.center_force.apply(&mut self.graph);

        let res = self
            .graph
            .node_weights()
            .map(|(data, pos)| {
                let (ix, size) = data;
                let center = egui::Pos2::new(pos.x, pos.y);
                let rect = egui::Rect::from_center_size(center, *size);
                (ix.clone(), rect)
            })
            .collect();

        Ok(res)
    }
}
