use std::fs;

fn main() {
    let contents = fs::read_to_string("input/part1.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.trim().split("\n");

    let mut increases = 0;

    // Initialize last as INF so the first reading is never
    // counted as an increase.
    let mut last = f64::INFINITY;

    for line in lines {
        let as_int = line.parse::<f64>().unwrap();

        if as_int > last {
            increases+=1;
        }

        last = as_int;
    }

    println!("{}", increases)
}
