use std::collections::HashSet;
use utils::read_data;

fn update_freq(freq: &mut i32, line: &str) {
    let sign = &line[..1];
    let number = &line[1..].parse::<i32>().unwrap();

    match sign {
        "+" => *freq += number,
        "-" => *freq -= number,
        _ => panic!("unknown sign: {}", sign),
    }
}

pub fn solve_1(input: &str) -> i32 {
    let data = read_data(input);

    let mut freq = 0;

    for line in data.lines() {
        update_freq(&mut freq, line);
    }

    freq
}

pub fn solve_2(input: &str) -> i32 {
    let data = read_data(input);
    let mut freq = 0;
    let mut seen_freqs: HashSet<i32> = HashSet::new();

    loop {
        let lines = data.lines();

        for line in lines {
            update_freq(&mut freq, line);

            if seen_freqs.contains(&freq) {
                return freq;
            }

            seen_freqs.insert(freq);
        }
    }
}
