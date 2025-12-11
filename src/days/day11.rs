use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::vec;

use super::Day;
use good_lp::{Expression, Solution, SolverModel, Variable, constraint, default_solver, microlp, solvers::highs, variable, variables};
use regex::Regex;

pub struct Day11;

pub struct Graph {
    pub adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self {
            adj_list: vec![Vec::new(); size],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
    }

    pub fn neighbors(&self, u: usize) -> &[usize] {
        &self.adj_list[u]
    }
}

impl Day11 {
    fn parse(input: &str) -> (usize, usize, usize, usize, usize, Graph) {
        let mut graph = Graph::new(input.lines().count() + 1); // +1 for "out" node
        let mut node_indices = std::collections::HashMap::new();
        let mut idx = 0;

        // Map node names to indices
        for line in input.lines() {
            if let Some((node, _)) = line.split_once(':') {
                let node = node.trim();
                node_indices.entry(node.to_string()).or_insert_with(|| {
                    let i = idx;
                    idx += 1;
                    i
                });
            }
        }

        // Add "out" node
        node_indices.entry("out".to_string()).or_insert_with(|| {
            let i = idx;
            idx += 1;
            i
        });

        // Add all edges
        for line in input.lines() {
            if let Some((node, edges)) = line.split_once(':') {
                let node = node.trim();
                let u = node_indices[node];
                for v_name in edges.trim().split_whitespace() {
                    let v = node_indices[v_name];
                    graph.add_edge(u, v);
                }
            }
        }


        (node_indices["you"], node_indices["out"], node_indices["svr"], node_indices["dac"], node_indices["fft"],  graph)
    }

    fn Astar(start: usize, target: usize, graph: &Graph) -> Option<Vec<usize>> {
        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut came_from = std::collections::HashMap::new();
        let mut g_score = vec![i32::MAX; graph.adj_list.len()];
        let mut f_score = vec![i32::MAX; graph.adj_list.len()];

        g_score[start] = 0;
        f_score[start] = 0;


        open_set.push(Reverse((0, start)));

        while let Some(Reverse((curr_f, current_node))) = open_set.pop() {
            if f_score[current_node] < curr_f {
                continue; // We can have a better path already, if so, skip this wrong prev entry
            }

            if current_node == target {
                // Path found, reconstruct the path
                let mut path = Vec::new();
                let mut current = current_node;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(current);
                    current = prev;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            closed_set.insert(current_node);

            // Expand neighbors
            for &neighbor in graph.neighbors(current_node) {
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let new_g_score = g_score[current_node] + 1; // Uniform cost

                if new_g_score < g_score[neighbor] {
                    came_from.insert(neighbor, current_node);
                    g_score[neighbor] = new_g_score;
                    f_score[neighbor] = new_g_score; // h(neighbor) = 0 cause we dont have any metric to calc dist or anything, not good

                    // Add the neighbor with the new best f_score
                    open_set.push(Reverse((f_score[neighbor], neighbor)));
                }
            }
        }

        None // Return None if no path is found
    }

    fn count_all_paths_with_memo(graph: &Graph, start: usize, target: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
        fn dfs(graph: &Graph, current: usize, target: usize, visited: &mut Vec<bool>, memo: &mut HashMap<(usize, usize), usize>) -> usize {
            if let Some(&cached) = memo.get(&(current, target)) {
                return cached;
            }
            
            if current == target {
                return 1;
            }
            visited[current] = true;
            let mut count = 0;
            for &neighbor in &graph.adj_list[current] {
                if !visited[neighbor] {
                    count += dfs(graph, neighbor, target, visited, memo);
                }
            }
            visited[current] = false;
            memo.insert((current, target), count);
            count
        }

        let mut visited = vec![false; graph.adj_list.len()];
        let result = dfs(graph, start, target, &mut visited, memo);
        memo.insert((start, target), result);
        result
    }
}

impl Day for Day11 {
    fn part1(&self, input: &str) -> String {
        let (start, target, _, _, _, graph) = Self::parse(input);
        let mut memo = HashMap::new();
        let count = Self::count_all_paths_with_memo(&graph, start, target, &mut memo);
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (_, out, svr, dac, fft, graph) = Self::parse(input);
        let mut memo = HashMap::new();

        let paths_svr_to_dac = Self::count_all_paths_with_memo(&graph, svr, dac, &mut memo);
        let paths_dac_to_fft = Self::count_all_paths_with_memo(&graph, dac, fft, &mut memo);
        let paths_fft_to_out = Self::count_all_paths_with_memo(&graph, fft, out, &mut memo);
        let paths_svr_to_fft = Self::count_all_paths_with_memo(&graph, svr, fft, &mut memo);
        let paths_fft_to_dac = Self::count_all_paths_with_memo(&graph, fft, dac, &mut memo);
        let paths_dac_to_out = Self::count_all_paths_with_memo(&graph, dac, out, &mut memo);

        let total_paths =
            (paths_svr_to_dac * paths_dac_to_fft * paths_fft_to_out) +
            (paths_svr_to_fft * paths_fft_to_dac * paths_dac_to_out);

        total_paths.to_string()
    }
}
