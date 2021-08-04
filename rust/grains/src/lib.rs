use core::panic;

pub fn square(s: u32) -> u64 {
    if s >= 1 && s <= 64 {
        return 1 << (s - 1);
    }

    panic!("Square must be between 1 and 64");
}

pub fn total() -> u64 {
    std::u64::MAX
}
