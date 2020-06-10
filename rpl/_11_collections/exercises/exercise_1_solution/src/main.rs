fn main() {
    // Given a list of integers: 1..10

    // 1 -> use a vector
    let mut ints = vec![10, 8, 7, 3, 4, 7, 7, 2];

    // 2 -> return mean (the average value)
    println!("mean: {}", mean(&ints));

    // ints is mutable so we can sort it.
    ints.sort();

    // 3 -> return median (when sorted, the value in the middle position)
    println!("median: {}", median(&ints));

    // 4 -> return mode (the value that occurs most often)
    println!("mode: {:?}", mode(&ints));
}

// accept a slice &[i32] instead of a vector Vec<i32>
// this way sum() can accept a vector as well as a slice, etc.
fn sum(nums: &[i32]) -> i32 {
    let mut t = 0;
    for n in nums {
        t += *n;
    }
    t
}

fn mean(nums: &[i32]) -> f32 {
    // You can use `as T` to convert values.
    // For example: `as f32` means convert the value to f32.
    let len = nums.len() as f32;
    let s = sum(nums) as f32;
    s / len

    // ALTERNATIVE SOLUTION:
    // Rust is a functional language.
    // You can use Lazy iterators to sum values, and more.
    /*
    nums
        // returns an iterator from the nums slice.
        .iter()
        // -> sum all the numbers.
        // -> ::<i32> is the turbo-fish syntax.
        //    tells `sum()` that it's going to sum
        //    i32 values.
        .sum::<i32>() as f32
        /
        nums.len() as f32
    */
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

use std::collections::HashMap;

fn mode(nums: &[i32]) -> i32 {
    // multi-variable assignment? nope.
    // see this: https://github.com/rust-lang/rfcs/issues/372
    let mut mo = 0;
    let mut max = 0;

    // although we can do something similar below using an iterator,
    // it'd be out of scope for now. because you didn't learn it yet.
    // so we're going to use a simple for loop.

    // collect frequencies in a hash map.
    let mut freqs = HashMap::with_capacity(nums.len());
    for n in nums {
        // n contains a reference to the next number in the nums slice.
        // asterisk allows us to change the value from the same memory position.
        *freqs.entry(n).or_insert(0) += 1;

        // get the value from key `n`.
        // note: you can use unwrap() here because we already know that a value
        //       with the key `n` exist from the code line above.
        let count = *freqs.get(n).unwrap();

        // find the number with the highest frequency, so far.
        if count > max {
            max = count;
            mo = *n;
        }
    }
    mo

    // study this for an alternative implementations (optional):
    // https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
}
