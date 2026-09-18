use q_recognizer::gesture::*;
use q_recognizer::q_point_cloud_recognizer::{classify, QParameters};
use wasm_bindgen::prelude::*;
use crate::wrappers::*;
/* 
// ---------------------------------------------------------------------
// Global storage
// ---------------------------------------------------------------------
// Global, thread-safe registry of recorded gestures.
// A plain `static Vec<T>` can't be mutated, so it's wrapped in a Mutex.
// OnceLock (std-only, no external crates) lazily initializes it on first use.
static GESTURES: OnceLock<Mutex<Vec<Gesture>>> = OnceLock::new();

// Returns the global gestures registry, initializing it on first access.
pub fn gestures() -> &'static Mutex<Vec<Gesture>> {
    GESTURES.get_or_init(|| Mutex::new(Vec::new()))
}*/

// ---------------------------------------------------------------------
// GesturesManager
// ---------------------------------------------------------------------


#[wasm_bindgen]
pub struct GesturesManager {
    gestures: Vec<Gesture>,
    raw_points_buffer: Vec<PointWrapper>, // fixed-length, reused across gestures
    counter: usize,                // current write position in the buffer
    stroke_cnt: i32
}

#[wasm_bindgen]
impl GesturesManager {
    #[wasm_bindgen(constructor)]
    pub fn new(buffer_size: usize) -> Self {
        Self {
            gestures:Vec::new(),
            // Pre-filled with real Points (not just capacity) so indexing
            // works right away and the same buffer can be reused.
            raw_points_buffer: vec![PointWrapper::default(); buffer_size],
            counter: 0,
            stroke_cnt: 0
        }
    }

    // Starts recording a new stroke for the given shape.
    pub fn start_gesture(&mut self) {
        self.stroke_cnt += 1;
    }

    // Adds an intermediate point to the current stroke.
    // Unlike `start_gesture`, this does NOT increment `stroke_cnt` or touch
    // the shapes/stroke bookkeeping — it just keeps recording points for
    // the stroke that is already in progress.
    pub fn add_stroke_point(&mut self, x: i32, y: i32) {
        self.add_raw_point(x, y);
    }   

    // Writes a raw point at the current buffer position.
    // Buffer is reused across gestures, so this indexes in place
    // instead of growing the vector.
     fn add_raw_point(&mut self, x: i32, y: i32) {
        if self.counter >= self.raw_points_buffer.len() {
            // Optional safety net: grow if the buffer was undersized.
            self.raw_points_buffer.push(PointWrapper::default());
        }

        self.raw_points_buffer[self.counter].set_x(x as f32);
        self.raw_points_buffer[self.counter].set_y(y as f32);
        self.raw_points_buffer[self.counter].set_stroke_id(self.stroke_cnt);
        self.counter += 1;
    }


    pub fn start_stroke(&mut self, x: i32, y: i32){
        self.add_raw_point(x, y);
    }

    
    pub fn end_stroke(&mut self){
        self.stroke_cnt +=1;
    }

    // Finalizes the current gesture, stores it globally, and resets
    // the write position so the same buffer can be reused for the next one.
    pub fn end_gesture(&mut self) {
        self.stroke_cnt = 0;
        self.counter = 0;
    }

    // Only the points actually written (0..counter) belong to this
    // gesture; the rest of the buffer may still hold stale data
    // from a previous recording.
    pub fn add_gesture(&mut self, name: &str) {
        if self.counter == 0{
            self.counter +=1;
            self.raw_points_buffer.push(PointWrapper::default());
        }
        let points = self.raw_points_buffer[..self.counter-1].to_vec();
        let mut gesture = GestureWrapper::new(points, name);
        gesture.set_points_raw(Vec::new()); // release the memory allocated to store the raw points
        self.gestures.push(gesture.inner());
    }

  
    pub fn take_gesture(&self, name:&str)->GestureWrapper{
        let points = self.raw_points_buffer[..self.counter-1].to_vec();
        GestureWrapper::new(points, name)
    }

    
    pub fn match_shape(&self, shape:GestureWrapper)->Option<String>{
    let q_parameters = QParameters::default();
    let gesture = shape.inner();
    let n = classify(&gesture, &self.gestures, &q_parameters);
        if (n.distance as i32) > 27{return None;}
        Some(n.class)  
    }

}
