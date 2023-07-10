use std::cmp::{max, min};
use std::f32::consts::PI;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::rc::Rc;
use macroquad::color::{BLUE, DARKBLUE, RED, YELLOW};
use macroquad::math::Vec2;
use macroquad::prelude::draw_circle_lines;
use macroquad::shapes::{draw_line, draw_rectangle_lines, draw_triangle_lines};

use crate::quadtree::TreeSurface;

//
// QuadObject Trait
//
pub trait QuadObject: Display {
    fn draw(&self);
    fn highlight(&self);
    fn center(&self) -> (i32, i32);
    fn is_overlap(&self, surface: &TreeSurface) -> bool;
    fn update(&mut self);
}

// -
// Objects
// -

// Boid
pub struct Boid {
    x: i32,
    y: i32,
    facing: f32,
    velocity: f32,
}

impl Boid {
    pub fn new(x: i32, y: i32, facing: f32) -> Boid {
        Boid { x, y, facing, velocity:1.0, }
    }
}

impl QuadObject for Boid {
    fn update(&mut self) {
        let (vx, vy) = (self.x as f32 + self.facing.sin(), self.y as f32 + self.facing.cos());
        self.x += vx as i32;
        self.y += vy as i32;
    }

    fn draw(&self) {
        let size: f32 = 4.0;

        let on_circle = Vec2   { x:(self.x as f32 + ( self.facing.sin() * 2.0*size)),     y:(self.y as f32 + (self.facing.cos() * 2.0*size))};
        let left_point = Vec2  { x:(self.x as f32 + ((self.facing + PI/2.0).sin() *size)), y:(self.y as f32 + ((self.facing + PI/2.0).cos() * size))};
        let right_point = Vec2 { x:(self.x as f32 + ((self.facing - PI/2.0).sin() *size)), y:(self.y as f32 + ((self.facing - PI/2.0).cos() * size))};

        draw_line(self.x as f32, self.y as f32, on_circle.x, on_circle.y, 1.0, BLUE);
        draw_triangle_lines(on_circle, left_point, right_point, 1.5, DARKBLUE);
    }

    fn highlight(&self) {
        let size: f32 = 4.0;

        let on_circle = Vec2   { x:(self.x as f32 + ( self.facing.sin() * 2.0*size)),     y:(self.y as f32 + (self.facing.cos() * 2.0*size))};
        let left_point = Vec2  { x:(self.x as f32 + ((self.facing + PI/2.0).sin() *size)), y:(self.y as f32 + ((self.facing + PI/2.0).cos() * size))};
        let right_point = Vec2 { x:(self.x as f32 + ((self.facing - PI/2.0).sin() *size)), y:(self.y as f32 + ((self.facing - PI/2.0).cos() * size))};

        draw_triangle_lines(on_circle, left_point, right_point, 1.5, YELLOW);
    }

    fn center(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn is_overlap(&self, surface: &TreeSurface) -> bool {
        let (mx, my) = self.center();
        surface.x0 <= mx && mx <= surface.x1 && surface.y0 <= my && my <= surface.y1
    }
}

impl Display for Boid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Boid!")
    }
}

// Rectangle
pub struct Rectangle {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}
impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {

        let mut x0 = x;
        let mut x1 = x + width;
        let mut y0 = y;
        let mut y1 = y + height;
        if width < 0 {
            swap(&mut x0, &mut x1);
        }
        if height < 0 {
            swap(&mut y0, &mut y1);
        }
        Rectangle { x0, y0, x1, y1, }
    }
    pub fn to_tree_surface(&self) -> TreeSurface {
        TreeSurface { x0:self.x0, y0:self.y0, x1:self.x1, y1:self.y1 }
    }

    pub fn is_rect_overlap(&self, object: &Rc<dyn QuadObject>) -> bool {
        let surface = self.to_tree_surface();
        object.is_overlap(&surface)
    }
    pub fn get_wh(&self) -> (i32, i32) {
        (self.x1 - self.x0, self.y1 - self.y0)
    }
    pub fn get_source(&self) -> (i32, i32) {
        (self.x0, self.y0)
    }
    pub fn adjust_to_point(&mut self, x: i32, y:i32) {
        self.x1 = x;
        self.y1 = y;
    }
    pub fn normalize(&mut self) {
        if self.x1 < self.x0 {
            swap(&mut self.x0, &mut self.x1);
        }
        if self.y1 < self.y0 {
            swap(&mut self.y0, &mut self.y1);
        }
    }
}
impl QuadObject for Rectangle {
    fn update(&mut self) {}

    fn draw(&self) {
        let (w, h) = self.get_wh();
        draw_rectangle_lines(self.x0 as f32, self.y0 as f32, w as f32, h as f32, 1.0, RED);
    }

    fn highlight(&self) {
        let (w, h) = self.get_wh();
        draw_rectangle_lines(self.x0 as f32, self.y0 as f32, w as f32, h as f32, 1.0, YELLOW);
    }

    fn center(&self) -> (i32, i32) {
        let (w, h) = self.get_wh();
        ((w / 2) + self.x0, (h / 2) + self.y0)
    }

    fn is_overlap(&self, surface: &TreeSurface) -> bool {
        if self.x0 < surface.x1 &&
            self.x1 > surface.x0 &&
            self.y0 < surface.y1 &&
            self.y1 > surface.y0 {
            true
        } else {
            false
        }
    }
}
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RECT: ({}, {}) -> ({}, {})", self.x0, self.y0, self.x1, self.y1)
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
    fn update(&mut self) {}

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