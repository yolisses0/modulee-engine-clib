use std::{
    ffi::{c_char, CStr},
    slice,
};

pub struct Graph {
    graph: modulee_engine::graph::Graph,
}

impl Graph {
    #[no_mangle]
    pub extern "C" fn get_debug_value(&self) -> f32 {
        self.graph.get_debug_value()
    }

    #[no_mangle]
    pub extern "C" fn set_debug_string(&mut self, pointer: *const c_char) {
        let c_str = unsafe { CStr::from_ptr(pointer) };
        let debug_string = c_str.to_str().expect("Bad encoding");
        self.graph.set_debug_string(debug_string);
        let string_value = self.graph.get_debug_string();
        println!("string value: {}", string_value);
    }

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
