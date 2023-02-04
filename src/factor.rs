pub fn factor(input: i128) -> Vec<i128>{
    // Initialize factors Vector
    let mut factors : Vec<i128> = Vec::new();

    let mut i : i128 = 1;

    // Check till i is less than or equal to square root n
    while i*i <= number{
        if number % i == 0 {
            factors.push(i);

            // to prevent duplication, if number is perfect square
            if i*i != number {
                factors.push(number / i);
            }
        }
        i+=1;
    }

    // It is generally useful to sort the vector
    // And it will not affect our time complexity,
    // Because logarithmic time is much less than square root time complexity

    factors.sort();

    // Return the factors Vector as answer
    return factors;
}
