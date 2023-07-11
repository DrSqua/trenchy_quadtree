use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use macroquad::input::{is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, MouseButton};
use macroquad::prelude::{KeyCode};
use macroquad::rand::ChooseRandom;

use crate::quad_objects::{QuadObject, Rectangle, Circle, Boid};
use crate::quadtree::QuadTree;
use rand::{Rng, thread_rng};

pub fn setup_shapes() -> Vec<Rc<RefCell<dyn QuadObject>>> {
    let mut input_vec: Vec<Rc<RefCell<dyn QuadObject>>> = vec![];

    // Red one :o
    input_vec.push(Rc::new(RefCell::new(Boid::new_red((input_vec.len() as u32),200, 200, 2.0))));


    // First
    input_vec.push(Rc::new(RefCell::new(Rectangle::new((input_vec.len() as u32),10, 10, 50, 50))));
    // Second
    input_vec.push(Rc::new(RefCell::new(Circle::new((input_vec.len() as u32),320, 100, 20))));
    // Third
    input_vec.push(Rc::new(RefCell::new(Circle::new((input_vec.len() as u32),120, 300, 20))));
    // Fourth
    input_vec.push(Rc::new(RefCell::new(Circle::new((input_vec.len() as u32),300, 300, 10))));
    input_vec.push(Rc::new(RefCell::new(Circle::new((input_vec.len() as u32),320, 300, 20))));
    input_vec.push(Rc::new(RefCell::new(Circle::new((input_vec.len() as u32),200, 200, 40))));
    input_vec.push(Rc::new(RefCell::new(Rectangle::new((input_vec.len() as u32),390, 390, 20, 20))));
    input_vec.push(Rc::new(RefCell::new(Rectangle::new((input_vec.len() as u32),450, 450, 40, 40))));
    input_vec.push(Rc::new(RefCell::new(Circle::new((input_vec.len() as u32),300, 300, 3))));
    // Bottom cluster
    input_vec.push(Rc::new(RefCell::new(Circle::new((input_vec.len() as u32),300, 400, 10))));

    // Boids
    let nums: Vec<i32> = (1..40).collect();
    let pos_iter = nums.iter().zip( nums.iter().rev() );
    for (x, y) in pos_iter {
        input_vec.push(Rc::new(RefCell::new(Boid::new((input_vec.len() as u32),*x * 10 + 100, *y * 10 + 100, 0.0))));
    }

    // Return
    input_vec
}

pub struct InputStore {
    pub is_selection: bool,
    pub selected: Option<Rectangle>,
    pub selected_objects: Option<Vec<Rc<RefCell<dyn QuadObject>>>>,

    pub do_quadtree: bool,
}

// --------------------
// Handle Input
// --------------------
pub fn handle_input(input_store: &mut InputStore, object_array: &mut Vec<Rc<RefCell<dyn QuadObject>>>) {
    // Toggle quadtree
    if is_key_pressed(KeyCode::Q) {
        input_store.do_quadtree = false;
    }

    // Add object
    if is_mouse_button_pressed(MouseButton::Right) {
        let mut rng = thread_rng();

        let (mx, my) = mouse_position();
        object_array.push(Rc::new(RefCell::new(Boid::new((object_array.len() as u32),mx as i32, my as i32, rng.gen_range(0.0..6.0) as f32))));
    }
    // Add 100
    if is_key_pressed(KeyCode::Up) {
        let mut rng = thread_rng();
        let mut nums: Vec<i32> = (1..100).collect();
        nums.shuffle();
        let pos_iter = nums.iter().zip( nums.iter().rev() );
        for (x, y) in pos_iter {
            object_array.push(Rc::new(RefCell::new(Boid::new((object_array.len() as u32),*x * 10 + 100, *y * 10 + 100, rng.gen_range(0.0..6.0) as f32))));
        }
    }
    if is_key_pressed(KeyCode::Down) {
        for _ in 1..100 {
            object_array.remove(object_array.len()-1);
        }
    }

    // Quadtree
    if !input_store.do_quadtree { return; }
    if is_mouse_button_down(MouseButton::Left) && input_store.is_selection {
        let rect = input_store.selected.as_mut().unwrap();
        let (mx, my) = mouse_position();
        rect.adjust_to_point(mx as i32, my as i32);
    }
    if is_mouse_button_released(MouseButton::Left) && input_store.is_selection {
        // Do selection
        input_store.is_selection = false;

        let rect = input_store.selected.as_mut().unwrap();
        rect.normalize();
    }
    if is_mouse_button_pressed(MouseButton::Left) && !input_store.is_selection {
        input_store.is_selection = true;
        let (x, y)  = mouse_position();
        let x = x as i32;
        let y = y as i32;
        input_store.selected = Some(Rectangle::new((object_array.len() as u32), x, y, 0, 0));
    }
}

// --------------------
// Update
// --------------------
pub fn update(input_store: &mut InputStore, object_array: &mut Vec<Rc<RefCell<dyn QuadObject>>>, quadtree: &mut QuadTree) {
    // Setup quadtree
    quadtree.clear();
    for object in object_array.iter() {
        quadtree.insert_object(Rc::clone(object));
        object.as_ref().borrow_mut().update();
    }
    // Operation
    for object in object_array.iter() {
        let query = quadtree.query_neighbours_and_condition(&object.clone(), Some(10));
        for query_object in query.iter() {
            query_object.as_ref().borrow_mut().update_movement(object);
        }
    }


    // Perform query
    match &input_store.selected {
        Some(_) => {
            let query = quadtree.query_surface(input_store.selected.as_ref().unwrap());
            input_store.selected_objects = Some(query);
        },
        None => {},
    }

}
