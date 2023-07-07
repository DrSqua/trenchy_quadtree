use std::any::{Any, TypeId};
use std::cmp::{max, min};
use std::fmt;
use std::fmt::{Display, Formatter, Pointer};
use std::ops::Deref;
use std::rc::Rc;
use macroquad::color::{DARKBLUE, RED, YELLOW};
use macroquad::math::Vec2;
use macroquad::prelude::draw_circle_lines;
use macroquad::shapes::{draw_rectangle_lines, draw_triangle, draw_triangle_lines};
use crate::KeyCode::V;

use crate::quadtree::TreeSurface;

//
// QuadObject Trait
//
pub trait QuadObject: Display {
    fn draw(&self);
    fn highlight(&self);
    fn center(&self) -> (i32, i32);
    fn is_overlap(&self, surface: &TreeSurface) -> bool;
}

// -
// Objects
// -

// Boid
pub struct Boid {
    x: i32,
    y: i32,
    facing: f32,
}

impl Boid {
    pub fn new(x: i32, y: i32, facing: f32) -> Boid {
        Boid { x, y, facing }
    }
}
impl QuadObject for Boid {
    fn draw(&self) {
        // TODO let source = Vec2 { x:(self.x as f32 + sin(self.facing)), y:(self.y as f32 + self.facing)};
        let p1 = Vec2 { x:(self.x as f32), y:(self.y as f32 - 4.0)};
        let p2 = Vec2 { x:(self.x as f32 + 4.0), y:(self.y as f32 + 4.0)};
        let p3 = Vec2 { x:(self.x as f32 - 4.0), y:(self.y as f32 + 4.0)};

        // draw_triangle(source, p1, p2, DARKBLUE);
        draw_triangle_lines(p1, p2, p3, 2.0, DARKBLUE)
    }

    fn highlight(&self) {
        // TODO let source = Vec2 { x:(self.x as f32 + sin(self.facing)), y:(self.y as f32 + self.facing)};
        let p1 = Vec2 { x:(self.x as f32), y:(self.y as f32 - 4.0)};
        let p2 = Vec2 { x:(self.x as f32 + 4.0), y:(self.y as f32 + 4.0)};
        let p3 = Vec2 { x:(self.x as f32 - 4.0), y:(self.y as f32 + 4.0)};

        // draw_triangle(source, p1, p2, DARKBLUE);
        draw_triangle_lines(p1, p2, p3, 2.0, YELLOW)
    }

    fn center(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn is_overlap(&self, surface: &TreeSurface) -> bool {
        let (mx, my) = self.center();
        surface.x0 <= mx && mx < surface.x1 && surface.y0 <= my && my < surface.y1
    }
}

impl Display for Boid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Boid!")
    }
}

// Rectangle
pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle { x, y, width, height }
    }
    pub fn to_tree_surface(&self) -> TreeSurface {
        TreeSurface { x0:self.x, x1:self.x+self.width, y0:self.y, y1:self.y+self.height }
    }

    pub fn is_rect_overlap(&self, object: &Rc<dyn QuadObject>) -> bool {
        let surface = self.to_tree_surface();
        object.is_overlap(&surface)
    }
    pub fn get_wh(&self) -> (i32, i32) {
        (self.width, self.height)
    }
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }
    pub fn get_source(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
impl QuadObject for Rectangle {
    fn draw(&self) {
        draw_rectangle_lines(self.x as f32, self.y as f32, self.width as f32, self.height as f32, 1.0, RED);
    }

    fn highlight(&self) {
        draw_rectangle_lines(self.x as f32, self.y as f32, self.width as f32, self.height as f32, 1.0, YELLOW);
    }

    fn center(&self) -> (i32, i32) { (((self.width) / 2) + self.x, ((self.height) / 2) + self.y) }

    fn is_overlap(&self, surface: &TreeSurface) -> bool {
        if self.x < surface.x1 &&
            self.x + self.width > surface.x0 &&
            self.y < surface.y1 &&
            self.y + self.height > surface.y0 {
            true
        } else {
            false
        }
    }
}
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RECT")
    }
}
// Circle
pub struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}
impl Circle {
    pub fn new(x: i32, y: i32, r: i32) -> Circle {
        Circle { x, y, radius:r }
    }
}
impl QuadObject for Circle {
    fn draw(&self) {
        draw_circle_lines(self.x as f32, self.y as f32, self.radius as f32, 1.0, RED);
    }

    fn highlight(&self) {
        draw_circle_lines(self.x as f32, self.y as f32, self.radius as f32, 1.0, YELLOW);
    }

    fn center(&self) -> (i32, i32) { (self.x, self.y) }

    fn is_overlap(&self, surface: &TreeSurface) -> bool {
        let xn = max(surface.x0, min(self.x, surface.x1));
        let yn = max(surface.y0, min(self.y, surface.y1));

        let dx = xn - self.x;
        let dy = yn - self.y;

        (dx.pow(2) + dy.pow(2)) <= self.radius.pow(2)
    }
}
impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle")
    }
}