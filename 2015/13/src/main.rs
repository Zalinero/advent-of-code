use std::collections::HashSet;

struct Relationship {
    name: String,
    happiness_change: i32,
    target_name: String,
}
fn main() {
    let input = std::fs::read_to_string("input.txt".to_string()).expect("where input file???");
    let mut result = parse_input(input);
    let mut perms = HashSet::new();
    make_permutations(&mut perms, &mut result.0, 0);

    let mut happiness_results = vec![];
    for perm in perms {
        happiness_results.push(happiness_computer(&result.1, &perm));
    }

    println!("Max happiness (part 2): {:#?}", happiness_results.iter().max().unwrap());
}
fn parse_input(input: String) -> (Vec<String>, Vec<Relationship>) {
    let mut human_entities: HashSet<String> = HashSet::new();
    let mut relationships: Vec<Relationship> = Vec::new();
    for entry in input.lines() {
        let mut words: Vec<_> = entry.split_whitespace().collect();
        words[10] = &words[10][0..words[10].len() - 1];
        let identifier: String = words[0].to_string();
        let good_or_evil = words[2];
        let target = words[10];
        let mut happy_units: i32 = words[3].parse().expect("everything is a number right?");
        if good_or_evil == "lose" {
            happy_units *= -1;
        }
        let rel = Relationship {
            name: identifier.to_string(),
            happiness_change: happy_units,
            target_name: target.to_string(),
        };
        human_entities.insert(identifier);
        relationships.push(rel);
    }
    (Vec::from_iter(human_entities), relationships)
}

fn make_permutations(
    permutations: &mut HashSet<Vec<String>>,
    people: &mut Vec<String>,
    idx: usize,
) {
    if permutations.len() >= (1..people.len() + 1).product() {
        return;
    }
    if idx == people.len() {
        permutations.insert(people.clone());
        return;
    }
    for i in 0..people.len() {
        people.swap(i, idx);
        make_permutations(permutations, people, idx + 1);
        people.swap(i, idx);
    }
}

fn happiness_computer(relationships: &Vec<Relationship>, arrangement: &Vec<String>) -> i32 {
    let mut total_happiness: Vec<i32>;

    total_happiness = arrangement
        .windows(2)
        .map(|win| {
            relationships
                .iter()
                .filter(|rel| {
                    (rel.name == win[0] && rel.target_name == win[1])
                        || (rel.name == win[1] && rel.target_name == win[0])
                })
                .map(|rel| rel.happiness_change)
        })
        .flatten()
        .collect();
    let mut the_rest: Vec<i32> = relationships
        .iter()
        .filter(|rel| {
            (rel.name == arrangement[0] && rel.target_name == arrangement[arrangement.len() - 1])
                || (rel.name == arrangement[arrangement.len() - 1]
                    && rel.target_name == arrangement[0])
        })
        .map(|rel| rel.happiness_change)
        .collect();
    total_happiness.append(&mut the_rest);
    total_happiness.iter().sum()
}
