pub struct Graph {
    graph: modulee_engine::graph::Graph,
}

impl Graph {
    #[no_mangle]
    pub extern "C" fn get_debug_value(&self) -> f64 {
        self.graph.get_debug_value()
    }
}
