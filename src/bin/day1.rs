use std::fs;

fn get_single_increase_count(mut readings: Vec<u32>) -> u32{
    let mut readings= readings.iter_mut();

    let mut previous_reading = readings.next().unwrap();
    let mut increases = 0;

    for reading in readings {
        if reading > previous_reading {
            increases += 1;
        }
        previous_reading = reading;
    }
    increases
}

fn main() {
    let filename = "data/day1.1.txt";

    let input = fs::read_to_string(filename).unwrap();
    let depth_readings: Vec<u32> = input.split("\n").map(|i| i.parse::<u32>().unwrap()).collect();

    println!("Single increase count: {}", get_single_increase_count(depth_readings.clone()));
}
