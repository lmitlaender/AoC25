use super::Day;
use std::ops::Index;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::thread;

pub struct Day07;
impl Day07 {
    fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
    }

    fn stream_down(
        map: Arc<Vec<Vec<char>>>, 
        from_x: usize, 
        from_y: usize, 
        m_count: Arc<Mutex<i64>>, 
        m_split_set: Arc<Mutex<HashSet<(usize, usize)>>>
    ) {
        for y in from_y..map.len() {
            if map[y][from_x] == '^' {
                let mut handles = vec![];
                {   
                    let mut started_set = m_split_set.lock().unwrap();
                    if started_set.insert((y, from_x)) {
                        let mut count = m_count.lock().unwrap();
                        *count += 1;
                        
                        let map_clone = Arc::clone(&map);
                        let m_count_clone = Arc::clone(&m_count);
                        let m_split_set_cloned = Arc::clone(&m_split_set);
                        let handle = thread::spawn(move || {
                            Self::stream_down(map_clone, from_x - 1, y, m_count_clone, m_split_set_cloned)
                        });
                        handles.push(handle);

                        let map_clone2 = Arc::clone(&map);
                        let m_count_clone2 = Arc::clone(&m_count);
                        let m_split_set_cloned2 = Arc::clone(&m_split_set);
                        let handle = thread::spawn(move || {
                            Self::stream_down(map_clone2, from_x + 1, y, m_count_clone2, m_split_set_cloned2)
                        });
                        handles.push(handle);
                    }
                }
                handles.into_iter().for_each(|handle| handle.join().unwrap());
                break;
            }
        }
    }

    fn stream_down2(
        map: Arc<Vec<Vec<char>>>, 
        from_x: usize, 
        from_y: usize, 
        m_count: Arc<Mutex<i64>>, 
        m_split_set: Arc<Mutex<HashSet<(usize, usize)>>>
    ) {
        for y in from_y..map.len() {
            if map[y][from_x] == '^' {
                let mut handles = vec![];
                {   
                    let mut split_set = m_split_set.lock().unwrap();
                    let map_clone = Arc::clone(&map);
                    let m_count_clone = Arc::clone(&m_count);
                    let m_split_set_cloned = Arc::clone(&m_split_set);
                    let handle = thread::spawn(move || {
                        Self::stream_down2(map_clone, from_x - 1, y, m_count_clone, m_split_set_cloned)
                    });
                    handles.push(handle);

                    let map_clone2 = Arc::clone(&map);
                    let m_count_clone2 = Arc::clone(&m_count);
                    let m_split_set_cloned2 = Arc::clone(&m_split_set);
                    let handle = thread::spawn(move || {
                        Self::stream_down2(map_clone2, from_x + 1, y, m_count_clone2, m_split_set_cloned2)
                    });
                    handles.push(handle);
                }
                handles.into_iter().for_each(|handle| handle.join().unwrap());
                break;
            } else if y >= map.len() - 1 {
                let mut count = m_count.lock().unwrap();
                *count += 1;
            }
        }
    }
}

impl Day for Day07 {
    fn part1(&self, input: &str) -> String {
        println!("Start!");
        let char_matrix = Self::parse(input);
        
        let from_y = 0;
        let from_x = char_matrix[0].iter().position(|&c| c == 'S').expect("No 'S' found in first line");
        let m_count = Arc::new(Mutex::new(0i64));
        let m_started_set = Arc::new(Mutex::new(HashSet::new()));
        Self::stream_down(
            Arc::new(char_matrix),
            from_x,
            from_y,
            Arc::clone(&m_count),
            Arc::clone(&m_started_set),
        );
        m_count.lock().unwrap().to_string()
    }

    fn part2(&self, input: &str) -> String {
        println!("Start 2!");
        let char_matrix = Self::parse(input);
        
        let from_y = 0;
        let from_x = char_matrix[0].iter().position(|&c| c == 'S').expect("No 'S' found in first line");
        let m_count = Arc::new(Mutex::new(0i64));
        let m_started_set = Arc::new(Mutex::new(HashSet::new()));
        Self::stream_down2(
            Arc::new(char_matrix),
            from_x,
            from_y,
            Arc::clone(&m_count),
            Arc::clone(&m_started_set)
        );
        m_count.lock().unwrap().to_string()
    }
}
