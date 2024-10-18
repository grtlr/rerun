use rand::thread_rng;

use crate::{
    func::{constant, StrengthFn},
    jiggle::jiggle,
    node::Node,
};
use std::hash::Hash;

pub struct ManyBodyBuilder<Ix: Hash + Eq + Clone> {
    strength: StrengthFn<Ix>,
}

impl<Ix: Hash + Eq + Clone> Default for ManyBodyBuilder<Ix> {
    fn default() -> Self {
        Self {
            strength: constant(-30.0),
        }
    }
}

impl<Ix: Hash + Eq + Clone> ManyBodyBuilder<Ix> {
    pub fn initialize(mut self, nodes: &[Node<Ix>]) -> ManyBody {
        let strengths = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| self.strength.0(&node.ix, i))
            .collect();

        ManyBody {
            strengths,
            theta_2: 0.81,
        }
    }
}

pub struct ManyBody {
    strengths: Vec<f32>,
    theta_2: f32,
}

impl ManyBody {
    pub fn force<Ix: Hash + Eq + Clone>(&mut self, alpha: f32, nodes: &mut [Node<Ix>]) {
        // TODO(grtlr): accerlerate with quadtree + barnes hut.
        for s in 0..nodes.len() {
            let (left, right) = nodes.split_at_mut(s);

            for (i, node) in left.iter_mut().enumerate() {
                for (j, data) in right.iter_mut().enumerate() {
                    let mut x = node.x - data.x;
                    let mut y = node.y - data.y;
                    let mut l = x * x + y * y;
                    if x == 0.0 {
                        x = jiggle(&mut thread_rng());
                        l += x * x;
                    }

                    if y == 0.0 {
                        y = jiggle(&mut thread_rng());
                        l += y * y;
                    }

                    let l = l.sqrt();
                    let w = self.strengths[s + j] * alpha / l;

                    node.vx += x * w;
                    node.vy += y * w;
                }
            }
        }
    }
}
