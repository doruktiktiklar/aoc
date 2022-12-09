use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{prelude, Read};

fn read_file_to_string(fname: &str) -> Result<String, std::io::Error> {
    let mut fhandle = File::open(fname)?;
    let mut fcontents = String::new();
    fhandle.read_to_string(&mut fcontents);
    Ok(fcontents)
}

fn get_max_score(in_cals_text: &str) -> u64 {
    let mut bin_heap = BinaryHeap::<u64>::new();
    let mut current_total: u64 = 0;
    for cur_line in in_cals_text.lines() {
        if cur_line.len() != 0 {
            let parsed_num: u64 = cur_line.parse().unwrap();
            current_total += parsed_num;
        } else {
            bin_heap.push(current_total);
            current_total = 0;
        }
    }
    bin_heap.push(current_total);
    let mut topthreemax = bin_heap.pop().unwrap();
    topthreemax += bin_heap.pop().unwrap();
    topthreemax += bin_heap.pop().unwrap();
    topthreemax
}

fn d2_p1() {
    let in_content = std::fs::read_to_string("./in/d2p1/input").unwrap();
    //let in_content = std::fs::read_to_string("./in/d2p1/tmp_in").unwrap();
    let mut total_score = 0;
    for cur_line in in_content.lines() {
        if (cur_line.chars().nth(0).unwrap() == 'A' && cur_line.chars().nth(2).unwrap() == 'X')
            || (cur_line.chars().nth(0).unwrap() == 'B' && cur_line.chars().nth(2).unwrap() == 'Y')
            || (cur_line.chars().nth(0).unwrap() == 'C' && cur_line.chars().nth(2).unwrap() == 'Z')
        {
            total_score += 3;
        } else if (cur_line.chars().nth(0).unwrap() == 'A'
            && cur_line.chars().nth(2).unwrap() == 'Y')
            || (cur_line.chars().nth(0).unwrap() == 'B' && cur_line.chars().nth(2).unwrap() == 'Z')
            || (cur_line.chars().nth(0).unwrap() == 'C' && cur_line.chars().nth(2).unwrap() == 'X')
        {
            total_score += 6;
        }
        if cur_line.chars().nth(2).unwrap() == 'X' {
            total_score += 1;
        } else if cur_line.chars().nth(2).unwrap() == 'Y' {
            total_score += 2;
        } else if cur_line.chars().nth(2).unwrap() == 'Z' {
            total_score += 3;
        }
    }
    println!("RPS normal strategy score: {}", total_score);
}

fn d2_p2() {
    let in_content = std::fs::read_to_string("./in/d2p1/input").unwrap();
    //let in_content = std::fs::read_to_string("./in/d2p1/tmp_in").unwrap();
    let mut total_score = 0;
    for cur_line in in_content.lines() {
        let my_act_encr = cur_line.chars().nth(2).unwrap();
        let their_act_encr = cur_line.chars().nth(0).unwrap();
        if my_act_encr == 'X' {
            if their_act_encr == 'A' {
                total_score += 3;
            } else if their_act_encr == 'B' {
                total_score += 1;
            } else if their_act_encr == 'C' {
                total_score += 2;
            }
        }
        if my_act_encr == 'Y' {
            if their_act_encr == 'A' {
                total_score += 1 + 3;
            } else if their_act_encr == 'B' {
                total_score += 2 + 3;
            } else if their_act_encr == 'C' {
                total_score += 3 + 3;
            }
        }
        if my_act_encr == 'Z' {
            if their_act_encr == 'A' {
                total_score += 2 + 6;
            } else if their_act_encr == 'B' {
                total_score += 3 + 6;
            } else if their_act_encr == 'C' {
                total_score += 1 + 6;
            }
        }
    }
    println!("RPS normal strategy score: {}", total_score);
}

fn get_priority(&in_char: &char) -> u32 {
    if in_char as u32 >= 'a' as u32 && in_char as u32 <= 'z' as u32 {
        in_char as u32 - 'a' as u32 + 1
    } else if in_char as u32 >= 'A' as u32 && in_char as u32 <= 'Z' as u32 {
        in_char as u32 - 'A' as u32 + 27
    } else {
        panic!("Cannot assign priority, given char: |{}|", in_char);
    }
}

fn d3_p1() {
    let in_content = std::fs::read_to_string("./in/d3p1/input").unwrap();
    //let in_content = std::fs::read_to_string("./in/d3p1/tmp_in").unwrap();

    let mut total_item_priorities = 0;
    for cur_line in in_content.lines() {
        let mut first_compartment_items = BTreeSet::<char>::new();
        let mut added_to_second_compartment_before = BTreeSet::<char>::new();
        if cur_line.chars().count() % 2 != 0 {
            panic!("Length of the line is odd");
        }
        let midpoint = cur_line.chars().count() / 2;
        for (ind, cur_char) in cur_line.chars().enumerate() {
            if ind < midpoint {
                first_compartment_items.insert(cur_char);
            } else {
                if first_compartment_items.contains(&cur_char)
                    && !added_to_second_compartment_before.contains(&cur_char)
                {
                    let found_priority = get_priority(&cur_char);
                    added_to_second_compartment_before.insert(cur_char);
                    total_item_priorities += found_priority;
                }
            }
        }
    }

    println!(
        "Total incorrectly placed item priorities: {}",
        total_item_priorities
    );
}

