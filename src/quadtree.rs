use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use macroquad::color::{Color, DARKGRAY, WHITE};
use macroquad::shapes::draw_line;
use macroquad::text::draw_text;
use crate::{QuadObject, Rectangle};

const MAX_OBJECTS_PER_NODE: usize = 4;
const MAX_LEAF_DEPTH: i32 = 10;
const LINE_WIDTH: f32 = 1.0;

const QUAD_LINES_COLOR: Color = WHITE;

// --------------------
// Object bounds to grid coordinates
// --------------------
fn assign_object_to_grid(surface: &TreeSurface, object: &Rc<RefCell<dyn QuadObject>>) -> Vec<i32> {
    // Define split points
    let (mx, my) = surface.mxy();

    // Result vec
    let mut result_vec = Vec::new();

    let object = object.as_ref().borrow();

    if object.is_overlap(&TreeSurface::from_size(surface.x0, surface.y0, mx - 1, my - 1)) {
        result_vec.push(0) }
    if object.is_overlap(&TreeSurface::from_size(mx, surface.y0, surface.x1, my - 1)) {
        result_vec.push(1) }
    if object.is_overlap(&TreeSurface::from_size(surface.x0, my, mx - 1, surface.y1)) {
        result_vec.push(2) }
    if object.is_overlap(&TreeSurface::from_size(mx, my, surface.x1, surface.y1)) {
        result_vec.push(3) }

    result_vec
}

// --------------------
// QuadTree
// --------------------
pub struct QuadTree {
    top_node: Box<TreeNode>,

    surface: TreeSurface,
}

impl Display for QuadTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quadtree: {}", self.top_node)
    }
}
impl QuadTree {
    pub fn new(x0: i32, y0: i32, width: i32, height: i32) -> QuadTree {
        let surface = TreeSurface { x0, y0, x1:(x0+width), y1:(y0+height) };

        QuadTree {
            top_node: Box::new(TreeNode::new(1, surface.x0, surface.y0, surface.x1, surface.y1)),
            surface,
        }
    }

    pub fn clear(&mut self) {
        self.top_node.clear();
        self.top_node = Box::new(TreeNode::new(1, self.surface.x0, self.surface.y0, self.surface.x1, self.surface.y1));
    }

    pub fn insert_object(&mut self, object: Rc<RefCell<dyn QuadObject>>) {
        self.top_node.insert_object(Rc::clone(&object));
    }
}


// --------------------
// TreeSurface
// --------------------
pub struct TreeSurface {
    pub x0: i32, pub y0: i32, pub x1: i32, pub y1: i32, // Defining topleft with o and bottomright with i
}

impl TreeSurface {
    pub fn from_size(x0: i32, y0: i32, x1: i32, y1: i32) -> TreeSurface {
        TreeSurface { x0, y0, x1, y1}
    }
    pub fn mx(&self) -> i32 {
        ((self.x1 - self.x0) / 2) + self.x0
    }
    pub fn my(&self) -> i32 {
        ((self.y1 - self.y0) / 2) + self.y0
    }
    pub fn mxy(&self) -> (i32, i32) {
        (self.mx(), self.my())
    }
}

impl Display for TreeSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Surface: x0={}, y0={}, x1={}, y1={}", self.x0, self.y0, self.x1, self.y1)
    }
}

// --------------------
// TreeNode
// --------------------
struct TreeNode {
    depth: i32,
    surface: TreeSurface,

    // Either objects or leaves have no items. We use Option<T> in that case
    objects: Option<Vec<Rc<RefCell<dyn QuadObject>>>>, // Holds a maximum of MAX_OBJECTS_PER_NODE objects in each TreeNode
    leaves: [Option<Box<TreeNode>>; 4], // Children nodes, max 4
}
impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.objects.is_none() { // Check if objectvector is not None
            let objects_len = self.objects.as_ref().unwrap().len();
            write!(f, "    Objects:|{:?}", objects_len)
        } else {
            write!(f, "    Child1:|{}\nChild2:|{}\nChild3:|{}\nChild4:|{}\n", self.leaves[0].as_ref().unwrap(), self.leaves[1].as_ref().unwrap(), self.leaves[2].as_ref().unwrap(), self.leaves[3].as_ref().unwrap())
        }
    }
}
impl TreeNode {
    pub fn new(depth: i32, ox: i32, oy: i32, ix: i32, iy: i32) -> TreeNode {
        let surface = TreeSurface { x0: ox, y0: oy, x1: ix, y1: iy };
        TreeNode {
            depth,
            surface,
            objects: Some(Vec::new()),
            leaves: [None, None, None, None],
        }
    }

