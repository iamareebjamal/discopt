use std::env;
use std::fs::File;
use std::io::Read;
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

    let mut new = 0;
    while i > 0 && w > 0 {
        if cache[i][w] != cache[i-1][w] {
            taken[i - 1] = 1;
            w -= items[i - 1].weight;
            new += items[i - 1].value;
        } else {
            i -= 1;
        }
    }

    println!("{}", new);

    (cache[items.len()][capacity], taken)
}

fn greedy_density(capacity: usize, items: &mut [Item]) -> (usize, Vec<u32>) {
    items.sort_by_key(|item| (Reverse(item.value / item.weight), Reverse(item.value)));

    let mut remaining_capacity = capacity;
    let mut items_inserted = 0;
    let mut value = 0;

    let mut taken = vec![0; items.len()];

    for item in items {
        if item.weight <= remaining_capacity {
            items_inserted += 1;
            remaining_capacity -= item.weight;
            value += item.value;
            taken[item.index] = 1;
        }
    }

    (value, taken)
}

fn solve(capacity: usize, items: &mut [Item]) -> (usize, Vec<u32>) {
    if capacity * items.len() <= 100_000_000 {
        dynamic_programming(capacity, items)
    } else {
        greedy_density(capacity, items)
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
