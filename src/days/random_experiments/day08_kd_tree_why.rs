use super::Day;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Day08;
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}
pub enum KdNode {
    Split(SplitNode),
    Leaf(LeafNode),
}

#[derive(Debug, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub struct SplitNode {
    pub axis: Axis,
    pub position: i64,
    pub parent: Weak<RefCell<KdNode>>,
    pub left: Rc<RefCell<KdNode>>,
    pub right: Rc<RefCell<KdNode>>,
}

pub struct LeafNode {
    pub parent: Weak<RefCell<KdNode>>,
    pub points: Vec<Point>,
}

pub struct KdTree {
    pub root: Rc<RefCell<KdNode>>
}

impl KdTree {
    fn build_tree(points: Vec<Point>, depth: i32, leaf_size: usize, parent: Weak<RefCell<KdNode>>) -> Rc<RefCell<KdNode>> {
        if points.len() <= leaf_size {
            return Rc::new(RefCell::new(KdNode::Leaf(LeafNode { parent: parent.clone(), points })));
        }

        let (axis, position) = Self::find_best_split_plane(&points, depth);
        let (left_points, right_points): (Vec<Point>, Vec<Point>) = points.into_iter().partition(|p| {
            match axis {
                Axis::X => p.x <= position,
                Axis::Y => p.y <= position,
                Axis::Z => p.z <= position,
            }
        });

        // Create the split node without children first
        let split = Rc::new(RefCell::new(KdNode::Split(SplitNode {
            axis: axis.clone(),
            position,
            parent: parent.clone(),
            left: Rc::new(RefCell::new(KdNode::Leaf(LeafNode { parent: parent.clone(), points: vec![] }))),
            right: Rc::new(RefCell::new(KdNode::Leaf(LeafNode { parent: parent.clone(), points: vec![] }))),
        })));
        // Now build children, passing split as parent
        if let KdNode::Split(ref mut split_node) = *split.borrow_mut() {
            split_node.left = Self::build_tree(left_points, depth + 1, leaf_size, Rc::downgrade(&split));
            split_node.right = Self::build_tree(right_points, depth + 1, leaf_size, Rc::downgrade(&split));
        }

        split
    }

    fn find_best_split_plane(points: &Vec<Point>, depth: i32) -> (Axis, i64) {
        // Determine the axis based on the depth
        let axis = match depth % 3 {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        };

        // Get all sorted candidates for the current axis
        let mut candidate_positions: Vec<i64> = points.iter().map(|p| match axis {
            Axis::X => p.x,
            Axis::Y => p.y,
            Axis::Z => p.z,
        }).collect();
        candidate_positions.sort_unstable();

        let mut best_diff = i64::MAX;
        let mut best_plane: (Axis, i64) = (Axis::X, 0);
        let mut left_count = 0i64;
        let mut right_count = points.len() as i64;

        for i in 0..candidate_positions.len() {
            let plane_position = candidate_positions[i];

            // Dont conside duplicates
            if i > 0 && plane_position == candidate_positions[i - 1] {
                continue;
            }

            // Left count gets the number of points added at the current sweep position
            left_count += points.iter().filter(|p| match axis {
                Axis::X => p.x == plane_position,
                Axis::Y => p.y == plane_position,
                Axis::Z => p.z == plane_position,
            }).count() as i64;

            // And right count gets them removed
            right_count -= points.iter().filter(|p| match axis {
                Axis::X => p.x == plane_position,
                Axis::Y => p.y == plane_position,
                Axis::Z => p.z == plane_position,
            }).count() as i64;

            // Calculate the difference
            let diff = (left_count - right_count).abs();
            if diff < best_diff {
                best_diff = diff;
                best_plane = (axis.clone(), plane_position);
            }
        }

        best_plane
    }

