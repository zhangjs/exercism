#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = (1..=num / 2).filter(|x| num % x == 0).sum();

    if sum > num {
        Some(Classification::Abundant)
    } else if sum == num {
        Some(Classification::Perfect)
    } else {
        Some(Classification::Deficient)
    }
}
