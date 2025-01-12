use std::slice;

pub struct Graph {
    graph: modulee_engine::graph::Graph,
}

impl Graph {
    #[no_mangle]
    pub extern "C" fn get_debug_value(&self) -> f64 {
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

    /// Fill a buffer with audio samples (f32 values) up to the provided length
    #[no_mangle]
    pub extern "C" fn fill_audio(buffer: *mut f32, length: usize) -> usize {
        if buffer.is_null() || length == 0 {
            return 0; // Handle invalid input
        }

        // Generate a simple sine wave as audio data
        let frequency = 440.0; // A4
        let sample_rate = 44100.0;
        let audio_data: Vec<f32> = (0..length)
            .map(|i| ((i as f32 * frequency * 2.0 * std::f32::consts::PI) / sample_rate).sin())
            .collect();

        // Write audio data into the provided buffer
        let output_slice = unsafe { slice::from_raw_parts_mut(buffer, length) };
        for (i, &sample) in audio_data.iter().enumerate() {
            if i < length {
                output_slice[i] = sample;
            } else {
                break;
            }
        }

        // Return the number of samples written
        audio_data.len().min(length)
    }
}
