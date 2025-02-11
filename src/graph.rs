use std::{
    ffi::{c_char, CStr},
    slice,
};

pub struct Graph {
    graph: modulee_engine::graph::Graph,
}

impl Graph {
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

    #[no_mangle]
    pub extern "C" fn set_note_on(&mut self, pitch: f32) {
        self.graph.set_note_on(pitch);
    }

    #[no_mangle]
    pub extern "C" fn set_note_off(&mut self, pitch: f32) {
        self.graph.set_note_off(pitch);
    }

    #[no_mangle]
    pub extern "C" fn set_groups(&mut self, groups_data: *const c_char) {
        let c_str = unsafe { CStr::from_ptr(groups_data) };
        let groups_json = c_str.to_str().expect("Bad encoding");
        if let Err(e) = self.graph.set_groups_from_json(groups_json) {
            eprintln!("Failed to set groups from JSON: {}\n{}", e, groups_json);
        }
    }

    #[no_mangle]
    pub extern "C" fn set_main_group_id(&mut self, main_group_id: usize) {
        self.graph.set_main_group_id(main_group_id);
    }
}
