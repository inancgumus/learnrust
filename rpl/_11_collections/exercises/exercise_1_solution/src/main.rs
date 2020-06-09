fn main() {
    // Given a list of integers: 1..10

    // 1 -> use a vector
    let mut ints = [10, 8, 7, 3, 4, 7, 7, 2];

    // 2 -> return mean (the average value)
    println!("mean: {}", mean(&ints));

    // ints is mutable so we can sort it.
    ints.sort();

    // 3 -> return median (when sorted, the value in the middle position)
    println!("median: {}", median(&ints));

    // 4 -> return mode (the value that occurs most often)
    // println!("mode: {}", mode(&ints));
}

fn mean(nums: &[i32]) -> f32 {
    nums
        .iter()              // returns an iterator from the nums slice
        .sum::<i32>() as f32 // -> sum the numbers by iterating
                             // -> ::<i32> is the turbo-fish syntax.
                             //    tells `sum()` that it's going to sum
                             //    i32 values.
        / nums.len() as f32
}

fn median(nums: &[i32]) -> f32 {
    // match an arm depending on the length of the numbers in the nums
    match nums.len() {
        0 => 0.0,            // return 0.0 if l is zero?
        1 => nums[0] as f32, // return the first element if l is one?

        // first, put nums.len() into l.
        // then check, is l divisible by 2?
        // if so return the average of middle two elements
        l if l % 2 == 0 => mean(&vec![nums[l / 2 - 1], nums[l / 2]]),

        l => nums[l / 2] as f32, // if l is odd, return the middle element.
    }
}

// TODO: Implement...
// fn mode(nums: &[i32]) -> Vec<f32> {
//     vec![0.0, 0.1]
// }
