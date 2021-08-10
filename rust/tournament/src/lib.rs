use std::{cmp::Ordering, collections::HashMap};

#[derive(Default)]
struct TeamInfo {
    play: u32,
    win: u32,
    loss: u32,
    draw: u32,
    points: u32,
}

impl TeamInfo {
    fn win(&mut self) {
        self.play += 1;
        self.win += 1;
        self.points += 3;
    }
    fn loss(&mut self) {
        self.play += 1;
        self.loss += 1;
    }
    fn draw(&mut self) {
        self.play += 1;
        self.draw += 1;
        self.points += 1;
    }
}

fn team_cmp(team1: (&String, &TeamInfo), team2: (&String, &TeamInfo)) -> Ordering {
    if team1.1.points == team2.1.points {
        team1.0.cmp(&team2.0)
    } else {
        team2.1.points.cmp(&team1.1.points)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, TeamInfo> = HashMap::new();

    for line in match_results.lines() {
        let v: Vec<_> = line.split(';').collect();
        if v.len() != 3 {
            continue;
        }

        match v[2] {
            "win" => {
                teams.entry(v[0].to_string()).or_default().win();
                teams.entry(v[1].to_string()).or_default().loss();
            }
            "loss" => {
                teams.entry(v[0].to_string()).or_default().loss();
                teams.entry(v[1].to_string()).or_default().win();
            }
            "draw" => {
                teams.entry(v[0].to_string()).or_default().draw();
                teams.entry(v[1].to_string()).or_default().draw();
            }
            _ => (),
        }
    }

    let mut vp: Vec<_> = teams.iter().collect();
    vp.sort_by(|&a, &b| team_cmp(a, b));

    let head = "Team                           | MP |  W |  D |  L |  P".to_string();
    std::iter::once(head)
        .chain(vp.iter().map(|&(name, t)| {
            format!(
                "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                name, t.play, t.win, t.draw, t.loss, t.points
            )
        }))
        .collect::<Vec<_>>()
        .join("\n")
}
