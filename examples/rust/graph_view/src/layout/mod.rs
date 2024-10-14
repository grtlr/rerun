use std::collections::HashMap;

use re_viewer::external::egui;

use crate::{error::Error, types::NodeIndex};

mod dot;
pub(crate) use dot::DotLayout;
mod force_directed;
pub(crate) use force_directed::ForceBasedLayout;
mod fruchterman;
pub(crate) use fruchterman::FruchtermanReingoldLayout;

pub fn bounding_rect_from_iter<'a>(
    rectangles: impl Iterator<Item = &'a egui::Rect>,
) -> Option<egui::Rect> {
    // Start with `None` and gradually expand the bounding box.
    let mut bounding_rect: Option<egui::Rect> = None;

    for rect in rectangles {
        bounding_rect = match bounding_rect {
            Some(bounding) => Some(bounding.union(*rect)),
            None => Some(*rect),
        };
    }

    bounding_rect
}

#[derive(Debug, Default)]
pub struct Layout {
    nodes: HashMap<NodeIndex, egui::Rect>,
}

impl Layout {
    // TODO(grtlr): For now we use enumerate to get slight disturbances, in the future we should use a proper random distribution.
    #[deprecated]
    fn rect_from_index(i: usize) -> egui::Rect {
        egui::Rect::from_center_size(
            egui::Pos2::new(100.0 * i as f32, 100.0 * i as f32),
            egui::Vec2::ZERO,
        )
    }

    pub fn select(&mut self, nodes: impl IntoIterator<Item = NodeIndex>) {
        self.nodes = nodes
            .into_iter()
            .enumerate()
            .map(|(i, incoming)| match self.nodes.get_mut(&incoming) {
                Some(rect) => (incoming, *rect),
                None => (incoming, Self::rect_from_index(i)),
            })
            .collect();
    }

    pub fn get(&self, ix: &NodeIndex) -> Option<&egui::Rect> {
        self.nodes.get(ix)
    }

    pub fn update(&mut self, ix: NodeIndex, extent: egui::Rect) -> Option<egui::Rect> {
        self.nodes.insert(ix, extent)
    }

    pub fn bounding_box(&self) -> Option<egui::Rect> {
        bounding_rect_from_iter(self.nodes.values())
    }

    pub fn empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn compute(
        &mut self,
        provider: &mut LayoutProvider,
        directed: impl IntoIterator<Item = (NodeIndex, NodeIndex)>,
        undirected: impl IntoIterator<Item = (NodeIndex, NodeIndex)>,
    ) -> Result<(), Error> {
        // TODO(grtlr): reuse the positions! Also, do this at step!
        let nodes: Vec<(NodeIndex, egui::Vec2)> = std::mem::take(&mut self.nodes)
            .into_iter()
            .map(|(ix, rect)| (ix, rect.size()))
            .collect();

        self.nodes = provider.compute(nodes, directed, undirected)?;

        Ok(())
    }
}

/// The current choice for the layout provider.
#[derive(Debug)]
pub(crate) enum LayoutProvider {
    Dot(DotLayout),
    ForceDirected(ForceBasedLayout),
    FruchtermanReingold(FruchtermanReingoldLayout),
}

impl LayoutProvider {
    pub(crate) fn new_dot() -> Self {
        LayoutProvider::Dot(Default::default())
    }

    pub(crate) fn new_force_directed() -> Self {
        LayoutProvider::ForceDirected(Default::default())
    }

    pub(crate) fn new_fruchterman_reingold() -> Self {
        LayoutProvider::FruchtermanReingold(Default::default())
    }
}

impl PartialEq for LayoutProvider {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LayoutProvider::Dot(_), LayoutProvider::Dot(_)) => true,
            (LayoutProvider::ForceDirected(_), LayoutProvider::ForceDirected(_)) => true,
            (LayoutProvider::FruchtermanReingold(_), LayoutProvider::FruchtermanReingold(_)) => {
                true
            }
            _ => false,
        }
    }
}

impl LayoutProvider {
    pub(crate) fn compute(
        &mut self,
        nodes: impl IntoIterator<Item = (NodeIndex, egui::Vec2)>,
        directed: impl IntoIterator<Item = (NodeIndex, NodeIndex)>,
        undirected: impl IntoIterator<Item = (NodeIndex, NodeIndex)>,
    ) -> Result<HashMap<NodeIndex, egui::Rect>, Error> {
        match self {
            LayoutProvider::Dot(layout) => layout.compute(nodes, directed, undirected),
            LayoutProvider::ForceDirected(layout) => layout.compute(nodes, directed, undirected),
            LayoutProvider::FruchtermanReingold(layout) => {
                layout.compute(nodes, directed, undirected)
            }
        }
    }
}