    pub fn clear(&mut self) {
        if self.objects.is_some() { // Check if objectvector is not None
            self.objects.as_mut().unwrap().clear();
            return;
        }
        self.leaves.iter_mut().for_each(|leaf| {
            let leaf: &mut Box<TreeNode> = leaf.as_mut().unwrap();
            leaf.clear();
        });
        for i in 1..3 {
            self.leaves[i] = None;
        }
    }

    pub fn insert_object(&mut self, object: Rc<RefCell<dyn QuadObject>>) {
        if self.objects.is_some() { // Check if objectvector is Some

            // First check if max object count has been reached
            if self.objects.as_ref().unwrap().len() == MAX_OBJECTS_PER_NODE {

                // Check if the current size is even valid
                let valid_to_split = (self.surface.x0 - self.surface.x1).abs() > 2 && (self.surface.y0 - self.surface.y1).abs() > 2;
                // Check if max depth has been reached
                let under_max_depth = self.depth < MAX_LEAF_DEPTH;

                // If neither guards have been activated
                if valid_to_split && under_max_depth {
                    self.switch_object_to_leaves(object);
                    return;
                }
            }

            // Else push object
            self.objects.as_mut().unwrap().push(object);


        } else { // We are using the leaves:
            let grid_index = assign_object_to_grid(&self.surface, &object);

            for value in grid_index {
                match value {
                    0 => {
                        let leaf = self.leaves[0].as_mut().unwrap();
                        leaf.insert_object(Rc::clone(&object));},
                    1 => {
                        let leaf = self.leaves[1].as_mut().unwrap();
                        leaf.insert_object(Rc::clone(&object));},
                    2 => {
                        let leaf = self.leaves[2].as_mut().unwrap();
                        leaf.insert_object(Rc::clone(&object));},
                    3 => {
                        let leaf = self.leaves[3].as_mut().unwrap();
                        leaf.insert_object(Rc::clone(&object));},
                    _ => (),
                }
            }
        }
    }

    // Internal
    fn switch_object_to_leaves(&mut self, extra_object: Rc<RefCell<dyn QuadObject>>) {
        // Populating leaves
        let (mx, my) = self.surface.mxy();

        self.leaves[0] = Some(Box::new(TreeNode::new(self.depth + 1,self.surface.x0, self.surface.y0, mx - 1, my - 1)));
        self.leaves[1] = Some(Box::new(TreeNode::new(self.depth + 1,mx, self.surface.y0, self.surface.x1, my - 1)));
        self.leaves[2] = Some(Box::new(TreeNode::new(self.depth + 1,self.surface.x0, my, mx - 1, self.surface.y1)));
        self.leaves[3] = Some(Box::new(TreeNode::new(self.depth + 1,mx, my, self.surface.x1, self.surface.y1)));

        // Add extra object
        self.objects.as_mut().unwrap().push(extra_object);

        // Loop through all object (including the extra)
        for object in self.objects.as_ref().unwrap() {
            let grid_index = assign_object_to_grid(&self.surface, &object);

            let mut index = 0;
            self.leaves.iter_mut().for_each(|leaf| {
                let leaf: &mut Box<TreeNode> = leaf.as_mut().unwrap();

                if grid_index.iter().any(|&x| (x as usize) == index) {
                    leaf.insert_object(Rc::clone(&object));
                }
                index += 1;
            });
        }
        self.objects.as_mut().unwrap().clear();
        self.objects = None;
    }
}

// ----------------------------------------
// Stats Functions
// ----------------------------------------
impl QuadTree {
    pub fn node_count(&self) -> i32 {
        self.top_node.node_count()
    }

    pub fn deepest_node(&self) -> i32 {
        self.top_node.deepest_node()
    }

    pub fn object_count(&self) -> i32 {
        self.top_node.object_count()
    }
}

impl TreeNode {
    pub fn node_count(&self) -> i32 {
        return if !self.objects.is_none() { // Check if objectvector is not None
            1
        } else {
            let sum =
            self.leaves.iter().map(|leaf| {
                let node_count = leaf.as_ref().unwrap().node_count();
                return node_count
            }).sum::<i32>();
            sum + 1
        }
    }

    pub fn deepest_node(&self) -> i32 {
        if !self.objects.is_none() { // Check if objectvector is not None
            self.depth
        } else {
            self.leaves.iter().map(|leaf| {
                let node_count = leaf.as_ref().unwrap().deepest_node();
                node_count
            }).max().unwrap()
        }
    }

    pub fn object_count(&self) -> i32 {
        if !self.objects.is_none() { // Check if objectvector is not None
            self.objects.as_ref().unwrap().len() as i32
        } else {
            self.leaves.iter().map(|leaf| {
                let node_count = leaf.as_ref().unwrap().object_count();
                node_count
            }).sum()
        }
    }

    pub fn leaf_count(&self) -> i32 {
        if self.objects.is_some() { // Check if objectvector is not None
            0
        } else {
            self.leaves.iter().map(|leaf| {
                let node_count = leaf.as_ref().unwrap().leaf_count();
                node_count
            }).sum::<i32>() + 4
        }
    }

