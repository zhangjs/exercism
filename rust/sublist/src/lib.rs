#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len1 = first_list.len();
    let len2 = second_list.len();

    if len1 > len2 && (0..=len1 - len2).any(|i| first_list[i..i + len2].eq(second_list)) {
        return Comparison::Superlist;
    } else if len1 == len2 && first_list.eq(second_list) {
        return Comparison::Equal;
    } else if len1 < len2 && (0..=len2 - len1).any(|i| second_list[i..i + len1].eq(first_list)) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
