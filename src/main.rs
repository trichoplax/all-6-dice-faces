use itertools::Itertools;

fn main() {
    let maximum_n = 10;

    for value in 1..=maximum_n {
        println!("{} : {}", value, probability_of_all_6_faces(value));
    }
}

fn probability_of_all_6_faces(value: u64) -> f64 {
    let denominator = 6_f64.powf(value as f64);
    let multi_product = (1..=value).map(|_i| 1..=6).multi_cartesian_product();
    let six_faced_results = multi_product.filter(|x| (1..=6).all(|y| x.contains(&y)));

    six_faced_results.count() as f64 / denominator as f64
}
