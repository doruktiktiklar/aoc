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

pub fn d2_p1() {
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

pub fn d2_p2() {
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

pub fn d3_p1() {
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

pub fn d3_p2() {
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

pub fn d4_p1() {
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

#[derive(Debug)]
struct CrateAction {
    nr_of_crates: usize,
    from_stack: usize,
    to_stack: usize,
}
fn move_crates_single(crates_vec: &mut Vec<Vec<u8>>, actions: &Vec<CrateAction>) {
    for cur_action in actions {
        for _ in 0..cur_action.nr_of_crates {
            let tmp0 = crates_vec[cur_action.from_stack].pop().unwrap();
            crates_vec[cur_action.to_stack].push(tmp0);
        }
    }
    println!("Stack heads: ");
    for a_stack in crates_vec {
        print!(
            "{}",
            char::from_u32(*a_stack.last().unwrap() as u32).unwrap()
        );
    }
    println!();
}

fn move_crates_multiple(crates_vec: &mut Vec<Vec<u8>>, actions: &Vec<CrateAction>) {
    for cur_action in actions {
        let from_stack_len = crates_vec[cur_action.from_stack].len();
        let mut stack_part: Vec<u8> = crates_vec[cur_action.from_stack]
            .drain((from_stack_len - cur_action.nr_of_crates)..from_stack_len)
            .collect();
        crates_vec[cur_action.to_stack].append(&mut stack_part);
        //for _ in 0..cur_action.nr_of_crates {
        //    let tmp0 = crates_vec[cur_action.from_stack].pop().unwrap();
        //    crates_vec[cur_action.to_stack].push(tmp0);
        //}
    }
    println!("Stack heads: ");
    for a_stack in crates_vec {
        print!(
            "{}",
            char::from_u32(*a_stack.last().unwrap() as u32).unwrap()
        );
    }
    println!();
}
fn get_crate_stacks_actions(in_content: String) -> (Vec<Vec<u8>>, Vec<CrateAction>) {
    #[derive(Debug)]
    enum ParseState {
        CRATES,
        ACTIONS,
    }
    let mut crate_vec: Vec<Vec<u8>> = Vec::new();
    let mut cr_actions_vec: Vec<CrateAction> = Vec::new();
    let mut cur_parse_state = ParseState::CRATES;

    let first_line_length = in_content.lines().next().unwrap().len();
    let nr_of_stacks = first_line_length / 4 + 1;
    crate_vec.resize(nr_of_stacks, Vec::new());

    let anchor_str_move = "move ";
    let anchor_str_from = " from ";
    let anchor_str_to = " to ";

    for cur_line in in_content.lines() {
        match cur_parse_state {
            ParseState::CRATES => {
                if cur_line.len() == 0 {
                    cur_parse_state = ParseState::ACTIONS;
                } else {
                    let mut crate_ind = 1;
                    while crate_ind < first_line_length {
                        let cur_char = cur_line.as_bytes()[crate_ind];
                        if cur_char != b' ' && !(cur_char >= b'0' && cur_char <= b'9') {
                            crate_vec[crate_ind / 4].push(cur_char);
                        }
                        crate_ind += 4;
                    }
                }
            }
            ParseState::ACTIONS => {
                let move_pos = cur_line.find(anchor_str_move).unwrap();
                let from_pos = cur_line.find(anchor_str_from).unwrap();
                let to_pos = cur_line.find(anchor_str_to).unwrap();

                let mut from_stack: usize = cur_line[from_pos + anchor_str_from.len()..to_pos]
                    .parse()
                    .unwrap();
                from_stack -= 1;
                let mut to_stack: usize = cur_line[to_pos + anchor_str_to.len()..].parse().unwrap();
                to_stack -= 1;
                cr_actions_vec.push(CrateAction {
                    nr_of_crates: cur_line[move_pos + anchor_str_move.len()..from_pos]
                        .parse()
                        .unwrap(),
                    from_stack,
                    to_stack,
                });
            }
        }
    }
    for char_vec in crate_vec.iter_mut() {
        char_vec.reverse();
    }
    (crate_vec, cr_actions_vec)
}

pub fn d5_p1() {
    //let in_content = std::fs::read_to_string("./in/d5p1/tmp_in").unwrap();
    let in_content = std::fs::read_to_string("./in/d5p1/input").unwrap();
    let mut crates_and_actions = get_crate_stacks_actions(in_content);
    move_crates_single(&mut crates_and_actions.0, &crates_and_actions.1);
}

pub fn d5_p2() {
    //let in_content = std::fs::read_to_string("./in/d5p1/tmp_in").unwrap();
    let in_content = std::fs::read_to_string("./in/d5p1/input").unwrap();
    let mut crates_and_actions = get_crate_stacks_actions(in_content);
    move_crates_multiple(&mut crates_and_actions.0, &crates_and_actions.1);
}
