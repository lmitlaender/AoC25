use super::Day;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use geo::{Contains, Coord, Polygon, Rect};

pub struct Day09;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i64, 
    y: i64,
}

impl Day09 {
    fn parse(input: &str) -> Vec<Point> {
        input.lines().map(|row| {
            let split = row.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            Point { x: split[0], y: split[1] }
        }).collect::<Vec<Point>>()
    }

    // Wasn't used in the end
    fn get_bin_matrix(points: &Vec<Point>) -> Vec<Vec<u8>> {
        let mut max_x = 0;
        let mut max_y = 0;
        
        for point in points {
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }

        let mut matrix = vec![vec![0u8; (max_x + 1) as usize]; (max_y + 1) as usize];

        // By utilizing the knowledge that passing an edge means we are inside the shape, we can fill the shape
        let mut vert_lines = vec![];
        for i in 0..points.len() {
            let p1 = &points[i];
            for j in i + 1..points.len() {
                let p2 = &points[j];
                if p1.x == p2.x {
                    vert_lines.push((p1.x, p1.y.min(p2.y), p1.y.max(p2.y)));
                }
            }
        }

        // Implement point in polygon with scanline algorithm
        let mut active = BinaryHeap::<Reverse<(i64, i64)>>::new();

        for y in 0..=max_y {
            // keep edges where y < end_y
            active.retain(|&Reverse((_, end_y))| y < end_y + 1);

            // add edges that start at this y
            for &(x, start_y, end_y) in &vert_lines {

                // skip horizontal edges
                if start_y == end_y { continue; }

                // top-inclusive
                if start_y == y {
                    active.push(Reverse((x, end_y)));
                }
            }

            // fill between pairs
            let mut inside = false;
            let mut last_x = 0;

            for &Reverse((x, _)) in &active {
                if inside {
                    for ix in last_x .. x + 1 {
                        matrix[y as usize][ix as usize] = 1;
                    }
                }
                inside = !inside;
                last_x = x;
            }
        }

        matrix
    }

}

impl Day for Day09 {
    // Brute force super simple
    fn part1(&self, input: &str) -> String {
        let points = Self::parse(input);

        let mut max_area = 0;

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let area = (((points[i].x - points[j].x)).abs() + 1) * (((points[i].y - points[j].y)).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = Self::parse(input);

        let coords: Vec<Coord<f64>> = points.iter().map(|p| Coord { x: p.x as f64, y: p.y as f64 }).collect();
        let poly = Polygon::new(coords.clone().into(), vec![]);

        let mut max_area = 0;

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let min_x = points[i].x.min(points[j].x);
                let max_x = points[i].x.max(points[j].x);
                let min_y = points[i].y.min(points[j].y);
                let max_y = points[i].y.max(points[j].y);

                let rect = Rect::new(
                    Coord { x: min_x as f64, y: min_y as f64 },
                    Coord { x: max_x as f64, y: max_y as f64 },
                );

                // Convert rect to polygon for contains check
                let rect_poly = Polygon::new(rect.to_polygon().exterior().clone(), vec![]);

                if poly.contains(&rect_poly) {
                    let area = ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize;
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area.to_string()
    }
}
