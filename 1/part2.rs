use std::fs;

fn main() {
    let contents = fs::read_to_string("input/part1.txt")
        .expect("Something went wrong reading the file");

    let lines = contents.trim().split("\n");

    // A
    // A B
    // A B C
    //   B C D
    //     C D
    //       D

    // Initialize last as INF so the first reading is never
    // counted as an increase.
    let mut last = f64::INFINITY;
    let mut increases = 0;

    // I'm lazy, there's probably a way to do this with just the iterator.
    let readings: Vec<f64> = lines.map(|line| line.parse::<f64>().unwrap()).collect();

    // Start at index 2, then walk up and always look back at the last 2 values + current
    // index to get the window. Then compare to the last window.
    for i in 2..readings.len() {
        let a = readings[i-2];
        let b = readings[i-1];
        let c = readings[i];

        let sum = a + b + c;

        if sum > last {
            increases+=1;
        }

        last = sum;
    }


    println!("{}", increases)
}