    fn nearest_neighbor_search(&self, target: &Point, excludes: &HashSet<Point>) -> Option<Point> {
        let mut best_point: Option<Point> = None;
        let mut best_distance = f64::MAX;

        // Recursive Search
        fn search(
            node: Rc<RefCell<KdNode>>,
            target: &Point,
            excludes: &HashSet<Point>,
            best_point: &mut Option<Point>,
            best_distance: &mut f64,
        ) {
            // Current node processing
            match &*node.borrow() {
                // If is Leaf then we just check all points
                KdNode::Leaf(leaf) => {
                    for point in &leaf.points {
                        if excludes.contains(point) {
                            continue;
                        }
                        let distance = ((point.x - target.x).pow(2)
                            + (point.y - target.y).pow(2)
                            + (point.z - target.z).pow(2)) as f64;
                        if distance < *best_distance {
                            *best_distance = distance;
                            *best_point = Some(*point);
                        }
                    }
                }
                // If is Split then we decide based on the axis and position where to recursively search.
                KdNode::Split(split) => {
                    let (next_node, other_node) = match split.axis {
                        Axis::X => {
                            if target.x <= split.position {
                                (split.left.clone(), split.right.clone())
                            } else {
                                (split.right.clone(), split.left.clone())
                            }
                        }
                        Axis::Y => {
                            if target.y <= split.position {
                                (split.left.clone(), split.right.clone())
                            } else {
                                (split.right.clone(), split.left.clone())
                            }
                        }
                        Axis::Z => {
                            if target.z <= split.position {
                                (split.left.clone(), split.right.clone())
                            } else {
                                (split.right.clone(), split.left.clone())
                            }
                        }
                    };

                    search(next_node, target, excludes, best_point, best_distance);

                    let axis_distance = match split.axis {
                        Axis::X => (target.x - split.position).abs() as f64,
                        Axis::Y => (target.y - split.position).abs() as f64,
                        Axis::Z => (target.z - split.position).abs() as f64,
                    };

                    if axis_distance < *best_distance {
                        search(other_node, target, excludes, best_point, best_distance);
                    }
                }
            }
        }

        search(self.root.clone(), target, excludes, &mut best_point, &mut best_distance);
        best_point
    }
}

impl Day08 {
    fn parse(input: &str) -> Vec<Point> {
        input.lines().map(|row| {
            let split = row.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            Point::new(split[0], split[1], split[2])
        }).collect::<Vec<Point>>()
    }
}

impl Day for Day08 {
    fn part1(&self, input: &str) -> String {
        let points = Self::parse(input);
        let kd_tree = KdTree {
            root: KdTree::build_tree(points.clone(), 0, 1, Weak::new()),
        };

        let mut excludes = HashMap::new();
        let mut pairs_found = 0; 
        let mut pairs = vec![];
        for i in 0..1000 {
            println!("Iteration {}", i);
            let mut closest_pair: Option<(Point, Point)> = None;
            let mut current_best_distance = f64::MAX;
            for point in &points {
                let exclude_set = excludes.entry(*point).or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(*point);
                    set
                });
                let neighbor = kd_tree.nearest_neighbor_search(point, exclude_set);
                if let Some(nearest) = neighbor {
                    let distance = ((point.x - nearest.x).pow(2)
                        + (point.y - nearest.y).pow(2)
                        + (point.z - nearest.z).pow(2)) as f64;
                    if distance < current_best_distance {
                        current_best_distance = distance;
                        closest_pair = Some((*point, nearest));
                    }
                }
            }

            // If after an iteration we didnt find a closest pair we are done
            if (closest_pair.is_none()) {
                break;
            } else {
                let (p1, p2) = closest_pair.unwrap();
                // Ensure exclude sets exist for both points
                excludes.entry(p1).or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(p1);
                    set
                });
                excludes.entry(p2).or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(p2);
                    set
                });
                excludes.get_mut(&p1).unwrap().insert(p2);
                excludes.get_mut(&p2).unwrap().insert(p1);
                pairs_found += 1;
                pairs.push((p1, p2));
            }
        }
        "No closest pair found".to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "42".into()
    }
}
