use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.trim().split("\n");

    let mut forward = 0;
    let mut depth = 0;

    for line in lines {
        let v: Vec<String> = line
            .trim()
            .split(" ")
            .map(|val| val.parse().unwrap())
            .collect();

        let direction = &v[0];
        let unit = v[1].parse::<u32>().unwrap();

        if direction == "forward" {
            forward+=unit;
        }

        if direction == "up" {
            depth-=unit;
        }

        if direction == "down" {
            depth+=unit;
        }
    }

    println!("depth: {}", depth);
    println!("forward: {}", forward);
    println!("answer: {}", depth*forward);

}
