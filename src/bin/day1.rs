use std::fs;

fn main() {
    let filename = "data/day1.1.txt";

    let input = fs::read_to_string(filename).unwrap();
    let mut depth_readings = input.split("\n").map(|i| i.parse::<u32>().unwrap());

    let mut previous_reading = depth_readings.next().unwrap();
    let mut increases = 0;

    for reading in depth_readings {
        println!("{}, {}", reading, previous_reading);

        if reading > previous_reading {
            increases += 1;
        }
        previous_reading = reading;
    }
    println!("{}", increases);
}
