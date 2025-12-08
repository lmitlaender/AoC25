use super::Day;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Day08;

pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
    
    pub fn are_all_same_root(&mut self) -> bool {
        let first_root = self.find(0);
        for i in 1..self.parent.len() {
            if self.find(i) != first_root {
                return false;
            }
        }
        true
    }
}

impl Day08 {
    fn parse(input: &str) -> Vec<Point> {
        input.lines().map(|row| {
            let split = row.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            Point::new(split[0], split[1], split[2])
        }).collect::<Vec<Point>>()
    }

    fn squared_distance(a: &Point, b: &Point) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl Day for Day08 {
    fn part1(&self, input: &str) -> String {
        let points = Self::parse(input);
        let mut heap = BinaryHeap::new();
        let mut uf = UnionFind::new(points.len());

        // Populate the heap with all pairwise distances
        for (i, point) in points.iter().enumerate() {
            for j in (i + 1)..points.len() {
                let other_point = &points[j];
                let dist = Self::squared_distance(point, other_point);
                heap.push(Reverse((dist, i, j)));
            }
        }

        let mut iterations = 0;
        while iterations < 1000 {
            if let Some(Reverse((_dist, i, j))) = heap.pop() {
                uf.union(i, j);
                iterations += 1;
            } else {
                break; // Exit if the heap is empty
            }
        }

        // Count the size of each connected component
        let mut component_sizes = vec![0; points.len()];
        for i in 0..points.len() {
            let root = uf.find(i);
            component_sizes[root] += 1;
        }

        // Find the sizes of the three largest components
        component_sizes.sort_unstable_by(|a, b| b.cmp(a));
        let largest_three: Vec<_> = component_sizes.into_iter().take(3).collect();

        largest_three.iter().product::<usize>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = Self::parse(input);
        let mut heap = BinaryHeap::new();
        let mut uf = UnionFind::new(points.len());

        // Populate the heap with all pairwise distances
        for (i, point) in points.iter().enumerate() {
            for j in (i + 1)..points.len() {
                let other_point = &points[j];
                let dist = Self::squared_distance(point, other_point);
                heap.push(Reverse((dist, i, j)));
            }
        }

        let mut iterations = 0;
        let mut result = 0;
        while true {
            if let Some(Reverse((_dist, i, j))) = heap.pop() {
                uf.union(i, j);
                if uf.are_all_same_root() {
                    println!("All connected after {} iterations with idx {} and {}", iterations, i, j);
                    result = points[i].x * points[j].x;
                    break;
                }
                iterations += 1;
            } else {
                break; // Exit if the heap is empty
            }
        }

        result.to_string()
    }
}
