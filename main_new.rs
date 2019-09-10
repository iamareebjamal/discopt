use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::cmp::{max, Reverse};

#[derive(Debug)]
struct Item {
    index: usize,
    value: usize,
    weight: usize
}

fn parse(input_data: &str) -> Result<(usize, Vec<Item>), Box<dyn std::error::Error>> {
    let mut iter = input_data.lines();

    let size_capacity: Vec<usize> = iter.next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let capacity = size_capacity[1];

    let items: Vec<Item> = iter.enumerate()
        .map(|(index, line)| {
            let item_lines: Vec<usize> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            Item {
                index,
                value: item_lines[0],
                weight: item_lines[1]
            }
        }).collect();

    return Ok((capacity, items))
}

fn dynamic_programming(capacity: usize, items: &[Item]) -> (usize, Vec<u32>) {
    let mut cache: Vec<Vec<usize>> = vec![vec![0; capacity + 1]; items.len() + 1];

    for i in 0..=items.len() {
        for w in 0..=capacity {
            if i == 0 || w == 0 {
                cache[i][w] = 0;
            } else if items[i - 1].weight <= w {
                cache[i][w] = max(cache[i - 1][w], items[i - 1].value + cache[i - 1][w - items[i - 1].weight]);
            } else {
                cache[i][w] = cache[i - 1][w];
            }
        }
    }

    let mut taken = vec![0; items.len()];

    let mut i = items.len();
    let mut w = capacity;

    while i > 0 && w > 0 {
        if cache[i][w] != cache[i-1][w] {
            taken[i - 1] = 1;
            w -= items[i - 1].weight;
        } else {
            i -= 1;
        }
    }

    (cache[items.len()][capacity], taken)
}

fn sort_by_value_density(items: &mut [Item]) {
    items.sort_by_key(|item| (Reverse(item.value / item.weight), Reverse(item.value)));
}

fn greedy_density(capacity: usize, items: &mut [Item]) -> (usize, Vec<u32>) {
    sort_by_value_density(items);

    let mut remaining_capacity = capacity;
    let mut value = 0;

    let mut taken = vec![0; items.len()];

    for item in items {
        if item.weight <= remaining_capacity {
            remaining_capacity -= item.weight;
            value += item.value;
            taken[item.index] = 1;
        }
    }

    (value, taken)
}

struct Node {
    value: usize,
    capacity: i32,
    index: usize,
    bound: f32
}

impl Node {

    fn new(value: usize, capacity: i32, index: usize) -> Node {
        return Node { value, capacity, index, bound: 0.0 }
    }

    fn calculate_optimistic_value(&mut self, all_items: &mut [Item]) -> f32 {
        if self.capacity < 0 {
            self.bound = 0.0;

            return self.bound;
        }

        let mut remaining_capacity = self.capacity as usize;
        let mut value = self.value as f32;
        let items = &mut all_items[self.index..];

        let mut last_item: Option<&Item> = None;

        for item in items.iter() {
            if item.weight <= remaining_capacity {
                remaining_capacity -= item.weight;
                value += item.value as f32;
            } else {
                last_item = Some(item);
                break;
            }
        }

        if last_item.is_some() && remaining_capacity > 0 {
            let item = last_item.unwrap();
            // println!(">>> {:?} {} {}", item, value, remaining_capacity);
            value += item.value as f32 * remaining_capacity as f32 / item.weight as f32;
        }

        self.bound = value;

        return self.bound
    }

}

fn branch_and_bound(capacity: usize, items: &mut [Item]) -> (usize, Vec<u32>) {
    sort_by_value_density(items);

    let mut max_value = 0;

    let mut stack: Vec<Node> = Vec::new();

    stack.push(Node::new(0, capacity as i32, 0));

    while !stack.is_empty() {
        let node = stack.pop().unwrap();

        if node.bound < max_value as f32 {
            continue;
        }

        if node.value > max_value {
            max_value = node.value;
        }

        if node.index >= items.len() {
            continue;
        }

        let mut node_exclude = Node::new(node.value, node.capacity, node.index + 1);
        if node_exclude.calculate_optimistic_value(items) > max_value as f32 {
            stack.push(node_exclude);
        }

        let mut node_include = Node::new(node.value + items[node.index].value, node.capacity - items[node.index].weight as i32, node.index + 1);
        if node_include.calculate_optimistic_value(items) > max_value as f32 {
            stack.push(node_include);
        }
    }

    println!("{}", max_value);

    return greedy_density(capacity, items);
}

fn solve(capacity: usize, items: &mut [Item]) -> (usize, Vec<u32>) {
    if capacity * items.len() <= 100_000_000 {
        branch_and_bound(capacity, items)
    } else {
        branch_and_bound(capacity, items)
    }
}

fn print_vec(taken: &[u32]) {
    let mut iter = taken.iter();
    if let Some(item) = iter.next() {
        print!("{}", item);

        for item in iter {
            print!(" {}", item);
        }

        print!("\n");
    }
}

fn run(file_name: &str) -> io::Result<()> {
    let mut file = File::open(file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (capacity, mut items) = parse(&contents).unwrap();
    let (value, taken) = solve(capacity, &mut items);

    println!("{} 1", value);

    print_vec(&taken);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    run(file_name).unwrap();
}
