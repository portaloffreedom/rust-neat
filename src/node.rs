use genome::gene_trait::Trait;
use link::Link;
use std::sync::Arc;

#[derive(Copy, Clone)]
pub enum NodeType {
    Neuron,
    Sensor,
}

#[derive(Copy, Clone)]
pub enum NodePlace {
    Hidden,
    Input,
    Output,
    Bias,
}

#[derive(Copy, Clone)]
pub enum FunctionType {
    Sigmoid,
}

pub struct Node {
    /// A node can be given an identification number for saving in files
    id: i32,

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
    node_trait: Option<Arc<Trait>>,

    /// When frozen, cannot be mutated (meaning its trait pointer is fixed)
    frozen: bool,

    /// The NNode cannot compute its own output- something is overriding it
    override_node: bool,

    /// Contains the activation value that will override this node's activation
    override_value: f64,

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
}

impl Node {
    pub fn new(id: i32, node_trait: Option<Arc<Trait>>, node_type: NodeType, node_place: NodePlace) -> Self {
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
        }
    }

    pub fn clone(&self) -> Self {
        Node {
            id: self.id,
            activation_count: self.activation_count,
            last_activation: self.last_activation,
            last_activation2: self.last_activation2,
            node_trait: self.node_trait,
            frozen: self.frozen,
            override_node: self.override_node,
            override_value: self.override_value,
            function_type: self.function_type,
            node_type: self.node_type,
            node_place: self.node_place,
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone(),
        }
    }
}