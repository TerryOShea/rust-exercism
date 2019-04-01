fn sum_of_multiple(limit: u32, factor: u32) -> u32 {
    let biggest_factor_multiple = if limit % factor > 0 {
        limit - (limit % factor)
    } else {
        limit - factor
    };
    (biggest_factor_multiple * (biggest_factor_multiple + factor)) / (2 * factor)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let initial_sum: u32 = factors
        .iter()
        .filter(|&f| *f != 0)
        .map(|f| sum_of_multiple(limit, *f))
        .sum();

    let mut product_factors = vec![];
    let num_factors = factors.len();

    for i in 0..(num_factors - 1) {
        for j in (i + 1)..num_factors {
            product_factors.push(factors[i] * factors[j]);
        }
    }

    println!("{:?}", product_factors);

    let product_factors_sum: u32 = product_factors
        .iter()
        .map(|f| sum_of_multiple(limit, *f))
        .sum();
    initial_sum - product_factors_sum
}
