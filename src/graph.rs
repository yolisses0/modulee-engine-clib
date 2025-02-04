use std::{
    ffi::{c_char, CStr},
    slice,
};

pub struct Graph {
    graph: modulee_engine::graph::Graph,
}

impl Graph {
    #[no_mangle]
    pub extern "C" fn set_nodes(&mut self, nodes_data: *const c_char) {
        let c_str = unsafe { CStr::from_ptr(nodes_data) };
        let nodes_json = c_str.to_str().expect("Bad encoding");
        if let Err(e) = self.graph.set_nodes_from_json(nodes_json) {
            eprintln!("Failed to set nodes from JSON: {}\n{}", e, nodes_json);
        }
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
