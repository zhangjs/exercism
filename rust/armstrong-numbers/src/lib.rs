pub fn is_armstrong_number(num: u32) -> bool {
    let mut v = Vec::new();
    let mut d = num;

    loop {
        v.push(d % 10);

        d = d / 10;

        if d == 0 {
            break;
        }
    }

    v.iter().map(|x| x.pow(v.len() as u32)).sum::<u32>() == num
}
