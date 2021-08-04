pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
    //(1..=n).fold(0, |x, y| x + y).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).sum::<u32>()
    //(1..=n).fold(0, |x, y| x + y * y)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
