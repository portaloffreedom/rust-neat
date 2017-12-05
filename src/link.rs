use genome::gene_trait::Trait;
use node::Node;
use std::sync::Arc;

#[derive(Clone)]
pub struct Link {
    /// Weight of the connection
    weight: f64,
    /// Node inputting into the link
    i_node: Arc<Node>,
    /// NNode that the link affects
    o_node: Arc<Node>,
    time_delay: bool,
    recurrent: bool,

    link_trait: Option<Arc<Trait>>,

    /// The amount of weight adjustment
    added_weight: f64,
}

impl Link {
    pub fn new(link_trait: Option<Arc<Trait>>, weight: f64, i_node: Arc<Node>, o_node: Arc<Node>, recurrent: bool)
               -> Self
    {
        Link {
            link_trait,
            weight,
            added_weight: 0.0,
            i_node,
            o_node,
            recurrent,
            time_delay: false,
        }
    }
}