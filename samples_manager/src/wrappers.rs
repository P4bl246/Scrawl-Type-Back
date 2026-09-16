use q_recognizer::gesture::*;
use q_recognizer::point::*;
use wasm_bindgen::prelude::*;

// --- Trait: acceso explícito al Point interno ---
pub trait InnerPoint {
    /// Devuelve un Rc clonado apuntando al mismo Point interno
    /// (comparte identidad, no copia el valor).
    fn inner(self) -> Point;
    fn set_x(&mut self, new:f32);
    fn set_y(&mut self, new:f32);
    fn set_stroke_id(&mut self, new:i32);
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct PointWrapper {
    inner: Point,
}

impl InnerPoint for PointWrapper {
    fn inner(self) -> Point {self.inner}
    fn set_x(&mut self, new:f32){self.inner.x=new;}
    fn set_y(&mut self, new:f32){self.inner.y=new;}
    fn set_stroke_id(&mut self, new:i32){self.inner.stroke_id=new;}
}

impl Default for PointWrapper{
    fn default()->Self{
        Self{inner:Point::new(0.0,0.0,0)}
    }
}

#[wasm_bindgen]
impl PointWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, stroke_id: i32) -> Self {
        Self { inner: Point::new(x, y, stroke_id)}
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 { self.inner.x }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 { self.inner.y }

    #[wasm_bindgen(getter, js_name = strokeId)]
    pub fn stroke_id(&self) -> i32 { self.inner.stroke_id }
}


pub trait InnerGesture {
    /// Devuelve un Rc clonado apuntando al mismo Point interno
    /// (comparte identidad, no copia el valor).
    fn inner(self) -> Gesture;
    
    fn set_points_raw(&mut self, new: Vec<Point>);
    /* 
    fn set_points(&mut self, new: Vec<Point>);
    fn set_name(&mut self, new: String);
    fn set_lut(&mut self, new: Option<Vec<Vec<usize>>>);
    */
}

#[wasm_bindgen]
pub struct GestureWrapper {
    inner: Gesture,
}

impl InnerGesture for GestureWrapper{
    fn inner(self)->Gesture{self.inner}
    
    fn set_points_raw(&mut self, new: Vec<Point>){self.inner.points_raw = new;}
    /* 
    fn set_points(&mut self, new: Vec<Point>){self.inner.points = new;}
    fn set_name(&mut self, new: String){self.inner.name = new;}
    fn set_lut(&mut self, new: Option<Vec<Vec<usize>>>){self.inner.lut = new;}
    */
}

#[wasm_bindgen]
impl GestureWrapper{

    pub fn new(wrapped_points: Vec<PointWrapper>, name: &str) -> Self {

        let points: Vec<Point> = wrapped_points.into_iter().map(|p| p.inner()).collect();
        
        Self {
            inner: Gesture::new(points, name),
        }
    }
}
/*
// --- Trait: acceso interno, SOLO para Rust ---
pub trait InnerGestures {
    /// Referencia al Vec<Gesture> interno — sin clonar, sin exponer a JS.
    fn inner_refs(&self) -> &Vec<Gesture>;
}

#[wasm_bindgen]
pub struct GesturesWrapper {
    inner: Rc<Vec<Gesture>>,
}

impl InnerGestures for GesturesWrapper {
    fn inner_refs(&self) -> &Vec<Gesture> {
        &self.inner
    }
}

#[wasm_bindgen]
impl GesturesWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(wrapped: Vec<GestureWrapper>) -> Self {
        let gestures: Vec<Gesture> = wrapped.into_iter().map(|g| g.inner()).collect();
        Self { inner: Rc::new(gestures) }
    }

    // Lo mínimo que JS necesita para saber que "tiene algo" — opcional
    #[wasm_bindgen(getter)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Otro handle compartiendo el MISMO Vec<Gesture> (barato, sin copiar datos)
    pub fn share(&self) -> Self {
        Self { inner: Rc::clone(&self.inner) }
    }
}
*/