    pub fn max_objects(&self) -> i32 {
        if self.objects.is_some() { // Check if objectvector is not None
            self.object_count()
        } else {
            self.leaves.iter().map(|leaf| {
                let max_object_count = leaf.as_ref().unwrap().max_objects();
                max_object_count
            }).max().unwrap()
        }
    }
}

// ----------------------------------------
// Complex methods
// ----------------------------------------
impl QuadTree {
    pub fn query_objects_in(&self, query_surface: &Rectangle) -> Vec<Rc<RefCell<dyn QuadObject>>> {
        self.top_node.query_surface(query_surface)
    }
}
impl TreeNode {
    pub fn query_surface(&self, query_surface: &Rectangle) -> Vec<Rc<RefCell<dyn QuadObject>>> {
        let mut query_result = vec![];

        if !self.objects.is_none() { // Check if objectvector is not None
            for object in self.objects.as_ref().unwrap().iter() {
                if query_surface.is_rect_overlap(object.borrow()) { query_result.push(Rc::clone(object)) }
            }
        } else {
            self.leaves.iter().map(|leaf| {
                query_result.append(leaf.as_ref().unwrap().query_surface(&query_surface).as_mut());
            }).collect()
        }
        query_result
    }
}

// ----------------------------------------
// Draw Functions
// ----------------------------------------
impl QuadTree {
    pub fn draw(&self) {
        // Borders
        draw_line(self.surface.x0 as f32, self.surface.y0 as f32, self.surface.x1 as f32, self.surface.y0 as f32, LINE_WIDTH, DARKGRAY);
        draw_line(self.surface.x0 as f32, self.surface.y1 as f32, self.surface.x1 as f32, self.surface.y1 as f32, LINE_WIDTH, DARKGRAY);
        draw_line(self.surface.x0 as f32, self.surface.y0 as f32, self.surface.x0 as f32, self.surface.y1 as f32, LINE_WIDTH, DARKGRAY);
        draw_line(self.surface.x1 as f32, self.surface.y0 as f32, self.surface.x1 as f32, self.surface.y1 as f32, LINE_WIDTH, DARKGRAY);

        // Children
        self.top_node.draw();

        // Text
        let draw_x = self.surface.x1 as f32 + 5.0;

        let mut info_str = String::from("Node count:");
        info_str.push_str(&self.node_count().to_string());
        draw_text(info_str.as_str(), draw_x, 40.0, 15.0, WHITE);
        info_str.clear();

        info_str.push_str("Deepest node:");
        info_str.push_str(&self.deepest_node().to_string());
        draw_text(info_str.as_str(), draw_x, 60.0, 15.0, WHITE);
        info_str.clear();

        info_str.push_str("Objects in nodes:");
        info_str.push_str(&self.object_count().to_string());
        draw_text(info_str.as_str(), draw_x, 80.0, 15.0, WHITE);
        info_str.clear();

        info_str.push_str("Allocated leafs:");
        info_str.push_str(&self.top_node.leaf_count().to_string());
        draw_text(info_str.as_str(), draw_x, 100.0, 15.0, WHITE);
        info_str.clear();

        info_str.push_str("Max objects in leaf:");
        info_str.push_str(&self.top_node.max_objects().to_string());
        draw_text(info_str.as_str(), draw_x, 120.0, 15.0, WHITE);
        info_str.clear();
    }
}

impl TreeNode {
    pub fn draw(&self) {
        // Borders
        draw_line(self.surface.x0 as f32, self.surface.y0 as f32, self.surface.x1 as f32, self.surface.y0 as f32, LINE_WIDTH, QUAD_LINES_COLOR);
        draw_line(self.surface.x0 as f32, self.surface.y1 as f32, self.surface.x1 as f32, self.surface.y1 as f32, LINE_WIDTH, QUAD_LINES_COLOR);
        draw_line(self.surface.x0 as f32, self.surface.y0 as f32, self.surface.x0 as f32, self.surface.y1 as f32, LINE_WIDTH, QUAD_LINES_COLOR);
        draw_line(self.surface.x1 as f32, self.surface.y0 as f32, self.surface.x1 as f32, self.surface.y1 as f32, LINE_WIDTH, QUAD_LINES_COLOR);

        // Children
        if self.objects.is_none() {
            self.leaves.iter().for_each(|leaf| {
                let leaf: &Box<TreeNode> = leaf.as_ref().unwrap();
                leaf.draw();
            })
        } else {
            let count  = self.objects.as_ref().unwrap().len().to_string();
            draw_text(count.as_str(), self.surface.x0 as f32 + 2.0, self.surface.y0 as f32 + 10.0, 15.0, WHITE);
        }
    }
}