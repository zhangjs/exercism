/// What should the type of _function be?
pub fn map<T, U, F: FnMut(T) -> U>(input: Vec<T>, mut f: F) -> Vec<U> {
    let mut v = Vec::with_capacity(input.len());

    for value in input {
        v.push(f(value))
    }

    v
}
