use std::fs;
use std::io::BufRead;

const SAFETY_THRESHOLD: i32 = 3;

struct Report {
    values: Vec<i32>,
}

fn main() {
    let input = fs::read_to_string("resources/input.txt").expect("Where input file????");
    let mut num_safe_reports = 0;
    let reports: Vec<Report> = get_reports(&input);

    for report in reports {
        if (evaluate_safety(&report.values)) {
            num_safe_reports = num_safe_reports + 1;
        } 
    }

    println!("Safe reports: {num_safe_reports}")
}

fn get_reports(input: &String) -> Vec<Report> {
    let mut reports: Vec<Report> = vec![];
    for line in input.lines() {
        reports.push(Report {
            values: line
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect(),
        });
    }
    reports
}

fn evaluate_safety(values: &Vec<i32>) -> bool {
    let ascending = values.get(0) < values.get(1);
    for i in 1..values.len() {
        let cur = values[i];
        let prev = values[i - 1];
        if ((cur - prev).abs() > SAFETY_THRESHOLD)
            || ((cur - prev < 0) == ascending)
            || (cur == prev)
        {
            return false;
        }
    }
    true
}
