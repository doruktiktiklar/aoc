use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
enum MonkeyOperation {
    ADD,
    MUL,
}

#[derive(Debug, PartialEq, Clone)]
enum MonkeyOperand {
    INPUT,
    CONST(u64),
}

#[derive(Debug)]
struct MonkeyState {
    items: Vec<u64>,
    operation: MonkeyOperation,
    operand1: MonkeyOperand,
    operand2: MonkeyOperand,
    divide_by: u64,
    cond_true_target: usize,
    cond_false_target: usize,
}

fn apply_monkey_operation(in_worry_level: u64, monkey_state: &MonkeyState) -> u64 {
    if monkey_state.operation == MonkeyOperation::ADD {
        if monkey_state.operand1 == MonkeyOperand::INPUT
            && monkey_state.operand2 == MonkeyOperand::INPUT
        {
            return in_worry_level + in_worry_level;
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand1 {
            return in_worry_level + val;
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand2 {
            return in_worry_level + val;
        }
    } else if monkey_state.operation == MonkeyOperation::MUL {
        if monkey_state.operand1 == MonkeyOperand::INPUT
            && monkey_state.operand2 == MonkeyOperand::INPUT
        {
            return in_worry_level * in_worry_level;
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand1 {
            return in_worry_level * val;
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand2 {
            return in_worry_level * val;
        }
    }
    panic!("Failed to apply the operation");
}

#[derive(Debug)]
struct MonkeyStateP2 {
    items: Vec<Vec<(MonkeyOperation, MonkeyOperand)>>,
    operation: MonkeyOperation,
    operand1: MonkeyOperand,
    operand2: MonkeyOperand,
    divide_by: u64,
    cond_true_target: usize,
    cond_false_target: usize,
}

fn apply_monkey_operation_p2(
    in_worry_level: &mut Vec<(MonkeyOperation, MonkeyOperand)>,
    monkey_state: &MonkeyStateP2,
) {
    if monkey_state.operation == MonkeyOperation::ADD {
        if monkey_state.operand1 == MonkeyOperand::INPUT
            && monkey_state.operand2 == MonkeyOperand::INPUT
        {
            in_worry_level.push((MonkeyOperation::ADD, MonkeyOperand::INPUT));
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand1 {
            in_worry_level.push((MonkeyOperation::ADD, MonkeyOperand::CONST(val)));
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand2 {
            in_worry_level.push((MonkeyOperation::ADD, MonkeyOperand::CONST(val)));
        }
    } else if monkey_state.operation == MonkeyOperation::MUL {
        if monkey_state.operand1 == MonkeyOperand::INPUT
            && monkey_state.operand2 == MonkeyOperand::INPUT
        {
            in_worry_level.push((MonkeyOperation::MUL, MonkeyOperand::INPUT));
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand1 {
            in_worry_level.push((MonkeyOperation::MUL, MonkeyOperand::CONST(val)));
        } else if let MonkeyOperand::CONST(val) = monkey_state.operand2 {
            in_worry_level.push((MonkeyOperation::MUL, MonkeyOperand::CONST(val)));
        }
    }
}

fn check_division_by(
    operation_chain: &Vec<(MonkeyOperation, MonkeyOperand)>,
    mod_operand: u64,
) -> u64 {
    let mut cur_num = 1;
    for cur_op in operation_chain {
        if cur_op.0 == MonkeyOperation::ADD {
            if cur_op.1 == MonkeyOperand::INPUT {
                cur_num = cur_num + cur_num % mod_operand;
            } else if let MonkeyOperand::CONST(val) = cur_op.1 {
                cur_num = ((val % mod_operand) + cur_num) % mod_operand;
            }
        } else if cur_op.0 == MonkeyOperation::MUL {
            if cur_op.1 == MonkeyOperand::INPUT {
                cur_num = (cur_num * cur_num) % mod_operand;
            } else if let MonkeyOperand::CONST(val) = cur_op.1 {
                cur_num = ((val % mod_operand) * cur_num) % mod_operand;
            }
        }
    }
    return cur_num % mod_operand;
}

fn convert_to_p2_repr(in_state: &MonkeyState) -> MonkeyStateP2 {
    let mut new_items: Vec<Vec<(MonkeyOperation, MonkeyOperand)>> = Vec::new();
    for item in in_state.items.iter() {
        new_items.push(vec![(MonkeyOperation::MUL, MonkeyOperand::CONST(*item))]);
    }
    return MonkeyStateP2 {
        items: new_items,
        operation: in_state.operation.clone(),
        operand1: in_state.operand1.clone(),
        operand2: in_state.operand2.clone(),
        divide_by: in_state.divide_by,
        cond_true_target: in_state.cond_true_target,
        cond_false_target: in_state.cond_false_target,
    };
}

fn parse_monkey_operation(in_expr: &str) -> (MonkeyOperation, MonkeyOperand, MonkeyOperand) {
    if in_expr.contains("*") {
        let operands: Vec<&str> = in_expr.split(" * ").collect();
        if operands[0] == "old" && operands[1] == "old" {
            return (
                MonkeyOperation::MUL,
                MonkeyOperand::INPUT,
                MonkeyOperand::INPUT,
            );
        } else if operands[0] == "old" {
            let operand1: u64 = operands[1].parse().unwrap();
            return (
                MonkeyOperation::MUL,
                MonkeyOperand::INPUT,
                MonkeyOperand::CONST(operand1),
            );
        } else if operands[1] == "old" {
            let operand0: u64 = operands[0].parse().unwrap();
            return (
                MonkeyOperation::MUL,
                MonkeyOperand::CONST(operand0),
                MonkeyOperand::INPUT,
            );
        }
    } else if in_expr.contains("+") {
        let operands: Vec<&str> = in_expr.split(" + ").collect();
        if operands[0] == "old" && operands[1] == "old" {
            return (
                MonkeyOperation::ADD,
                MonkeyOperand::INPUT,
                MonkeyOperand::INPUT,
            );
        } else if operands[0] == "old" {
            let operand1: u64 = operands[1].parse().unwrap();
            return (
                MonkeyOperation::ADD,
                MonkeyOperand::INPUT,
                MonkeyOperand::CONST(operand1),
            );
        } else if operands[1] == "old" {
            let operand0: u64 = operands[0].parse().unwrap();
            return (
                MonkeyOperation::ADD,
                MonkeyOperand::CONST(operand0),
                MonkeyOperand::INPUT,
            );
        }
    }
    panic!(
        "Failed to parse the monkye operation, expression was: {}",
        in_expr
    );
}
fn parse_monkey_states(in_content: &str) -> Vec<MonkeyState> {
    let mut line_iter = in_content.lines();
    let mut maybe_cur_line = line_iter.next();
    const ITEMS_START: &str = "  Starting items: ";
    const OPERATION_START: &str = "  Operation: new = ";
    const TEST_START: &str = "  Test: divisible by ";
    const COND_TRUE_START: &str = "    If true: throw to monkey ";
    const COND_FALSE_START: &str = "    If false: throw to monkey ";

    let mut monkeys_starting_states: Vec<MonkeyState> = Vec::new();
    while let Some(line) = maybe_cur_line {
        if line.starts_with("Monkey ") {
            let items_line = line_iter.next().unwrap();
            let item_nums: Vec<u64> = items_line[ITEMS_START.len()..]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
            let operation_line = line_iter.next().unwrap();
            let operation_expression = &operation_line[OPERATION_START.len()..];
            let test_start_line = line_iter.next().unwrap();
            let division_by: u64 = test_start_line[TEST_START.len()..].parse().unwrap();
            let cond_true_line = line_iter.next().unwrap();
            let cond_true_target: usize = cond_true_line[COND_TRUE_START.len()..].parse().unwrap();
            let cond_false_line = line_iter.next().unwrap();
            let cond_false_target: usize =
                cond_false_line[COND_FALSE_START.len()..].parse().unwrap();
            let operation_parameters = parse_monkey_operation(operation_expression);
            monkeys_starting_states.push(MonkeyState {
                items: item_nums,
                operation: operation_parameters.0,
                operand1: operation_parameters.1,
                operand2: operation_parameters.2,
                divide_by: division_by,
                cond_true_target,
                cond_false_target,
            })
        }
        maybe_cur_line = line_iter.next();
    }
    monkeys_starting_states
}

fn simulate_monkey_actions_p1(monkey_states: &mut Vec<MonkeyState>, nr_of_states: usize) {
    let mut monkey_activeness: Vec<u64> = vec![0; monkey_states.len()];
    for cur_step in 0..nr_of_states {
        for cur_monkey_ind in 0..monkey_states.len() {
            while monkey_states[cur_monkey_ind].items.len() > 0 {
                monkey_activeness[cur_monkey_ind] += 1;
                let cur_item = monkey_states[cur_monkey_ind].items.remove(0);
                let cur_worry_level =
                    apply_monkey_operation(cur_item, &monkey_states[cur_monkey_ind]) / 3;
                if cur_worry_level % monkey_states[cur_monkey_ind].divide_by == 0 {
                    let target_monk: usize = monkey_states[cur_monkey_ind].cond_true_target;
                    monkey_states[target_monk].items.push(cur_worry_level);
                } else {
                    let target_monk: usize = monkey_states[cur_monkey_ind].cond_false_target;
                    monkey_states[target_monk].items.push(cur_worry_level);
                }
            }
        }
    }
    monkey_activeness.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top2active: Vec<u64> = monkey_activeness.iter().take(2).map(|&x| x).collect();
    println!("Monkey business score: {:?}", top2active[0] * top2active[1]);
}

fn simulate_monkey_actions_p2(monkey_states: &mut Vec<MonkeyStateP2>, nr_of_states: usize) {
    let mut monkey_activeness: Vec<u64> = vec![0; monkey_states.len()];
    for cur_step in 0..nr_of_states {
        for cur_monkey_ind in 0..monkey_states.len() {
            while monkey_states[cur_monkey_ind].items.len() > 0 {
                monkey_activeness[cur_monkey_ind] += 1;
                let mut cur_item = monkey_states[cur_monkey_ind].items.remove(0);
                apply_monkey_operation_p2(&mut cur_item, &monkey_states[cur_monkey_ind]);
                if check_division_by(&cur_item, monkey_states[cur_monkey_ind].divide_by) == 0 {
                    let target_monk: usize = monkey_states[cur_monkey_ind].cond_true_target;
                    monkey_states[target_monk].items.push(cur_item);
                } else {
                    let target_monk: usize = monkey_states[cur_monkey_ind].cond_false_target;
                    monkey_states[target_monk].items.push(cur_item);
                }
            }
        }
    }
    monkey_activeness.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top2active: Vec<u64> = monkey_activeness.iter().take(2).map(|&x| x).collect();
    println!("Monkey business score: {:?}", top2active[0] * top2active[1]);
}

pub fn d11_p1() {
    //let in_content = std::fs::read_to_string("./in/d11p1/tmp_in0").unwrap();
    let in_content = std::fs::read_to_string("./in/d11p1/input").unwrap();
    let mut monkeys_states = parse_monkey_states(&in_content);
    simulate_monkey_actions_p1(&mut monkeys_states, 20);
}

pub fn d11_p2() {
    //let in_content = std::fs::read_to_string("./in/d11p1/tmp_in0").unwrap();
    let in_content = std::fs::read_to_string("./in/d11p1/input").unwrap();
    let mut monkeys_states = parse_monkey_states(&in_content);
    let mut monkeys_states_p2: Vec<MonkeyStateP2> = monkeys_states
        .iter()
        .map(|x| convert_to_p2_repr(x))
        .collect();
    simulate_monkey_actions_p2(&mut monkeys_states_p2, 10000);
}

#[derive(Debug)]
struct HeightMapInfo {
    map: Vec<Vec<u8>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

fn parse_height_map(in_content: &str) -> HeightMapInfo {
    let mut parsed_map: Vec<Vec<u8>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);
    for (i, cur_line) in in_content.lines().enumerate() {
        parsed_map.push(Vec::new());
        for (j, cur_char) in cur_line.bytes().enumerate() {
            if cur_char == b'S' {
                parsed_map[i].push(b'a');
                start_pos = (i, j);
            } else if cur_char == b'E' {
                parsed_map[i].push(b'z');
                end_pos = (i, j);
            } else {
                parsed_map[i].push(cur_char);
            }
        }
    }
    HeightMapInfo {
        map: parsed_map,
        start_x: start_pos.0,
        start_y: start_pos.1,
        end_x: end_pos.0,
        end_y: end_pos.1,
    }
}

fn get_p1_valid_neighbors(in_map: &Vec<Vec<u8>>, in_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut valid_neighbors: Vec<(usize, usize)> = Vec::new();
    if in_pos.1 != 0 && in_map[in_pos.0][in_pos.1 - 1] <= in_map[in_pos.0][in_pos.1] {
        valid_neighbors.push((in_pos.0, in_pos.1 - 1));
    }
    if in_pos.0 != 0 && in_map[in_pos.0 - 1][in_pos.1] <= in_map[in_pos.0][in_pos.1] {
        valid_neighbors.push((in_pos.0 - 1, in_pos.1));
    }
    if in_pos.0 != in_map.len() - 1 && in_map[in_pos.0 + 1][in_pos.1] <= in_map[in_pos.0][in_pos.1]
    {
        valid_neighbors.push((in_pos.0 + 1, in_pos.1));
    }
    if in_pos.1 != in_map[0].len() - 1
        && in_map[in_pos.0][in_pos.1 + 1] <= in_map[in_pos.0][in_pos.1]
    {
        valid_neighbors.push((in_pos.0, in_pos.1 + 1));
    }
    if in_pos.1 != 0 && in_map[in_pos.0][in_pos.1 - 1] > in_map[in_pos.0][in_pos.1] {
        if (in_map[in_pos.0][in_pos.1 - 1] - in_map[in_pos.0][in_pos.1]) <= 1 {
            valid_neighbors.push((in_pos.0, in_pos.1 - 1));
        }
    }
    if in_pos.0 != 0 && in_map[in_pos.0 - 1][in_pos.1] > in_map[in_pos.0][in_pos.1] {
        if (in_map[in_pos.0 - 1][in_pos.1] - in_map[in_pos.0][in_pos.1]) <= 1 {
            valid_neighbors.push((in_pos.0 - 1, in_pos.1));
        }
    }
    if in_pos.0 != in_map.len() - 1 && in_map[in_pos.0 + 1][in_pos.1] > in_map[in_pos.0][in_pos.1] {
        if (in_map[in_pos.0 + 1][in_pos.1] - in_map[in_pos.0][in_pos.1]) <= 1 {
            valid_neighbors.push((in_pos.0 + 1, in_pos.1));
        }
    }
    if in_pos.1 != in_map[0].len() - 1
        && in_map[in_pos.0][in_pos.1 + 1] > in_map[in_pos.0][in_pos.1]
    {
        if (in_map[in_pos.0][in_pos.1 + 1] - in_map[in_pos.0][in_pos.1]) <= 1 {
            valid_neighbors.push((in_pos.0, in_pos.1 + 1));
        }
    }
    valid_neighbors
    //vec![valid_neighbors
    //    .iter()
    //    .map(|x| (x, in_map[x.0][x.1]))
    //    .max_by(|&x, &y| y.1.cmp(&x.1))
    //    .map(|x| x.0)
    //    .unwrap()
    //    .clone()]
}

fn get_p1_nr_of_steps(in_map_info: &HeightMapInfo) -> Option<u32> {
    let mut visited_set: BTreeSet<(usize, usize)> = BTreeSet::new();
    //let mut next_visit_deque: VecDeque<(usize, usize, u32, BTreeSet<(usize, usize)>)> =
    //    VecDeque::new();
    let mut next_visit_deque: VecDeque<(usize, usize, u32)> = VecDeque::new();
    let mut has_reached_to_target = false;
    let mut path_length = None;
    next_visit_deque.push_back((in_map_info.start_x, in_map_info.start_y, 0));
    while !has_reached_to_target && !next_visit_deque.is_empty() {
        let cur_node = next_visit_deque.pop_front().unwrap();
        //let cur_node = next_visit_deque.pop_back().unwrap();
        //println!("Inspecting: {:?}", cur_node);
        visited_set.insert((cur_node.0, cur_node.1));
        if cur_node.0 == in_map_info.end_x && cur_node.1 == in_map_info.end_y {
            path_length = Some(cur_node.2);
            has_reached_to_target = true;
        } else {
            let valid_neighbors =
                get_p1_valid_neighbors(&in_map_info.map, (cur_node.0, cur_node.1));
            let mut next_targets: VecDeque<(usize, usize, u32)> = valid_neighbors
                .iter()
                .filter(|x| !visited_set.contains(&x))
                //.filter(|x| {
                //    let mut long_path = false;
                //    if let Some(val) = path_length {
                //        if val < cur_node.3.len() as u32 {
                //            long_path = true
                //        }
                //    }
                //    !cur_node.3.contains(&x) && !long_path
                //})
                //.map(|&x| {
                //    let mut path = cur_node.3.clone();
                //    path.insert((x.0, x.1));
                //    (x.0, x.1, cur_node.2 + 1, path)
                //})
                .map(|&x| (x.0, x.1, cur_node.2 + 1))
                .collect();
            next_visit_deque.append(&mut next_targets);
            const VEC_LIMIT: usize = 100000000;
            let mut pruned_next_visit_deque: VecDeque<(usize, usize, u32)> = VecDeque::new();
            if next_visit_deque.len() > VEC_LIMIT {
                panic!("next visit dequeue has more than {} elements", VEC_LIMIT);
            }
            for next_visit_node in next_visit_deque {
                if !visited_set.contains(&(next_visit_node.0, next_visit_node.1)) {
                    pruned_next_visit_deque.push_back(next_visit_node.clone());
                }
            }
            next_visit_deque = pruned_next_visit_deque;
        }
    }
    //println!("path: {:?}", found_path);
    path_length
}

fn p1_nr_of_steps(
    map: &Vec<Vec<u8>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> Option<u32> {
    let mut visited_set: BTreeSet<(usize, usize)> = BTreeSet::new();
    //let mut next_visit_deque: VecDeque<(usize, usize, u32, BTreeSet<(usize, usize)>)> =
    //    VecDeque::new();
    let mut next_visit_deque: VecDeque<(usize, usize, u32)> = VecDeque::new();
    let mut has_reached_to_target = false;
    let mut path_length = None;
    next_visit_deque.push_back((start_x, start_y, 0));
    while !has_reached_to_target && !next_visit_deque.is_empty() {
        let cur_node = next_visit_deque.pop_front().unwrap();
        //let cur_node = next_visit_deque.pop_back().unwrap();
        //println!("Inspecting: {:?}", cur_node);
        visited_set.insert((cur_node.0, cur_node.1));
        if cur_node.0 == end_x && cur_node.1 == end_y {
            path_length = Some(cur_node.2);
            has_reached_to_target = true;
        } else {
            let mut valid_neighbors = get_p1_valid_neighbors(&map, (cur_node.0, cur_node.1));
            let mut next_targets: VecDeque<(usize, usize, u32)> = valid_neighbors
                .iter()
                .filter(|x| !visited_set.contains(&x))
                //.filter(|x| {
                //    let mut long_path = false;
                //    if let Some(val) = path_length {
                //        if val < cur_node.3.len() as u32 {
                //            long_path = true
                //        }
                //    }
                //    !cur_node.3.contains(&x) && !long_path
                //})
                //.map(|&x| {
                //    let mut path = cur_node.3.clone();
                //    path.insert((x.0, x.1));
                //    (x.0, x.1, cur_node.2 + 1, path)
                //})
                .map(|&x| (x.0, x.1, cur_node.2 + 1))
                .collect();
            next_visit_deque.append(&mut next_targets);
            const VEC_LIMIT: usize = 10000000;
            if next_visit_deque.len() > VEC_LIMIT {
                panic!("next visit dequeue has more than {} elements", VEC_LIMIT);
            }
            let mut pruned_next_visit_deque: VecDeque<(usize, usize, u32)> = VecDeque::new();
            for next_visit_node in next_visit_deque {
                if !visited_set.contains(&(next_visit_node.0, next_visit_node.1)) {
                    pruned_next_visit_deque.push_back(next_visit_node.clone());
                }
            }
            next_visit_deque = pruned_next_visit_deque;
        }
    }
    //println!("path: {:?}", found_path);
    path_length
}
fn one_hop_targets(
    in_map_info: &HeightMapInfo,
    in_pos: (usize, usize),
    target_char: u8,
) -> Vec<((usize, usize), (usize, usize))> {
    let mut valid_neighbors: Vec<((usize, usize), (usize, usize))> = Vec::new();
    if in_pos.1 != 0 && in_map_info.map[in_pos.0][in_pos.1 - 1] == target_char {
        valid_neighbors.push((in_pos, (in_pos.0, in_pos.1 - 1)));
    }
    if in_pos.0 != 0 && in_map_info.map[in_pos.0 - 1][in_pos.1] == target_char {
        valid_neighbors.push((in_pos, (in_pos.0 - 1, in_pos.1)));
    }
    if in_pos.0 != in_map_info.map.len() - 1
        && in_map_info.map[in_pos.0 + 1][in_pos.1] == target_char
    {
        valid_neighbors.push((in_pos, (in_pos.0 + 1, in_pos.1)));
    }
    if in_pos.1 != in_map_info.map[0].len() - 1
        && in_map_info.map[in_pos.0][in_pos.1 + 1] == target_char
    {
        valid_neighbors.push((in_pos, (in_pos.0, in_pos.1 + 1)));
    }
    valid_neighbors
}

fn necessary_subpaths(map_info: &HeightMapInfo) -> Vec<Vec<((usize, usize), (usize, usize))>> {
    let mut found_necessary_paths: Vec<Vec<((usize, usize), (usize, usize))>> = Vec::new();
    for from_char in b'a'..b'y' {
        let mut paths_from_this_char: Vec<((usize, usize), (usize, usize))> = Vec::new();
        for (i, cur_row) in map_info.map.iter().enumerate() {
            for (j, &cur_char) in cur_row.iter().enumerate() {
                if cur_char == from_char {
                    let mut cur_targets = one_hop_targets(&map_info, (i, j), from_char + 1);
                    paths_from_this_char.append(&mut cur_targets);
                }
            }
        }
        println!(
            "For {}, necessary paths: {:?}",
            from_char as char, paths_from_this_char
        );
        found_necessary_paths.push(paths_from_this_char);
    }

    return found_necessary_paths;
}

fn is_around(loc1: (usize, usize), loc2: (usize, usize)) -> bool {
    if loc1.0 == loc2.0 {
        if loc1.1 == loc2.1 {
            return true;
        }
        if loc1.1 != 0 && loc1.1 - 1 == loc2.1 {
            return true;
        } else if loc1.1 + 1 == loc2.1 {
            return true;
        }
    }
    if loc1.1 == loc2.1 {
        if loc1.0 != 0 && loc1.0 - 1 == loc2.0 {
            return true;
        } else if loc1.0 + 1 == loc2.0 {
            return true;
        }
    }
    false
}

fn path_from_subpaths(
    map_info: &HeightMapInfo,
    subpaths: &Vec<Vec<((usize, usize), (usize, usize))>>,
) {
    //let mut subpath_lengths: Vec<Vec<Vec<Option<u32>>>> = Vec::new();
    //for same_height_subpaths_ind in 0..subpaths.len()-1 {
    //    let from_subpaths = &subpaths[same_height_subpaths_ind];
    //    let to_subpaths = &subpaths[same_height_subpaths_ind+1];
    //    let mut cur_path_lengths: Vec<Vec<Option<u32>>> = vec![vec![None; to_subpaths.len()]; from_subpaths.len()];
    //    for (i, src_path) in from_subpaths.iter().enumerate() {
    //        for (j,dst_path) in to_subpaths.iter().enumerate() {
    //            println!("Will check the path between: {:?} and {:?}", src_path.1 , dst_path.0);

    //                cur_path_lengths[i][j] = get_p1_nr_of_steps(&HeightMapInfo {
    //                    map: map_info.map.clone(), start_x: src_path.1.0, start_y: src_path.1.1,
    //                    end_x: dst_path.0.0, end_y: dst_path.0.1 })
    //        }
    //    }
    //    subpath_lengths.push(cur_path_lengths);
    //}
    //println!("Built a 3d path lengts vector");
    let mut found_path_lens: Vec<u32> = Vec::new();
    let mut subpath_index = vec![0usize; subpaths.len()];
    while let Ok(()) = vec2d_next_index(&subpaths, &mut subpath_index) {
        let first_subpath = &subpaths[0][*subpath_index.first().unwrap()];
        let last_subpath = &subpaths[subpaths.len() - 1][*subpath_index.last().unwrap()];
        let begin_len = p1_nr_of_steps(
            &map_info.map,
            map_info.start_x,
            map_info.start_y,
            first_subpath.0 .0,
            first_subpath.0 .1,
        );
        let end_len = p1_nr_of_steps(
            &map_info.map,
            last_subpath.1 .0,
            last_subpath.1 .1,
            map_info.end_x,
            map_info.end_y,
        );
        println!("Considering : {:?}", subpath_index);
        if begin_len.is_some() && end_len.is_some() {
            let mut total_len = begin_len.unwrap() + end_len.unwrap();
            if first_subpath.0 .0 != first_subpath.1 .0 || first_subpath.0 .1 != first_subpath.1 .1
            {
                total_len += 1;
            }
            let mut path_exist = true;
            for cur_ind in 0..subpaths.len() - 1 {
                let src_subpath = &subpaths[cur_ind][subpath_index[cur_ind]];
                let dst_subpath = &subpaths[cur_ind + 1][subpath_index[cur_ind + 1]];
                if dst_subpath.0 .0 != dst_subpath.1 .0 || dst_subpath.0 .1 != dst_subpath.1 .1 {
                    total_len += 1;
                }
                let sub_len = p1_nr_of_steps(
                    &map_info.map,
                    src_subpath.1 .0,
                    src_subpath.1 .1,
                    dst_subpath.0 .0,
                    dst_subpath.0 .1,
                );
                if sub_len.is_none() {
                    path_exist = false;
                    break;
                }
                total_len += sub_len.unwrap();
            }
            if path_exist {
                found_path_lens.push(total_len);
            }
        }
    }
    //println!("Subpath lengths: {:?}", subpath_lengths);
    println!("Found path lengths: {:?}", found_path_lens);
}

fn vec2d_next_index<T>(in_vec: &Vec<Vec<T>>, index_vec: &mut Vec<usize>) -> Result<(), String> {
    if in_vec.len() != index_vec.len() {
        return Err(String::from("2d vector and index length do not match"));
    }
    for i in 0..in_vec.len() {
        if in_vec[i].len() <= index_vec[i] {
            return Err(String::from("Got an index vec with invalid size"));
        }
    }
    let mut carry = 1;
    for i in (0..in_vec.len()).rev() {
        index_vec[i] += carry;
        if index_vec[i] >= in_vec[i].len() {
            index_vec[i] = 0;
        } else {
            carry = 0;
            break;
        }
    }
    if carry == 1 {
        return Err(String::from("End of the iterator"));
    }
    Ok(())
}

pub fn d12_p1() {
    //let in_content = std::fs::read_to_string("./in/d12p1/tmp_in0").unwrap();
    let in_content = std::fs::read_to_string("./in/d12p1/input").unwrap();
    let map_info = parse_height_map(&in_content);
    let found_path = p1_nr_of_steps(
        &map_info.map,
        map_info.start_x,
        map_info.start_y,
        map_info.end_x,
        map_info.end_y,
    );
    println!("found path length: {:?}", found_path);
    //let subpaths = necessary_subpaths(&map_info);
    //path_from_subpaths(&map_info, &subpaths);
    //let path_length = get_p1_nr_of_steps(&map_info);
    // println!("path length: {}", path_length);
}

fn path_len_from_selected_height(map_info: &HeightMapInfo, selected_height: u8) -> Option<u32> {
    let start_points: Vec<(usize, usize)> = Vec::new();
    let mut min_path_len = u32::max_value();
    for i in 0..map_info.map.len() {
        for j in 0..map_info.map[i].len() {
            if map_info.map[i][j] == selected_height {
                let found_path_res =
                    p1_nr_of_steps(&map_info.map, i, j, map_info.end_x, map_info.end_y);
                if let Some(found_path_len) = found_path_res {
                    if found_path_len < min_path_len {
                        min_path_len = found_path_len;
                    }
                }
            }
        }
    }
    Some(min_path_len)
}

pub fn d12_p2() {
    let in_content = std::fs::read_to_string("./in/d12p1/input").unwrap();
    let map_info = parse_height_map(&in_content);
    let found_path_len = path_len_from_selected_height(&map_info, b'a');
    println!("found path length: {:?}", found_path_len);
}
