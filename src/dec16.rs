use std::{cmp::{min, Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet}, fs::read_to_string, hash::Hash, usize};


#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    x: usize,
    y: usize,
    dir: Direction,
    dist: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



pub fn part1() -> usize {
    let maze: Vec<Vec<char>> = read_to_string("src/in/dec16.in")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut edges: HashMap<((usize, usize, Direction), (usize, usize, Direction)), usize> = HashMap::new();
    let mut neighbors_of_vertices: HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction)>> = HashMap::new();
    let mut Q: BinaryHeap<Node> = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize, Direction), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize, Direction), Option<(usize, usize, Direction)>> = HashMap::new();
    // Q.
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if maze[y][x]== '.' {
                let neighbors_when_facing_north = get_neighbors((x, y, Direction::N), &maze);
                neighbors_of_vertices.insert((x, y, Direction::N), neighbors_when_facing_north);
                Q.push(Node { x, y, dir: Direction::N, dist: usize::MAX });
                dist.insert((x, y, Direction::N), usize::MAX);
                prev.insert((x, y, Direction::N), None);

                let neighbors_when_facing_east = get_neighbors((x, y, Direction::E), &maze);
                neighbors_of_vertices.insert((x, y, Direction::E), neighbors_when_facing_east);
                Q.push(Node { x, y, dir: Direction::E, dist: usize::MAX });
                dist.insert((x, y, Direction::E), usize::MAX);
                prev.insert((x, y, Direction::E), None);

                let neighbors_when_facing_south = get_neighbors((x, y, Direction::S), &maze);
                neighbors_of_vertices.insert((x, y, Direction::S), neighbors_when_facing_south);
                Q.push(Node { x, y, dir: Direction::S, dist: usize::MAX });
                dist.insert((x, y, Direction::S), usize::MAX);
                prev.insert((x, y, Direction::S), None);

                let neighbors_when_facing_west = get_neighbors((x, y, Direction::W), &maze);
                neighbors_of_vertices.insert((x, y, Direction::W), neighbors_when_facing_west);
                Q.push(Node { x, y, dir: Direction::W, dist: usize::MAX });
                dist.insert((x, y, Direction::W), usize::MAX);
                prev.insert((x, y, Direction::W), None);

                if  maze[y-1][x] != '#' {
                    edges.insert(((x, y, Direction::N), (x, y-1, Direction::N)), 1);
                    edges.insert(((x, y, Direction::E), (x, y-1, Direction::N)), 1001);
                    edges.insert(((x, y, Direction::W), (x, y-1, Direction::N)), 1001);
                };
                if  maze[y][x+1] != '#' {
                    let e = Node {x: x+1, y: y, dir: Direction::E, dist: usize::MAX };
                    edges.insert(((x, y, Direction::N), (x+1, y, Direction::E)), 1001);
                    edges.insert(((x, y, Direction::E), (x+1, y, Direction::E)), 1);
                    edges.insert(((x, y, Direction::S), (x+1, y, Direction::E)), 1001);
                }
                if  maze[y+1][x] != '#' {
                    let s = Node {x: x, y: y+1, dir: Direction::S, dist: usize::MAX };
                    edges.insert(((x, y, Direction::E), (x, y+1, Direction::S)), 1001);
                    edges.insert(((x, y, Direction::S), (x, y+1, Direction::S)), 1);
                    edges.insert(((x, y, Direction::W), (x, y+1, Direction::S)), 1001);

                }
                if  maze[y][x-1] != '#' {
                    edges.insert(((x, y, Direction::N), (x-1, y, Direction::W)), 1001);
                    edges.insert(((x, y, Direction::S), (x-1, y, Direction::W)), 1001);
                    edges.insert(((x, y, Direction::W), (x-1, y, Direction::W)), 1);
                }
            }
            if maze[y][x]== 'S' {
                let neighbors_when_facing_north = get_neighbors((x, y, Direction::N), &maze);
                neighbors_of_vertices.insert((x, y, Direction::N), neighbors_when_facing_north);
                Q.push(Node { x, y, dir: Direction::N, dist: 0 });
                // dist.insert((x, y, Direction::N), usize::MAX); // do i need this?
                dist.insert((x, y, Direction::S), usize::MAX);
                dist.insert((x, y, Direction::W), usize::MAX);

                if  maze[y-1][x] != '#' {
                    edges.insert(((x, y, Direction::N), (x, y-1, Direction::N)), 1001);
                    // edges.insert(((x, y, Direction::E), (x, y-1, Direction::N)), 1001);
                    // edges.insert(((x, y, Direction::W), (x, y-1, Direction::N)), 1001);
                };
                if  maze[y][x+1] != '#' {
                    // let e = Node {x: x+1, y: y, dir: Direction::E, dist: usize::MAX, prev: None };
                    // edges.insert(((x, y, Direction::N), (x+1, y, Direction::E)), 1001);
                    edges.insert(((x, y, Direction::N), (x+1, y, Direction::E)), 1);
                    // edges.insert(((x, y, Direction::S), (x+1, y, Direction::E)), 1001);
                }
            }
            if maze[y][x]== 'E' {
                Q.push(Node { x, y, dir: Direction::N, dist: usize::MAX });
                dist.insert((x, y, Direction::N), usize::MAX);
                prev.insert((x, y, Direction::N), None);
                Q.push(Node { x, y, dir: Direction::E, dist: usize::MAX });
                dist.insert((x, y, Direction::E), usize::MAX);
                prev.insert((x, y, Direction::E), None);

            }
        }
    }
    
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    while let Some(vertex) = Q.pop() {  
        if visited.contains(&(vertex.x, vertex.y, vertex.dir)) { continue; }
        visited.insert((vertex.x, vertex.y, vertex.dir));
        if (vertex.x, vertex.y, vertex.dir) == (3,10,Direction::E) {
            dbg!(neighbors_of_vertices.get(&(vertex.x, vertex.y, vertex.dir)));
        }
        if let Some(neighbors) = neighbors_of_vertices.get(&(vertex.x, vertex.y, vertex.dir)) {
            for neighbor in neighbors {
                if let Some(alt) = vertex.dist.checked_add(*edges.get(&((vertex.x, vertex.y, vertex.dir), (neighbor.0, neighbor.1, neighbor.2))).unwrap()) {
                    if alt < *dist.get(&(neighbor.0, neighbor.1, neighbor.2)).unwrap() {
                        prev.insert((neighbor.0, neighbor.1, neighbor.2), Some((vertex.x, vertex.y, vertex.dir)));
                        dist.insert((neighbor.0, neighbor.1, neighbor.2), alt);
                        Q.push(Node { x: neighbor.0, y: neighbor.1, dir: neighbor.2, dist: alt });
                    }
                }
            }
        }
    }
    // east is correct for big puzzle
    // første ga 36
    // andre ga 48

    // let mut pos = (2, maze.len()-6, Direction::E);
    let mut pos = (maze[0].len()-2, 1, Direction::E);
    let mut i = 0;
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut stack = vec!(prev.get(&pos).unwrap().unwrap());
    let mut visitedd: HashSet<(usize, usize, Direction)> = HashSet::new();
    while stack.len()>0 {
        let (x, y, dir) = stack.pop().unwrap();
        if visitedd.contains(&(x, y, dir)) { continue; }
        visitedd.insert((x, y, dir));
        set.insert((x, y));
        // if (x, y) != (0, 0) {
        if let Some(p) = prev.get(&(x, y,dir)) {
            if let Some(_) = p {
                stack.push(prev.get(&(x, y,dir)).unwrap().unwrap());
                let d = dist.get(&(x, y, dir)).unwrap();
                if dist.get(&(x-1, y, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x-1, y, Direction::N));
                }
                if dist.get(&(x, y-1, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y-1, Direction::N));
                }
                if dist.get(&(x+1, y, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x+1, y, Direction::N));
                }
                if dist.get(&(x, y+1, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y+1, Direction::N));
                }
                if dist.get(&(x-1, y, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x-1, y, Direction::E));
                }
                if dist.get(&(x, y-1, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y-1, Direction::E));
                }
                if dist.get(&(x+1, y, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x+1, y, Direction::E));
                }
                if dist.get(&(x, y+1, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y+1, Direction::E));
                }
                if dist.get(&(x-1, y, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x-1, y, Direction::S));
                }
                if dist.get(&(x, y-1, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y-1, Direction::S));
                }
                if dist.get(&(x+1, y, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x+1, y, Direction::S));
                }
                if dist.get(&(x, y+1, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y+1, Direction::S));
                }
                if dist.get(&(x-1, y, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x-1, y, Direction::W));
                }
                if dist.get(&(x, y-1, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y-1, Direction::W));
                }
                if dist.get(&(x+1, y, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x+1, y, Direction::W));
                }
                if dist.get(&(x, y+1, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
                    stack.push((x, y+1, Direction::W));
                }
                
                
            }
        }
        let dd = dist.get(&(x, y, dir));
        if dd == None { continue;}
        let d = dd.unwrap();
        if dist.get(&(x-1, y, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x-1, y, Direction::N));
        }
        if dist.get(&(x, y-1, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y-1, Direction::N));
        }
        if dist.get(&(x+1, y, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x+1, y, Direction::N));
        }
        if dist.get(&(x, y+1, Direction::N)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::N))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y+1, Direction::N));
        }
        if dist.get(&(x-1, y, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x-1, y, Direction::E));
        }
        if dist.get(&(x, y-1, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y-1, Direction::E));
        }
        if dist.get(&(x+1, y, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x+1, y, Direction::E));
        }
        if dist.get(&(x, y+1, Direction::E)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::E))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y+1, Direction::E));
        }
        if dist.get(&(x-1, y, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x-1, y, Direction::S));
        }
        if dist.get(&(x, y-1, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y-1, Direction::S));
        }
        if dist.get(&(x+1, y, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x+1, y, Direction::S));
        }
        if dist.get(&(x, y+1, Direction::S)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::S))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y+1, Direction::S));
        }
        if dist.get(&(x-1, y, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x-1, y, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x-1, y, Direction::W));
        }
        if dist.get(&(x, y-1, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x, y-1, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y-1, Direction::W));
        }
        if dist.get(&(x+1, y, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x+1, y, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x+1, y, Direction::W));
        }
        if dist.get(&(x, y+1, Direction::W)) == Some(&(d-1)) || Some(dist.get(&(x, y+1, Direction::W))) == Some(d.checked_sub(1001).as_ref()) {
            stack.push((x, y+1, Direction::W));
        }
            
    }
    

    if let Some(_) = prev.get(&(maze[0].len()-2, 1, Direction::N)) {
        if let Some(_) = prev.get(&(maze[0].len()-2, 1, Direction::E)) {
            return *min(dist.get(&(maze[0].len()-2, 1, Direction::N)), dist.get(&(maze[0].len()-2, 1, Direction::E))).unwrap();
        } else {
            return *dist.get(&(maze[0].len()-2, 1, Direction::N)).unwrap();
        }
    }  
    *dist.get(&(maze[0].len()-2, 1, Direction::E)).unwrap()


}

fn get_neighbors((x, y, dir): (usize, usize, Direction), maze: &Vec<Vec<char>>) -> Vec<(usize, usize, Direction)> {
    let mut neighbors = vec!();
    if  maze[y-1][x] != '#' && dir != Direction::S {
        let n = (x, y-1, Direction::N);
        neighbors.push(n);
    }
    if  maze[y][x+1] != '#' && dir !=Direction::W {
        let e = (x+1, y, Direction::E);
        neighbors.push(e);
    }
    if  maze[y+1][x] != '#' && dir !=Direction::N {
        let s = (x, y+1, Direction::S);
        neighbors.push(s);
    }
    if  maze[y][x-1] != '#' && dir !=Direction::E {
        let w = (x-1, y, Direction::W);
        
        neighbors.push(w);
    }
    return neighbors;
}
pub fn part2() -> usize {
    0
}