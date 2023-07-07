use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use macroquad::color::{BLACK, WHITE, YELLOW};
use macroquad::input::{is_key_down, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_delta_position, mouse_position, MouseButton};
use macroquad::prelude::{BLUE, Conf, KeyCode, rand};
use macroquad::text::draw_text;
use macroquad::window::{clear_background, next_frame};
use crate::quad_objects::{QuadObject, Rectangle, Circle, Boid};
use crate::quadtree::QuadTree;

mod quadtree;
mod quad_objects;


fn window_conf() -> Conf {
    Conf {
        window_title: "QuadTree".to_owned(),
        window_width: 550,
        window_height: 550,
        ..Default::default()
    }
}

fn setup_shapes() -> Vec<Rc<dyn QuadObject>> {
    let mut input_vec: Vec<Rc<dyn QuadObject>> = vec![];

    // First
    input_vec.push(Rc::new(Rectangle::new(10, 10, 50, 50)));
    // Second
    input_vec.push(Rc::new(Circle::new(320, 100, 20)));
    // Third
    input_vec.push(Rc::new(Circle::new(120, 300, 20)));
    // Fourth
    input_vec.push(Rc::new(Circle::new(300, 300, 10)));
    input_vec.push(Rc::new(Circle::new(320, 300, 20)));
    input_vec.push(Rc::new(Circle::new(200, 200, 40)));
    input_vec.push(Rc::new(Rectangle::new(390, 390, 20, 20)));
    input_vec.push(Rc::new(Rectangle::new(450, 450, 40, 40)));
    input_vec.push(Rc::new(Circle::new(300, 300, 3)));
        // Bottom cluster
        input_vec.push(Rc::new(Circle::new(300, 400, 10)));

    // Boids
    input_vec.push(Rc::new(Boid::new(200, 200, 2.0)));

    let mut nums: Vec<i32> = (1..40).collect();
    let pos_iter = nums.iter().zip( nums.iter().rev() );
    for (x, y) in pos_iter {
        input_vec.push(Rc::new(Boid::new(*x * 10 + 100, *y * 10 + 100, 0.0)));
    }

    // Return
    input_vec
}

struct InputStore {
    is_selection: bool,
    selected: Option<Rectangle>,
    selected_objects: Option<Vec<Rc<dyn QuadObject>>>,
}

// --------------------
// Handle Input
// --------------------
fn handle_input(input_store: &mut InputStore, object_array: &mut Vec<Rc<dyn QuadObject>>, quadtree: &QuadTree) {
    if is_mouse_button_pressed(MouseButton::Right) {
        let (mx, my) = mouse_position();
        object_array.push(Rc::new(Boid::new(mx as i32, my as i32, 2.0)));
    }

    if is_mouse_button_down(MouseButton::Left) && input_store.is_selection {
        let mut rect = input_store.selected.as_mut().unwrap();
        let (x, y) = rect.get_source();
        let (mx, my) = mouse_position();
        rect.set_width(mx as i32 - x);
        rect.set_height(my as i32 - y);
    }
    if is_mouse_button_released(MouseButton::Left) && input_store.is_selection {
        // Do selection
        input_store.is_selection = false;
        let query = quadtree.query_objects_in(input_store.selected.as_ref().unwrap());
        input_store.selected_objects = Some(query);
    }
    if is_mouse_button_pressed(MouseButton::Left) && !input_store.is_selection {
        input_store.is_selection = true;
        let (x, y)  = mouse_position();
        let x = x as i32;
        let y = y as i32;
        input_store.selected = Some(Rectangle::new(x, y, 0, 0));
    }
}

// --------------------
// Update
// --------------------
fn update(object_array: &mut Vec<Rc<dyn QuadObject>>, quadtree: &mut QuadTree) {
    quadtree.clear();
    for object in object_array.iter() {
        quadtree.insert_object(Rc::clone(&object));
    }
}

// --------------------
// Draw
// --------------------
fn draw(input_store: &mut InputStore, object_array: &mut Vec<Rc<dyn QuadObject>>, quadtree: &mut QuadTree) {
    clear_background(BLACK);

    quadtree.draw();
    for object in object_array {
        object.draw();
    }

    match &input_store.selected {
        Some(rect) => {
            rect.highlight(); },
        None => {},
    }
    match &input_store.selected_objects {
        Some(objects) => {
            // Text
            let mut info_str = String::from("Node count:");
            info_str.push_str(&objects.len().to_string());
            draw_text(info_str.as_str(), 25.0, 10.0, 15.0, YELLOW);

            // Objects
            for object in objects {
                object.highlight();
            }
        }
        None => {}
    }
}

// --------------------
// Main Loop
// --------------------
#[macroquad::main(window_conf)]
async fn main() {
    // Input to update setup
    let input_control = &mut InputStore{ is_selection:false, selected: None, selected_objects: None };

    // Simulation setup
    let mut run_simulation = true;
    let object_array: &mut Vec<Rc<dyn QuadObject>> = &mut setup_shapes();
    let mut quadtree = QuadTree::new(25, 25, 500, 500);

    // Loop
    while run_simulation {
        // Handle_Input
        handle_input(input_control, object_array, &quadtree);
        if is_key_down(KeyCode::Escape) { run_simulation = false }

        // Update
        update(object_array, quadtree.borrow_mut());

        // Draw
        draw(input_control, object_array, quadtree.borrow_mut());

        next_frame().await
    }
}