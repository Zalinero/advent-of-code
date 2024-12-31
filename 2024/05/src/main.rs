#[derive(Debug, PartialEq)]
struct Rule {
    first: usize,
    second: usize,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Where input file????");
    let split_input = input.split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<Rule> = read_the_rules(split_input[0]);
    let updates: Vec<Vec<usize>> = get_the_news(split_input[1]);
    let result = make_the_steps(&rules, &updates);
    println!("Result one: {}, Result 2: {}", result.0, result.1);
}

fn read_the_rules(input_rules: &str) -> Vec<Rule> {
    input_rules
        .lines()
        .map(|rule_str| {
            let rule_split: Vec<&str> = rule_str.split("|").collect();
            Rule {
                first: rule_split[0].parse::<usize>().unwrap(),
                second: rule_split[1].parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn get_the_news(input_updates: &str) -> Vec<Vec<usize>> {
    input_updates
        .lines()
        .map(|line| {
            let page: Vec<usize> = line
                .split(",")
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            return page;
        })
        .collect()
}

fn make_the_steps(rules: &Vec<Rule>, updates: &Vec<Vec<usize>>) -> (i32, i32) {
    let mut result1 = 0;
    let mut result2 = 0;
    let mut misshapen_ones: Vec<&Vec<usize>> = vec![];
    for update in updates {
        if update_valid(update, rules) {
            result1 = result1 + get_middle(update) as i32;
        } else {
            misshapen_ones.push(update);
        }
    }
    println!("found {} bad updates", misshapen_ones.len());
    let mut fixed = 1;
    for update in misshapen_ones {
        result2 = result2 + get_middle(&fix_it(update,rules)) as i32;
        println!("fixed {} updates", fixed);
        fixed += 1;
    }

    (result1, result2)
}

fn update_valid(update: &Vec<usize>, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if !update_valid_rule(update, rule) {
            return false;
        }
    }
    true
}

fn update_valid_rule(update: &Vec<usize>, rule: &Rule) -> bool {
        let fst_idx = update.iter().position(|&e| e == rule.first);
        let snd_idx = update.iter().position(|&e| e == rule.second);
        match fst_idx {
            Some(fst) => snd_idx.is_none_or(|snd| snd > fst),
            _ => true,
        }
    }

    fn get_middle(update: &Vec<usize>) -> usize {
        let middle = update[(update.len() / 2)];
        middle
    }

    fn fix_it(update: &Vec<usize>, rules: &Vec<Rule>) -> Vec<usize> {
        let mut update = update.clone();
        if (update_valid(&update, rules)) {
            return update;
        }
        for rule in rules {
                let fst = update.iter().position(|r| *r==rule.first);
                let snd = update.iter().position(|r| *r==rule.second);
                match (fst, snd) {
                    (Some(f), Some(s)) =>
                        {
                            if f > s {
                                update.swap(f,s);
                                return fix_it(&update, rules)
                            }
                        },
                    _ => continue
                }
            }
        update
    }
