use std::collections::HashMap;
use utils::read_data;

pub fn solve_1(input: &str) -> i32 {
    let data = read_data(input);
    let mut repeated_2s = 0;
    let mut repeated_3s = 0;

    for box_id in data.lines() {
        let mut id_char_counter: HashMap<char, u32> = HashMap::new();
        let mut should_count_2 = true;
        let mut should_count_3 = true;

        for c in box_id.chars() {
            *id_char_counter.entry(c).or_insert(0) += 1;
        }

        for count in id_char_counter.values() {
            match count {
                2 => if should_count_2 {
                    repeated_2s += 1;
                    should_count_2 = false;
                },
                3 => if should_count_3 {
                    repeated_3s += 1;
                    should_count_3 = false;
                },
                _ => (),
            }
        }
    }

    repeated_2s * repeated_3s
}

fn is_differ_by_1(id_1: &str, id_2: &str) -> Option<String> {
    if id_1.len() != id_2.len() {
        return None;
    }

    let mut n_diff = 0;
    let mut common = String::new();

    for (c_1, c_2) in id_1.chars().zip(id_2.chars()) {
        if n_diff > 1 {
            return None;
        }

        if c_1 != c_2 {
            n_diff += 1;
        } else {
            common.push(c_1);
        }
    }

    Some(common)
}

pub fn solve_2(input: &str) -> String {
    let data = read_data(input);
    let data_vec: Vec<&str> = data.lines().collect();

    for (i, id_a) in data_vec.iter().enumerate() {
        for id_b in data_vec[i + 1..].iter() {
            match is_differ_by_1(id_a, id_b) {
                Some(common) => return common,
                None => (),
            }
        }
    }

    "Cannot find".to_string()
}
