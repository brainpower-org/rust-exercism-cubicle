use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let result_header = "Team                           | MP |  W |  D |  L |  P".to_owned();

    if match_results.len() == 0 {
        return result_header;
    }

    let results: HashMap<&str, (i32, i32, i32)> = HashMap::new();

    let mut result_body = match_results.lines().fold(results, |mut acc, line| {
        let m: Vec<&str> = line.split(';').collect();
        let home_team = acc.entry(m[0]).or_insert((0, 0, 0));

        match m[2] {
            "win" => {
                home_team.0 += 1;
            }
            "draw" => {
                home_team.1 += 1;
            }
            "loss" => {
                home_team.2 += 1;
            }
            _ => {}
        };
        let guest_team = acc.entry(m[1]).or_insert((0, 0, 0));

        match m[2] {
            "win" => {
                guest_team.2 += 1;
            }
            "draw" => {
                guest_team.1 += 1;
            }
            "loss" => {
                guest_team.0 += 1;
            }
            _ => {}
        };
        dbg!(acc)
    }).into_iter()
        .collect::<Vec<(&str, (i32, i32, i32))>>();

    result_body.sort_by(|a, b| {
        ((b.1).0 * 3 + (b.1).1).cmp(&((a.1).0 * 3 + (a.1).1)).then((a.0).cmp(b.0))
    });

    let result_body_string = result_body.into_iter()
        .map(|(key, value)| {
            format!("{0:<29}  |  {1} |  {2} |  {3} |  {4} |  {5}", key, (value.0 + value.1 + value.2), value.0, value.1, value.2, value.0 * 3 + value.1)
        }).collect::<Vec<String>>().join("\n");

    [result_header, result_body_string].join("\n")
}
