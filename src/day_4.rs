use chrono::NaiveDateTime;
use regex::Regex;
use utils::read_data;

#[derive(Debug)]
enum Action {
    ShiftBegin,
    WakeUp,
    Sleep,
}

#[derive(Debug)]
struct Record {
    time: NaiveDateTime,
    guard_id: i32,
    action: Action,
}

impl Record {
    pub fn from_str (input_str: &str) -> Record {
        let re = Regex::new(r"\[(?P<time>.+)\]\s+(?P<action_desc>.+)").unwrap();
        let re_guard_id = Regex::new(r"Guard #(?P<guard_id>\d+)").unwrap();
        let caps = re.captures(input_str).unwrap();

        let mut guard_id = 0;

        let action = match &caps["action_desc"] {
            "falls asleep" => Action::Sleep,
            "wakes up" => Action::WakeUp,
            _ => {
                let caps_guard = re_guard_id.captures(&caps["action_desc"]).unwrap();
                guard_id = caps_guard["guard_id"].parse::<i32>().unwrap();
                Action::ShiftBegin
            },
        };

        Record {
            time: NaiveDateTime::parse_from_str(&caps["time"], "%Y-%m-%d %H:%M").unwrap(),
            guard_id: guard_id,
            action: action
        }
    }
}


pub fn solve_1 (input: &str) -> i32 {
    let data = read_data(input);

    for line in data.lines() {
        let r = Record::from_str(line);
        println!("{:?}", r);
    }
    100
}
