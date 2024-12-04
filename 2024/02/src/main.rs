const SAFETY_THRESHOLD: i32 = 3;

struct Report {
    values: Vec<i32>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Where input file????");
    let reports: Vec<Report> = get_reports(&input);
    let part_one = evaluate_safe_reports(&reports, false);
    let part_two = evaluate_safe_reports(&reports, true);
    println!("Solution part 1: {part_one} safe reports");
    println!("Solution part 2: {part_two} safe reports after dampening.")
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

fn evaluate_safe_reports(reports: &Vec<Report>, use_dampener: bool) -> usize {
    reports
        .iter()
        .map(|report| is_safe(&report.values, use_dampener))
        .filter(|result| *result == true)
        .count()
}

fn is_safe(values: &Vec<i32>, use_dampener: bool) -> bool {
    let ascending = values.get(0) < values.get(1);
    for i in 1..values.len() {
        let cur = values[i];
        let prev = values[i - 1];
        if ((cur - prev).abs() > SAFETY_THRESHOLD)
            || ((cur - prev < 0) == ascending)
            || (cur == prev)
        {
            return if use_dampener {
                problem_dampener(values)
            } else {
                false
            };
        }
    }
    true
}

fn problem_dampener(values: &Vec<i32>) -> bool {
    for i in 0..values.len() {
        let mut dampened_values = values.clone();
        dampened_values.remove(i);
        if is_safe(&dampened_values, false) {
            return true;
        }
    }
    false
}
