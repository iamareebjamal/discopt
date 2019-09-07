use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::cmp::max;

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

fn solve(capacity: usize, items: &[Item]) -> (usize, &[u32]) {
    let mut cache: Vec<Vec<usize>> = vec![vec![0; capacity + 1]; items.len() + 1];

    for i in 0..items.len() + 1 {
        for w in 0..capacity + 1 {
            if i == 0 || w == 0 {
                cache[i][w] = 0;
            } else if items[i - 1].weight <= w {
                cache[i][w] = max(cache[i - 1][w], items[i - 1].value + cache[i - 1][w - items[i - 1].weight]);
            } else {
                cache[i][w] = cache[i - 1][w];
            }
        }
    }

    (cache[items.len()][capacity], &[])
}

fn run(file_name: &str) -> io::Result<()> {
    let mut file = File::open(file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (capacity, items) = parse(&contents).unwrap();
    let (value, taken) = solve(capacity, &items);

    println!("{}", value);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    run(file_name);
}
