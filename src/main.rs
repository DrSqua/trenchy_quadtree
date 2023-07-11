use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use macroquad::input::is_key_down;
use macroquad::prelude::{Conf, KeyCode};
use macroquad::window::{next_frame};
use crate::graphical::{draw, draw_performance, TimingStruct};
use crate::main_loop::{handle_input, InputStore, setup_shapes, update};
use crate::quad_objects::{QuadObject, Rectangle};
use crate::quadtree::QuadTree;

mod quadtree;
mod quad_objects;
mod main_loop;
mod graphical;

fn window_conf() -> Conf {
    Conf {
        window_title: "QuadTree".to_owned(),
        window_width: 750,
        window_height: 550,
        ..Default::default()
    }
}

// --------------------
// Main Loop
// --------------------
#[macroquad::main(window_conf)]
async fn main() {
    // Input to update setup
    let input_control = &mut InputStore{ is_selection:false, selected: None, selected_objects: None, do_quadtree:true };

    // Simulation setup
    let mut run_simulation = true;
    let object_array: &mut Vec<Rc<RefCell<dyn QuadObject>>> = &mut setup_shapes();
    let mut quadtree = QuadTree::new(25, 25, 500, 500);

    // Loop
    while run_simulation {
        let mut time_struct = TimingStruct {start:Instant::now(), after_handle_input:Instant::now(), after_update:Instant::now(), after_draw:Instant::now(), after_object_update:Instant::now(), after_query_by_object:Instant::now(), after_quadtree:Instant::now() };

        // Handle_Input
        handle_input(input_control, object_array);
        if is_key_down(KeyCode::Escape) { run_simulation = false }
        time_struct.after_handle_input = Instant::now();

        // Update
        update(time_struct.borrow_mut(), input_control, object_array, quadtree.borrow_mut());
        time_struct.after_update = Instant::now();

        // Draw
        draw(input_control, object_array, quadtree.borrow_mut());
        time_struct.after_draw = Instant::now();

        draw_performance(&time_struct, quadtree.borrow());

        next_frame().await
    }
}
