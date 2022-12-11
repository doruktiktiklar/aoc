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
    //simulate_monkey_actions_p2(&mut monkeys_states_p2, 2);
    simulate_monkey_actions_p2(&mut monkeys_states_p2, 10000);
}
