use std::cell::RefCell;
use std::rc::Rc;
use macroquad::text::draw_text;
use macroquad::time::get_fps;
use macroquad::window::{clear_background};
use macroquad::color::{BLACK, WHITE, YELLOW};
use std::time::Instant;
use crate::{InputStore, QuadObject, QuadTree};

pub struct TimingStruct {
    pub(crate) start: Instant,
    pub(crate) after_handle_input: Instant,
    pub(crate) after_update: Instant,
    pub(crate) after_draw: Instant,
}

//
pub fn draw_performance(time_struct: &TimingStruct, quadtree: &QuadTree) {
    // Performance
    let draw_x = quadtree.get_surface().x1 as f32 + 5.0;

    // Fps
    let mut info_str = String::from("FPS: ");
    let fps = get_fps();
    info_str.push_str(&fps.to_string());
    draw_text(info_str.as_str(), draw_x, 280.0, 15.0, WHITE);

    info_str.clear();
    info_str.push_str("Handle Input: ");
    info_str.push_str(&(time_struct.after_handle_input - time_struct.start).as_micros().to_string());
    draw_text(info_str.as_str(), draw_x, 300.0, 15.0, WHITE);

    info_str.clear();
    info_str.push_str("Update: ");
    info_str.push_str(&(time_struct.after_update - time_struct.after_handle_input).as_micros().to_string());
    draw_text(info_str.as_str(), draw_x, 320.0, 15.0, WHITE);

    info_str.clear();
    info_str.push_str("Draw: ");
    info_str.push_str(&(time_struct.after_draw - time_struct.after_update).as_micros().to_string());
    draw_text(info_str.as_str(), draw_x, 340.0, 15.0, WHITE);
}

// --------------------
// Draw
// --------------------
pub fn draw(input_store: &mut InputStore, object_array: &mut Vec<Rc<RefCell<dyn QuadObject>>>, quadtree: &mut QuadTree) {
    clear_background(BLACK);

    // Normal draws
    quadtree.draw();
    for object in object_array.iter() {
        object.borrow().draw();
    }

    // Highlight by red
    let red = object_array.first().unwrap();
    let query = quadtree.query_neighbours_and_condition(red, Some(10));
    for object in query.iter() {
        object.borrow().highlight();
    }

    // Highlight rect
    match &input_store.selected {
        Some(rect) => {
            rect.highlight(); },
        None => {},
    }
    // Highlight selected object by rect
    match &input_store.selected_objects {
        Some(objects) => {
            // Text
            let mut info_str = String::from("Node count:");
            info_str.push_str(&objects.len().to_string());
            draw_text(info_str.as_str(), 25.0, 20.0, 15.0, YELLOW);

            // Objects
            for object in objects {
                object.borrow().highlight();
            }
        }
        None => {}
    }
    // Object vector size
    let mut info_str = String::from("Object vect: ");
    let len = &object_array.len().to_string();
    info_str.push_str(len);
    draw_text(info_str.as_str(), 120.0, 20.0, 15.0, WHITE);
}