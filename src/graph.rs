use std::slice;

pub struct Graph {
    graph: modulee_engine::graph::Graph,
}

impl Graph {
    #[no_mangle]
    pub extern "C" fn get_debug_value(&self) -> f32 {
        self.graph.get_debug_value()
    }

    fn new() -> Self {
        Self {
            graph: modulee_engine::graph::Graph::new(),
        }
    }

    #[no_mangle]
    pub extern "C" fn create_graph_pointer() -> *mut Self {
        Box::into_raw(Box::new(Self::new()))
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroy_graph_pointer(ptr: *mut Self) {
        assert!(!ptr.is_null());
        drop(Box::from_raw(ptr));
    }

    #[no_mangle]
    pub extern "C" fn process_block(&mut self, buffer: *mut f32, length: usize) {
        let buffer = unsafe { slice::from_raw_parts_mut(buffer, length) };
        self.graph.process_block(buffer, length);
    }
}
