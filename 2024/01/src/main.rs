use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Where input file????");
    let mut total_distance: i32 = 0;
    let mut similarity_score: i32 = 0;
    let mut list_1: Vec<i32> = vec![];
    let mut list_2: Vec<i32> = vec![];

    for line in input.lines() {
        let mut line_locs = line.split_whitespace();
        let get_id = |loc_str: &str| loc_str.parse::<i32>()
            .expect("Error parsing location");
        list_1.push(get_id(line_locs.next().unwrap()));
        list_2.push(get_id(line_locs.next().unwrap()));
    }

    assert_eq!(list_1.len(), list_2.len(), "Lists aren't of same length!");

    list_1.sort();
    list_2.sort();

    for i in 0..list_1.len() {
        let loc_1 = list_1.get(i).unwrap();
        total_distance = total_distance + calc_distance(loc_1, list_2.get(i).unwrap());
        similarity_score = similarity_score + calc_sim_score(loc_1, &list_2);
    }

    println!("Total distance is {total_distance}");
    println!("Similarity score is {similarity_score}")
}

fn calc_distance(loc_1: &i32, loc_2: &i32) -> i32 {
    (loc_1 - loc_2).abs()
}

fn calc_sim_score(loc_1: &i32, list_2: &Vec<i32> ) -> i32 {
    let mut sim_score = 0;
    for loc in list_2.iter() {
        if loc == loc_1 {
            sim_score = sim_score + loc;
        }
    }
    sim_score
}
