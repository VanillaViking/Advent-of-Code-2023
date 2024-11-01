use std::{fs, collections::HashMap};

#[derive(Debug, Clone)]
struct Node {
    label: String,
    right: String,
    left: String,
}

#[derive(Debug)]
struct Game {
    directions: String,
    network: HashMap<String, Node>,
    part2_nodes: Vec<Node>,
}

impl Game {
}


fn main() {
    let input = fs::read_to_string("input").unwrap();

    let (directions, nodes_str) = input.split_once("\n\n").unwrap();
    let mut all_nodes: HashMap<String, Node> = HashMap::new();
    let mut part2_nodes: Vec<Node> = Vec::new();

    nodes_str.lines().for_each(|line| {
        let (label, left_right_str) = line.split_once(" = ").unwrap();

        let (left, right) = left_right_str.split_once(", ").unwrap();

        all_nodes.insert(label.to_owned(), Node {label: label.to_owned(), left: left[1..].to_owned(), right: right[..3].to_owned()});
        if label.ends_with("A") {
            part2_nodes.push(Node {label: label.to_owned(), left: left[1..].to_owned(), right: right[..3].to_owned()});
        }
    });

    let mut game = Game {directions: directions.to_owned(), network: all_nodes, part2_nodes};

    dbg!(&game);

    println!("{}", part1(&game));
    println!("{}", part2(&mut game));



}


fn part1(game: &Game) -> u32 {
    let mut current = game.network.get("AAA").unwrap();
    let mut steps = 0;
    while current.label != "ZZZ" {
        for char in game.directions.chars() {
            current = match char { 
                'R' => game.network.get(&current.right).unwrap(),
                'L' => game.network.get(&current.left).unwrap(),
                _ => panic!()
            };

            steps += 1;

            if current.label == "ZZZ" {
                break;
            }

        }
    }
    return steps
}
    
#[derive(Debug, Clone)]
enum Rep {
    None,
    Running(u32),
    Done(u32),
}

fn part2(game: &mut Game) -> u64 {
    let mut steps: u64 = 0;
    let mut done = false;
    
    let mut repeating: Vec<Rep> = vec![Rep::None; game.part2_nodes.len()];

    while done == false {

        for char in game.directions.chars() {
            match char { 
                'R' => game.part2_nodes.iter_mut().enumerate().for_each(|(idx, node)| { 
                    let right = game.network.get(&node.right).unwrap();
                    if right.label.ends_with("Z") {
                       match repeating[idx] {
                        Rep::None => repeating[idx] = Rep::Running(0),
                        Rep::Running(s) => repeating[idx] = Rep::Done(s+1),
                        Rep::Done(_) => (),
                       };
                    } else {
                       match repeating[idx] {
                        Rep::None => (),
                        Rep::Running(s) => repeating[idx] = Rep::Running(s+1),
                        Rep::Done(_) => (),
                    };
                    }
                    *node = right.clone();
                }),
                'L' => game.part2_nodes.iter_mut().enumerate().for_each(|(idx, node)| { 
                    let left = game.network.get(&node.left).unwrap();
                    if left.label.ends_with("Z") {
                       match repeating[idx] {
                        Rep::None => repeating[idx] = Rep::Running(0),
                        Rep::Running(s) => repeating[idx] = Rep::Done(s+1),
                        Rep::Done(_) => (),
                       };
                    } else {
                       match repeating[idx] {
                        Rep::None => (),
                        Rep::Running(s) => repeating[idx] = Rep::Running(s+1),
                        Rep::Done(_) => (),
                    };
                    }
                    *node = left.clone()
                }),
                _ => panic!()
            };
            steps += 1;
            
            done = true;
            for node in game.part2_nodes.iter() {
                if !node.label.ends_with("Z") {
                    done = false
                }
            }
            dbg!(&repeating);
        };
    }

    steps
}
