use genome::gene_trait::Trait;
use node::Node;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Link {
    /// Weight of the connection
    pub weight: f64,
    /// Node inputting into the link
    pub i_node: Rc<RefCell<Node>>,
    /// NNode that the link affects
    pub o_node: Rc<RefCell<Node>>,
    time_delay: bool,
    recurrent: bool,

    pub link_trait: Option<Rc<RefCell<Trait>>>,

    /// The amount of weight adjustment
    added_weight: f64,
}

impl Link {
    pub fn new(link_trait: Option<Rc<RefCell<Trait>>>, weight: f64, i_node: Rc<RefCell<Node>>, o_node: Rc<RefCell<Node>>, recurrent: bool)
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