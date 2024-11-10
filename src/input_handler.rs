use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Mutex};

use wasm_bindgen::prelude::Closure;
use web_sys::{KeyboardEvent, MouseEvent};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug)]
pub enum Key {
    Forward,
    Backward,
    Left,
    Right,
}



pub struct InputHandler {
    mouse_x: f32,
    mouse_y: f32,
    pub dx: f32,
    pub dy: f32,
    last_mouse_x: f32,
    last_mouse_y: f32,

    pub keys: Vec<bool>,
}

impl InputHandler {

    pub fn new() -> Rc<RefCell<Self>> {

        Self::init_listeners()

        //let s = Rc::new(Ref::new(s));

        //s.borrow_mut().init_listeners();
        
        
    }

    fn init_listeners() -> Rc<RefCell<Self>> {
        use wasm_bindgen::JsCast;

        let s = Self {
            mouse_x: 0.0,
            mouse_y: 0.0,
            dx: 0.0,
            dy: 0.0,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
            keys: vec![false; 4],
        };
        let s = Rc::new(RefCell::new(s));

        let document = web_sys::window().unwrap().document().unwrap();

        {
            let s = Rc::clone(&s);
            let key_up_closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                let key: Key = match e.key().as_str() {
                    "w" => Key::Forward,
                    "s" => Key::Backward,
                    "a" => Key::Left,
                    "d" => Key::Right,
                    _ => return,
                };
                log(&format!("key up: {:?}", key));
                s.borrow_mut().key_up(key);
            }) as Box<dyn FnMut(KeyboardEvent)>);
            document.set_onkeyup(Some(key_up_closure.as_ref().unchecked_ref()));
            key_up_closure.forget();
        }

        {
            let s2 = Rc::clone(&s);
            let key_down_closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                let key: Key = match e.key().as_str() {
                    "w" => Key::Forward,
                    "s" => Key::Backward,
                    "a" => Key::Left,
                    "d" => Key::Right,
                    _ => return,
                };
                log(&format!("key down: {:?}", key));
                s2.borrow_mut().key_down(key);
            }) as Box<dyn FnMut(_)>);
            document.set_onkeydown(Some(key_down_closure.as_ref().unchecked_ref()));
            key_down_closure.forget();
        }

        {
            let s3 = Rc::clone(&s);
            let mouse_move_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
                let x = e.client_x() as f32;
                let y = e.client_y() as f32;
                s3.borrow_mut().mouse_move(x, y);
            }) as Box<dyn FnMut(_)>);
            document.set_onmousemove(Some(mouse_move_closure.as_ref().unchecked_ref()));
            mouse_move_closure.forget();
        }

        s
    }

    pub fn key_down(&mut self, key: Key) {
        self.keys[key as usize] = true;
    }

    pub fn key_up(&mut self, key: Key) {
        self.keys[key as usize] = false;
    }

    pub fn mouse_move(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.dx = x - self.last_mouse_x;
        self.dy = y - self.last_mouse_y;
        self.last_mouse_x = x;
        self.last_mouse_y = y;
    }

    pub fn get_mouse(&self) -> (f32, f32) {
        (self.mouse_x, self.mouse_y)
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        !self.keys[key as usize]
    }
}