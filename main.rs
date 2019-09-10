use std::env;
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::io;
use std::cmp::{max, Ordering, Reverse};

#[derive(Debug)]
struct Item {
    index: usize,
    value: usize,
    weight: usize
}

fn parse(input_data: &str) -> Result<(usize, Vec<Item>), Box<std::error::Error>> {
    let mut iter = input_data.lines();

    let mut size_capacity: Vec<usize> = iter.next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let size = size_capacity[0];
    let capacity = size_capacity[1];

    let mut items: Vec<Item> = iter.enumerate()
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

fn calculate_optimistic_value(cur_value: usize, capacity: usize, items: &mut [Item]) -> f32 {
//    println!("{:?} {} {}", items, cur_value, capacity);

    let mut remaining_capacity = capacity;
    let mut value = cur_value as f32;

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
//        println!(">>> {:?} {} {}", item, value, remaining_capacity);
        value += item.value as f32 * remaining_capacity as f32 / item.weight as f32;
    }

    value
}

fn calculate_optimistic_value_node(items: &mut [Item], node: (usize, i32, usize)) -> f32 {
    return calculate_optimistic_value(node.0, node.1 as usize, &mut items[node.2..])
}

fn branch_and_bound(capacity: usize, items: &mut [Item]) -> (usize, Vec<u32>) {
    sort_by_value_density(items);

    let mut max_value = 0;

    let mut stack: Vec<(usize, i32, usize)> = Vec::new();

    stack.push((0, capacity as i32, 0));


    while !stack.is_empty() {
        let (value, cur_capacity, index) = stack.pop().unwrap();

        if cur_capacity < 0 {
            continue;
        }

        if value > max_value {
            max_value = value;
        }

        if capacity <= 0 || index >= items.len() {
            continue;
        }

//        let max_profit = calculate_optimistic_value(value, cur_capacity as usize, &mut items[index..]);

//        println!("{} {} {} {} {}",value, cur_capacity, index, max_profit, max_value);

        let node_exclude = (value, cur_capacity, index + 1);
        if calculate_optimistic_value_node(items, node_exclude) > value as f32 {
            stack.push(node_exclude);
        }

        let node_include = (value + items[index].value, cur_capacity - items[index].weight as i32, index + 1);

        if calculate_optimistic_value_node(items, node_include) > value as f32 {
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

    run(file_name);
}
