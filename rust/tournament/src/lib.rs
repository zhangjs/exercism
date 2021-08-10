use std::collections::HashMap;

#[derive(Default)]
struct TeamInfo {
    win: u32,
    draw: u32,
    loss: u32,
}

impl TeamInfo {
    fn plays(&self) -> u32 {
        self.win + self.draw + self.loss
    }
    fn points(&self) -> u32 {
        3 * self.win + self.draw
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, TeamInfo> = HashMap::new();

    for line in match_results.lines() {
        let v: Vec<_> = line.split(';').collect();
        if v.len() != 3 {
            continue;
        }

        match v[2] {
            "win" => {
                teams.entry(v[0]).or_default().win += 1;
                teams.entry(v[1]).or_default().loss += 1;
            }
            "loss" => {
                teams.entry(v[0]).or_default().loss += 1;
                teams.entry(v[1]).or_default().win += 1;
            }
            "draw" => {
                teams.entry(v[0]).or_default().draw += 1;
                teams.entry(v[1]).or_default().draw += 1;
            }
            _ => (),
        }
    }

    let mut vp: Vec<_> = teams.into_iter().collect();
    vp.sort_by(|a, b| {
        if a.1.points() == b.1.points() {
            a.0.cmp(&b.0)
        } else {
            b.1.points().cmp(&a.1.points())
        }
    });

    let head = "Team                           | MP |  W |  D |  L |  P".to_string();
    std::iter::once(head)
        .chain(vp.iter().map(|(name, t)| {
            format!(
                "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                name,
                t.plays(),
                t.win,
                t.draw,
                t.loss,
                t.points()
            )
        }))
        .collect::<Vec<_>>()
        .join("\n")
}
