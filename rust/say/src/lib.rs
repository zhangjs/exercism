static LOW_ARRAY: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static TY_ARRAY: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

static SCALE_ARRAY: [&str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn twenty_encode(n: u64) -> String {
    assert!(n < 20);
    LOW_ARRAY[n as usize].to_string()
}

fn hundred_encode(n: u64) -> String {
    assert!(n < 100);
    match n {
        0..=19 => twenty_encode(n),
        20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => TY_ARRAY[(n / 10 - 2) as usize].to_string(),
        _ => TY_ARRAY[(n / 10 - 2) as usize].to_string() + "-" + &twenty_encode(n % 10),
    }
}

fn thousand_encode(n: u64) -> String {
    assert!(n < 1000);

    let div = n / 100;
    let rem = n % 100;

    match (div, rem) {
        (0, r) => hundred_encode(r),
        (d, 0) => twenty_encode(d) + " hundred",
        (d, r) => twenty_encode(d) + " hundred " + &hundred_encode(r),
    }
}

pub fn encode(n: u64) -> String {
    let mut v = vec![];
    let mut div = n;

    if div == 0 {
        return "zero".to_string();
    }

    let mut scale = 0;
    while div > 0 {
        if div % 1000 != 0 {
            if scale > 0 {
                v.push(SCALE_ARRAY[scale - 1].to_string());
            }
            v.push(thousand_encode(div % 1000));
        }
        div = div / 1000;
        scale += 1;
    }

    v.reverse();
    v.join(" ")
}
