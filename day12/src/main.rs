use petgraph::graphmap::UnGraphMap;
use std::collections::VecDeque;

fn main() {
    let edges = include_str!("input")
        .lines()
        .map(|l| l.split('-').collect::<Vec<&str>>())
        .map(|l| (Cave::from_string(l[0]), Cave::from_string(l[1]), ()))
        .collect::<Vec<(Cave, Cave, ())>>();

    let map = UnGraphMap::from_edges(edges);

    println!("Part 1: {:?}", count_paths(&map, false));
    println!("Part 2: {:?}", count_paths(&map, true));
}

fn count_paths(map: &UnGraphMap<Cave, ()>, can_visit_twice: bool) -> usize {
    let start = find_node(map, "start").unwrap();
    let end = find_node(map, "end").unwrap();

    let mut to_process = VecDeque::from([(vec![start], false)]);
    let mut found_paths = vec![];

    while let Some(path) = to_process.pop_front() {
        let current_cave = path.0.last().unwrap();
        // check all the neighbours
        for neighbour in map.neighbors(*current_cave) {
            let mut new_path;
            // if we're large we can visit multiple times, else check if we've already visited
            if neighbour.is_large || !path.0.contains(&neighbour) {
                new_path = path.clone();
            } else if can_visit_twice && !path.1 && neighbour != start && neighbour != end {
                // start and end can only be visited once! this hit me hard :(
                new_path = (path.0.clone(), true);
            } else {
                // we cant visit this neighbour, its now an invalid path, dont push to output/queue
                continue;
            }

            new_path.0.extend([neighbour]);
            // we're at the end
            if neighbour == end {
                found_paths.push(new_path);
            } else {
                // add it to the queue of paths to complete parsing
                to_process.push_back(new_path);
            }
        }
    }
    found_paths.len()
}

fn find_node<'a>(map: &'a UnGraphMap<Cave, ()>, name: &str) -> Option<Cave<'a>> {
    map.nodes().find(|n| n.name == name)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cave<'a> {
    name: &'a str,
    is_large: bool,
}

impl<'a> Cave<'a> {
    fn from_string(string: &'a str) -> Cave {
        Cave {
            is_large: string.chars().all(char::is_uppercase),
            name: string,
        }
    }
}
