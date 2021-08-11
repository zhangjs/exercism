#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut sols: Vec<Vec<(u8, u8)>> = vec![];
    let mut moves: u8 = 1;

    sols.push(vec![(capacity_1, 0), (0, capacity_2)]);
    match start_bucket {
        &Bucket::One => {
            sols.push(vec![(0, capacity_2)]);
            sols.push(vec![(capacity_1, 0)]);
        }
        &Bucket::Two => {
            sols.push(vec![(capacity_1, 0)]);
            sols.push(vec![(0, capacity_2)]);
        }
    }

    while let Some(sv) = sols.iter().last() {
        if let Some(s) = sv.iter().find(|s| s.0 == goal) {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: s.1,
            });
        }
        if let Some(s) = sv.iter().find(|s| s.1 == goal) {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: s.0,
            });
        }

        let mut next_moves: Vec<(u8, u8)> = sv
            .iter()
            .map(|s| {
                let mut v = vec![(0, s.1), (s.0, 0), (capacity_1, s.1), (s.0, capacity_2)];
                let min = (s.0 + s.1).min(capacity_1);
                v.push((min, s.0 + s.1 - min));
                let min = (s.0 + s.1).min(capacity_2);
                v.push((s.0 + s.1 - min, min));

                v
            })
            .flatten()
            .collect();
        next_moves.sort();
        next_moves.dedup();

        let pre_moves: Vec<&(u8, u8)> = sols.iter().flatten().collect();
        let next: Vec<(u8, u8)> = next_moves
            .into_iter()
            .filter(|s| !pre_moves.iter().any(|m| s.0 == m.0 && s.1 == m.1))
            .collect();

        if next.is_empty() {
            break;
        }

        moves += 1;
        sols.push(next);
    }

    None
}
