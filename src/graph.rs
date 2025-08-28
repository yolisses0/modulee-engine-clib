use modulee_engine::ControlUpdateData;
use std::ffi::{c_char, CStr};

pub struct Graph {
    graph: modulee_engine::Graph,
}

#[repr(C)]
pub struct Outputs {
    output_0: f32,
    output_1: f32,
}

impl Graph {
    fn new(sample_rate: f32) -> Self {
        Self {
            graph: modulee_engine::Graph::new(sample_rate),
        }
    }

    #[no_mangle]
    pub extern "C" fn create_graph_pointer(sample_rate: f32) -> *mut Self {
        Box::into_raw(Box::new(Self::new(sample_rate)))
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroy_graph_pointer(ptr: *mut Self) {
        assert!(!ptr.is_null());
        drop(Box::from_raw(ptr));
    }

    #[no_mangle]
    pub extern "C" fn set_graph(&mut self, graph_data: *const c_char) {
        let c_str = unsafe { CStr::from_ptr(graph_data) };
        let graph_json = c_str.to_str().expect("Bad encoding");
        if let Err(e) = self.graph.update_from_json(graph_json) {
            eprintln!("Failed to set graph from JSON: {}\n{}", e, graph_json);
        }
    }

    #[no_mangle]
    pub extern "C" fn process(&mut self) {
        self.graph.process();
    }

    #[no_mangle]
    pub extern "C" fn get_outputs(&self) -> Outputs {
        let outputs = self.graph.get_output_values();
        Outputs {
            output_0: outputs.0,
            output_1: outputs.1,
        }
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
    pub extern "C" fn set_sample_rate(&mut self, sample_rate: f32) {
        self.graph.set_sample_rate(sample_rate);
    }

    #[no_mangle]
    pub extern "C" fn update_control(&mut self, id: usize, value: f32) {
        self.graph.update_control(&ControlUpdateData { id, value });
    }
}
