pub const NUM_TRAIT_PARAMS: usize = 8;

#[derive(Clone)]
pub struct Trait {
    id: i32,
    params: [f64; NUM_TRAIT_PARAMS]
}

impl Trait {
    pub fn new(id: i32, params: [f64; NUM_TRAIT_PARAMS]) -> Self {
        Trait { id, params }
    }
}