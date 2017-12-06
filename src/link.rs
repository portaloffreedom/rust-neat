use genome::gene_trait::Trait;
use node::Node;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{Eq, PartialEq};

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

use std::fmt::{Formatter, Debug};
use std::fmt::Result as fmtResult;

impl Debug for Link {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        unimplemented!()
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        if self.weight != other.weight { return false; }
        if self.link_trait.is_some() && other.link_trait.is_some() {
            if self.link_trait.ne(&other.link_trait) {
                return false;
            }
        } else if self.link_trait.is_some() || other.link_trait.is_some() {
            return false;
        }

        if self.i_node != other.i_node { return false }
        if self.o_node != other.o_node { return false }
        if self.time_delay != other.time_delay { return false }
        if self.recurrent != other.recurrent { return false }

        return true;
    }
}

impl Eq for Link {}