fn d3_p2() {
    let in_content = std::fs::read_to_string("./in/d3p1/input").unwrap();
    //let in_content = std::fs::read_to_string("./in/d3p1/tmp_in").unwrap();
    enum MEMB {
        FIRST,
        SECOND,
        THIRD,
    }
    let mut group_member: MEMB = MEMB::FIRST;
    let mut total_item_priorities = 0;

    let mut first_member_items = BTreeSet::<char>::new();
    let mut second_member_items = BTreeSet::<char>::new();
    let mut third_member_items = BTreeSet::<char>::new();
    for cur_line in in_content.lines() {
        match group_member {
            MEMB::FIRST => {
                first_member_items.extend(cur_line.chars());
                group_member = MEMB::SECOND;
            }
            MEMB::SECOND => {
                second_member_items.extend(cur_line.chars());
                group_member = MEMB::THIRD;
            }
            MEMB::THIRD => {
                third_member_items.extend(cur_line.chars());
                let mut intersection_set: BTreeSet<char> =
                    BTreeSet::intersection(&first_member_items, &second_member_items)
                        .map(|&x| x)
                        .collect();
                intersection_set = third_member_items
                    .intersection(&intersection_set)
                    .map(|&x| x)
                    .collect();
                for elf_item in &intersection_set {
                    total_item_priorities += get_priority(elf_item);
                }
                first_member_items.clear();
                second_member_items.clear();
                third_member_items.clear();
                group_member = MEMB::FIRST;
            }
        }
    }

    println!(
        "Total item priorities of 3-group intersections: {}",
        total_item_priorities
    );
}

fn d4_p1() {
    let in_content = std::fs::read_to_string("./in/d4p1/input").unwrap();
    //let in_content = std::fs::read_to_string("./in/d4p1/tmp_in").unwrap();

    let mut nr_of_full_containment = 0;
    for cur_line in in_content.lines() {
        let first_dash_position = cur_line.find('-').unwrap();
        let comma_position = cur_line[first_dash_position..].find(',').unwrap();
        let second_dash_position = cur_line[first_dash_position + comma_position..]
            .find('-')
            .unwrap();
        //println!(
        //    "first dash: {} , comma: {} , second dash: {}",
        //    first_dash_position, comma_position, second_dash_position
        //);

        let first_inter_begin: u32 = cur_line[..first_dash_position].parse().unwrap();
        let first_inter_end: u32 = cur_line
            [first_dash_position + 1..first_dash_position + comma_position]
            .parse()
            .unwrap();
        let second_inter_begin: u32 = cur_line[first_dash_position + comma_position + 1
            ..first_dash_position + comma_position + second_dash_position]
            .parse()
            .unwrap();

        let second_inter_end: u32 = cur_line
            [first_dash_position + comma_position + second_dash_position + 1..]
            .parse()
            .unwrap();
        if first_inter_begin <= second_inter_begin && first_inter_end >= second_inter_end {
            nr_of_full_containment += 1;
        } else if first_inter_begin >= second_inter_begin && first_inter_end <= second_inter_end {
            nr_of_full_containment += 1;
        }
    }
    println!("Number of full containment: {}", nr_of_full_containment);
}

fn d5_p1() {
    let in_content = std::fs::read_to_string("./in/d5p1/tmp_in").unwrap();
    //let in_content = std::fs::read_to_string("./in/d5p1/input").unwrap();
    #[derive(Debug)]
    enum ParseState {
        CRATES,
        ACTIONS,
    }
    #[derive(Debug)]
    struct CrateAction {
        nr_of_crates: u32,
        from_stack: u32,
        to_stack: u32,
    }
    let mut crate_vec: Vec<Vec<char>> = Vec::new();
    let mut cr_actions_vec: Vec<CrateAction> = Vec::new();
    let mut cur_parse_state = ParseState::CRATES;

    let first_line_length = in_content.lines().next().unwrap().chars().count();
    let nr_of_stacks = first_line_length / 4;
    crate_vec.resize(nr_of_stacks, Vec::new());
    for cur_line in in_content.lines() {
        match cur_parse_state {
            ParseState::CRATES => {
                if cur_line == "\n" {
                    cur_parse_state = ParseState::ACTIONS;
                } else {
                    let mut crate_ind = 2;
                    while crate_ind < first_line_length {
                        if cur_line.chars().nth(crate_ind).unwrap() != ' ' {
                            crate_vec[crate_ind / 4].push(cur_line.chars().nth(crate_ind).unwrap());
                        }
                        crate_ind += 4;
                    }
                }
            }
            ParseState::ACTIONS => {}
        }
    }
}

fn main() {
    d4_p1();
}
