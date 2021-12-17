fn main() {
    let max_y = 154;

    // The region is below the y axis, so the solution is for y to be such that on the return 
    // it jumps straight from 0 to -154.
    println!("Analytic solution for part 1: {}", (max_y * (max_y - 1)) / 2)
}