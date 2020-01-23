use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let result_header =  "Team                           | MP |  W |  D |  L |  P".to_owned();

    let results: HashMap<&str, (i32, i32, i32)> = HashMap::new();

    let result_body = match_results.lines().fold(results, |mut acc, line| {
        let m: Vec<&str> =line.split(';').collect();
        let home_team = acc.entry(m[0]).or_insert((0,0,0));

        match m[2] {
            "win" => {
                home_team.0 += 1;
            },
            "draw" => {
                home_team.1 += 1;
            }
            "loss" => {
                home_team.2 += 1;
            }
            _ => {}
        };
        let guest_team = acc.entry(m[1]).or_insert((0,0,0));

        match m[2] {
            "win" => {
                guest_team.2 += 1;
            },
            "draw" => {
                guest_team.1 += 1;
            }
            "loss" => {
                guest_team.0 += 1;
            }
            _ => {}
        };
        dbg!(acc)
    }).into_iter().collect::<Vec<(&str, (i32, i32, i32))>>().sort_by(|a, b| { (a.0 * 3 + a.1)b)}).into_iter().map(|(key, value)| {
        format!("{0:<29}  |  {1} |  {2} |  {3} |  {4} |  {5}", key, (value.0 + value.1 + value.2), value.0, value.1, value.2, value.0 * 3 + value.1)
    }).collect::<Vec<String>>().join("\n");

    [result_header, result_body].join("\n")
}
