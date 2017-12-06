use genome::gene_trait::{NUM_TRAIT_PARAMS,Trait};
use link::Link;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NodeType {
    Neuron,
    Sensor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NodePlace {
    Hidden,
    Input,
    Output,
    Bias,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FunctionType {
    Sigmoid,
}

#[derive(Debug)]
pub struct Node {
    /// A node can be given an identification number for saving in files
    pub id: i32,

    /// keeps track of which activation the node is currently in
    activation_count: u32,
    /// Holds the previous step's activation for recurrency
    last_activation: f64,
    /// Holds the activation BEFORE the previous step's.
    /// This is necessary for a special recurrent case when the innode
    /// of a recurrent link is one time step ahead of the outnode.
    /// The innode then needs to send from TWO time steps ago
    last_activation2: f64,

    /// Points to a trait of parameters
    pub node_trait: Option<Rc<RefCell<Trait>>>,


//    /// Used for Gene decoding
//    analogue: Option<Arc<Node>>,

    /// The NNode cannot compute its own output- something is overriding it
    override_node: bool,
    /// Contains the activation value that will override this node's activation
    override_value: f64,

    /// When frozen, cannot be mutated (meaning its trait pointer is fixed)
    frozen: bool,

    /// type is either SIGMOID ..or others that can be added
    function_type: FunctionType,
    /// type is either NEURON or SENSOR
    node_type: NodeType,
    /// Used for genetic marking of nodes
    node_place: NodePlace,

    /// A list of incoming weighted signals from other nodes
    incoming: Vec<Link>,
    /// A list of links carrying this node's signal
    outgoing: Vec<Link>,


    /// The incoming activity before being processed
    activesum: f64,
    /// The total activation entering the NNode
    activation: f64,
    /// To make sure outputs are active
    active_flag: bool,

    // ************ LEARNING PARAMETERS ***********
    // The following parameters are for use in
    //   neurons that learn through habituation,
    //   sensitization, or Hebbian-type processes

    params: [f64; NUM_TRAIT_PARAMS],
}

impl Node {
    pub fn new(id: i32, node_trait: Option<Rc<RefCell<Trait>>>, node_type: NodeType, node_place: NodePlace) -> Self {
        Node {
            id,
            activation_count: 0,
            last_activation: 0.0,
            last_activation2: 0.0,
            node_trait,
            frozen: false,
            override_node: false,
            override_value: 0.0,
            function_type: FunctionType::Sigmoid,
            node_type,
            node_place,
            incoming: Vec::new(),
            outgoing: Vec::new(),
            activesum: 0.0,
            activation: 0.0,
            active_flag: false,
            params: [0.0; NUM_TRAIT_PARAMS],
        }
    }

    pub fn duplicate(&self) -> Self {
        Node {
            id: self.id,
            activation_count: self.activation_count,
            last_activation: 0.0,
            last_activation2: 0.0,
            node_trait: self.node_trait.clone(),
            frozen: false,
            override_node: false,
            override_value: self.override_value,
            function_type: self.function_type,
            node_type: self.node_type,
            node_place: self.node_place,
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone(),
            activesum: 0.0,
            activation: 0.0,
            active_flag: false,
            params: self.params,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        if self.id != other.id { return false }
        if self.node_trait.is_some() && other.node_trait.is_some() {
            let node_trait_1 = self.node_trait.clone().unwrap();
            let node_trait_2 = other.node_trait.clone().unwrap();
            if node_trait_1.borrow().id != node_trait_2.borrow().id {
                return false;
            }
        } else if self.node_trait.is_some() || other.node_trait.is_some() {
            return false;
        }

        if self.override_node != other.override_node { return false }
        if self.function_type != other.function_type { return false }
        if self.node_type != other.node_type { return false }
        if self.node_place != other.node_place { return false }
        if self.incoming.len() != other.incoming.len() { return false }
        if self.outgoing.len() != other.outgoing.len() { return false }
        if self.active_flag != other.active_flag { return false }

        return true;
    }
}
impl Eq for Node {}