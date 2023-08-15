use std::collections::BTreeSet;

pub fn d6_p1() {
    let in_content = std::fs::read_to_string("./in/d6p1/input").unwrap(); // for testing quickly "./in/d6p1/tmp_in4"
    let mut end_index = 4;
    let mut cur_four_bytes = Vec::<u8>::from(&in_content[0..end_index]);
    let mut uniques: BTreeSet<u8> = BTreeSet::new();
    uniques.extend(cur_four_bytes.iter());
    while uniques.len() != 4 && end_index < in_content.as_bytes().len() {
        cur_four_bytes.remove(0);
        cur_four_bytes.push(in_content.as_bytes()[end_index]);
        uniques.clear();
        uniques.extend(cur_four_bytes.iter());
        end_index += 1;
    }
    println!("Marker found after parsing the index {}", end_index);
}

pub fn d6_p2() {
    let in_content = std::fs::read_to_string("./in/d6p1/input").unwrap(); // for testing quickly "./in/d6p1/tmp_in4"
    const WINDOW_SIZE: usize = 14;
    let mut end_index = WINDOW_SIZE;
    let mut cur_four_bytes = Vec::<u8>::from(&in_content[0..end_index]);
    let mut uniques: BTreeSet<u8> = BTreeSet::new();
    uniques.extend(cur_four_bytes.iter());
    while uniques.len() != WINDOW_SIZE && end_index < in_content.as_bytes().len() {
        cur_four_bytes.remove(0);
        cur_four_bytes.push(in_content.as_bytes()[end_index]);
        uniques.clear();
        uniques.extend(cur_four_bytes.iter());
        end_index += 1;
    }
    println!(
        "Message start marker found after parsing the index {}",
        end_index
    );
}

#[derive(Debug)]
struct ElfFile {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct ElfDir {
    name: String,
    files: Vec<ElfFile>,
    inner_dirs: Vec<usize>,
    outer_dir: Option<usize>,
}

fn build_dir_structure(in_content: &str) -> Vec<ElfDir> {
    let mut all_dirs: Vec<ElfDir> = Vec::new();
    all_dirs.push(ElfDir {
        name: String::from("/"),
        files: Vec::new(),
        inner_dirs: Vec::new(),
        outer_dir: None,
    });
    let mut cur_dir_index: usize = 0;
    let mut lines_iter = in_content.lines();
    let mut maybe_line = lines_iter.next();
    while let Some(cur_line) = maybe_line {
        if cur_line.starts_with("$ ls") {
            let mut maybe_ls_line = lines_iter.next();
            while maybe_ls_line.is_some() && !maybe_ls_line.unwrap().starts_with("$ ") {
                let node_features: Vec<&str> = maybe_ls_line.unwrap().split(" ").collect();
                let first_feature = node_features[0];
                let parsed_node_name = node_features[1];
                if first_feature == "dir" {
                    let new_dir = ElfDir {
                        name: String::new()
                            + &all_dirs[cur_dir_index].name
                            + "/"
                            + parsed_node_name,
                        files: Vec::new(),
                        inner_dirs: Vec::new(),
                        outer_dir: Some(cur_dir_index),
                    };
                    all_dirs.push(new_dir);
                    let new_dir_index = all_dirs.len() - 1;
                    all_dirs[cur_dir_index].inner_dirs.push(new_dir_index);
                } else {
                    let parsed_node_size: u32 = first_feature.parse().unwrap();
                    all_dirs[cur_dir_index].files.push(ElfFile {
                        name: parsed_node_name.to_string(),
                        size: parsed_node_size,
                    });
                }
                maybe_ls_line = lines_iter.next();
            }
            maybe_line = maybe_ls_line;
        } else if cur_line.starts_with("$ cd /") {
            cur_dir_index = 0;
            maybe_line = lines_iter.next();
        } else if cur_line.starts_with("$ cd ..") {
            cur_dir_index = all_dirs[cur_dir_index].outer_dir.unwrap();
            maybe_line = lines_iter.next();
        } else if cur_line.starts_with("$ cd") {
            let parsed_name = &cur_line["$ cd ".len()..];
            let mut new_dir_index = cur_dir_index;
            for &dir_index in &all_dirs[cur_dir_index].inner_dirs {
                let mut dir_name: &str = &all_dirs[dir_index].name;
                dir_name = &dir_name[dir_name.rfind("/").unwrap() + 1..];
                if dir_name == parsed_name {
                    new_dir_index = dir_index;
                }
            }
            if new_dir_index == cur_dir_index {
                panic!("Cannot cd into: {}", parsed_name);
            }
            cur_dir_index = new_dir_index;

            maybe_line = lines_iter.next();
        } else {
            panic!("Cannot parse the input");
        }
    }

    for cur_dir in all_dirs.iter_mut() {
        if cur_dir.name.starts_with("//") {
            cur_dir.name.remove(0);
        }
    }
    all_dirs
}

fn get_total_file_size(in_dir: &ElfDir) -> u32 {
    let mut total_size = 0;
    for file in &in_dir.files {
        total_size += file.size;
    }
    total_size
}

fn get_directory_sizes(dir_structure: Vec<ElfDir>) -> Vec<u32> {
    let mut directory_sizes: Vec<u32> = vec![0; dir_structure.len()];
    let mut is_dir_size_calculated: Vec<bool> = vec![false; dir_structure.len()];

    let mut cur_target_dir = 0;
    while is_dir_size_calculated.iter().any(|&x| !x) {
        if is_dir_size_calculated[cur_target_dir] {
            for i in (0..directory_sizes.len()).rev() {
                if !is_dir_size_calculated[i] {
                    cur_target_dir = i;
                }
            }
        } else {
            let mut size_of_sub_dirs = 0;
            let mut sub_dir_iter = dir_structure[cur_target_dir].inner_dirs.iter();
            let mut cur_sub_dir = sub_dir_iter.next();
            while cur_sub_dir.is_some() && is_dir_size_calculated[*cur_sub_dir.unwrap()] {
                size_of_sub_dirs += directory_sizes[*cur_sub_dir.unwrap()];
                cur_sub_dir = sub_dir_iter.next();
            }
            if cur_sub_dir.is_some() && !is_dir_size_calculated[*cur_sub_dir.unwrap()] {
                cur_target_dir = *cur_sub_dir.unwrap();
            } else {
                let mut cur_size = 0;
                cur_size += get_total_file_size(&dir_structure[cur_target_dir]);
                cur_size += size_of_sub_dirs;
                directory_sizes[cur_target_dir] = cur_size;
                is_dir_size_calculated[cur_target_dir] = true;
            }
        }
    }

    directory_sizes
}

pub fn d7_p1() {
    let in_content = std::fs::read_to_string("./in/d7p1/input").unwrap(); // for testing quickly "./in/d7p1/tmp_in"
    let all_dirs = build_dir_structure(&in_content);
    let sum_of_under_100000: u32 = get_directory_sizes(all_dirs)
        .iter()
        .filter(|&&x| x <= 100000)
        .sum();
    println!(
        "Sum of directory sizes smaller than 100000: {:?}",
        sum_of_under_100000
    );
}

pub fn d7_p2() {
    const TOTAL_DISK_SPACE: u32 = 70000000;
    const NEEDED_UNUSED_SPACE: u32 = 30000000;
    let in_content = std::fs::read_to_string("./in/d7p1/input").unwrap(); // for testing quickly "./in/d7p1/tmp_in"
    let all_dirs = build_dir_structure(&in_content);
    let current_directory_sizes = get_directory_sizes(all_dirs);
    let currently_used: u32 = current_directory_sizes[0];
    let extra_space_needed = NEEDED_UNUSED_SPACE - (TOTAL_DISK_SPACE - currently_used);
    let minimum_sized_deletion_target_dir = current_directory_sizes
        .iter()
        .filter(|&&x| x >= extra_space_needed)
        .min()
        .unwrap();

    println!(
        "Size of the deletion target directory: {}",
        minimum_sized_deletion_target_dir
    );
}

pub fn d8_p1() {
    let in_content = std::fs::read_to_string("./in/d8p1/input").unwrap(); // for testing quickly "./in/d8p1/tmp_in"
    let mut tree_grid: Vec<Vec<char>> = Vec::new();
    for cur_line in in_content.lines() {
        tree_grid.push(Vec::new());
        for cur_char in cur_line.chars() {
            tree_grid.last_mut().unwrap().push(cur_char);
        }
    }
    let total_rows = tree_grid.len();
    let total_cols = tree_grid[0].len();
    let mut nr_of_visible = 2 * total_cols + 2 * total_rows - 4;
    for i in 1..total_rows - 1 {
        for j in 1..total_cols - 1 {
            let mut vertically_invisible1 = false;
            for vert_ind in 0..i {
                if tree_grid[vert_ind][j] >= tree_grid[i][j] {
                    vertically_invisible1 = true;
                }
            }
            let mut vertically_invisible2 = false;
            for vert_ind in i + 1..total_rows {
                if tree_grid[vert_ind][j] >= tree_grid[i][j] {
                    vertically_invisible2 = true;
                }
            }
            let mut horizontally_invisible1 = false;
            for hori_ind in 0..j {
                if tree_grid[i][hori_ind] >= tree_grid[i][j] {
                    horizontally_invisible1 = true;
                }
            }
            let mut horizontally_invisible2 = false;
            for hori_ind in j + 1..total_cols {
                if tree_grid[i][hori_ind] >= tree_grid[i][j] {
                    horizontally_invisible2 = true;
                }
            }
            if !vertically_invisible1
                || !vertically_invisible2
                || !horizontally_invisible1
                || !horizontally_invisible2
            {
                nr_of_visible += 1;
            }
        }
    }
    println!("Number of visible: {}", nr_of_visible);
}

pub fn d8_p2() {
    let in_content = std::fs::read_to_string("./in/d8p1/input").unwrap(); // for testing quickly "./in/d8p1/tmp_in"
    let mut tree_grid: Vec<Vec<char>> = Vec::new();
    for cur_line in in_content.lines() {
        tree_grid.push(Vec::new());
        for cur_char in cur_line.chars() {
            tree_grid.last_mut().unwrap().push(cur_char);
        }
    }
    let total_rows = tree_grid.len();
    let total_cols = tree_grid[0].len();
    let mut max_score = 0;
    for i in 1..total_rows - 1 {
        for j in 1..total_cols - 1 {
            let mut vertical_score1 = 0;
            let mut cur_vs1_ind = i - 1;
            if cur_vs1_ind != 0 {
                while cur_vs1_ind > 0 && tree_grid[cur_vs1_ind][j] < tree_grid[i][j] {
                    vertical_score1 += 1;
                    cur_vs1_ind -= 1;
                }
                vertical_score1 += 1;
            }

            let mut vertical_score2 = 0;
            let mut cur_vs2_ind = i + 1;
            while cur_vs2_ind < total_rows && tree_grid[cur_vs2_ind][j] < tree_grid[i][j] {
                vertical_score2 += 1;
                cur_vs2_ind += 1;
            }
            if cur_vs2_ind < total_rows {
                vertical_score2 += 1;
            }

            let mut horizontal_score1 = 0;
            let mut cur_hs1_ind = j - 1;
            if cur_hs1_ind != 0 {
                while cur_hs1_ind > 0 && tree_grid[i][cur_hs1_ind] < tree_grid[i][j] {
                    horizontal_score1 += 1;
                    cur_hs1_ind -= 1;
                }
                horizontal_score1 += 1;
            }

            let mut horizontal_score2 = 0;
            let mut cur_hs2_ind = j + 1;
            while cur_hs2_ind < total_cols && tree_grid[i][cur_hs2_ind] < tree_grid[i][j] {
                horizontal_score2 += 1;
                cur_hs2_ind += 1;
            }
            if cur_hs2_ind < total_cols {
                horizontal_score2 += 1;
            }

            let cur_score =
                vertical_score1 * vertical_score2 * horizontal_score1 * horizontal_score2;
            if cur_score > max_score {
                max_score = cur_score;
            }
        }
    }
    println!("Maximum scenic score: {}", max_score);
}

fn update_rope_tail(
    head_pos: &(i32, i32),
    tail_pos: &(i32, i32),
    tail_history: &mut BTreeSet<(i32, i32)>,
) -> (i32, i32) {
    let mut new_tail_pos = tail_pos.clone();
    while !(new_tail_pos.0 <= head_pos.0 + 1
        && new_tail_pos.0 >= head_pos.0 - 1
        && new_tail_pos.1 <= head_pos.1 + 1
        && new_tail_pos.1 >= head_pos.1 - 1)
    {
        let vert_direction = if head_pos.1 - new_tail_pos.1 > 0 {
            1
        } else if head_pos.1 == new_tail_pos.1 {
            0
        } else {
            -1
        };
        let hori_direction = if head_pos.0 - new_tail_pos.0 > 0 {
            1
        } else if head_pos.0 == new_tail_pos.0 {
            0
        } else {
            -1
        };
        new_tail_pos.0 += hori_direction;
        new_tail_pos.1 += vert_direction;
        tail_history.insert(new_tail_pos);
    }
    return new_tail_pos;
}

pub fn d9_p1() {
    let in_content = std::fs::read_to_string("./in/d9p1/input").unwrap(); // for testing quickly "./in/d9p1/tmp_in"

    let mut cur_head_pos: (i32, i32) = (0, 0);
    let mut cur_tail_pos: (i32, i32) = (0, 0);
    let mut tail_pos_history: BTreeSet<(i32, i32)> = BTreeSet::new();
    tail_pos_history.insert((0, 0));
    for cur_line in in_content.lines() {
        let parsed_nr_of_steps: i32 = cur_line[2..].parse().unwrap();
        if cur_line.as_bytes()[0] == b'R' {
            cur_head_pos.0 += parsed_nr_of_steps;
        } else if cur_line.as_bytes()[0] == b'L' {
            cur_head_pos.0 -= parsed_nr_of_steps;
        } else if cur_line.as_bytes()[0] == b'U' {
            cur_head_pos.1 += parsed_nr_of_steps;
        } else if cur_line.as_bytes()[0] == b'D' {
            cur_head_pos.1 -= parsed_nr_of_steps;
        }
        cur_tail_pos = update_rope_tail(&cur_head_pos, &cur_tail_pos, &mut tail_pos_history);
    }
    println!("Number of tail movements: {}", tail_pos_history.len());
}

fn check_rope_validity(in_rope: &Vec<(i32, i32)>) -> bool {
    let mut is_rope_valid = true;
    for i in 0..(in_rope.len() - 1) {
        let cur_head_pos = in_rope[i];
        let cur_tail_pos = in_rope[i + 1];
        if !(cur_tail_pos.0 <= cur_head_pos.0 + 1
            && cur_tail_pos.0 >= cur_head_pos.0 - 1
            && cur_tail_pos.1 <= cur_head_pos.1 + 1
            && cur_tail_pos.1 >= cur_head_pos.1 - 1)
        {
            is_rope_valid = false;
        }
    }
    is_rope_valid
}

fn update_the_entire_rope(in_rope: &mut Vec<(i32, i32)>, tail_history: &mut BTreeSet<(i32, i32)>) {
    while !check_rope_validity(in_rope) {
        for i in 0..(in_rope.len() - 1) {
            let mut cur_tail_pos = in_rope[i + 1].clone();
            let cur_head_pos = in_rope[i];
            if !(cur_tail_pos.0 <= cur_head_pos.0 + 1
                && cur_tail_pos.0 >= cur_head_pos.0 - 1
                && cur_tail_pos.1 <= cur_head_pos.1 + 1
                && cur_tail_pos.1 >= cur_head_pos.1 - 1)
            {
                let vert_direction = if cur_head_pos.1 - cur_tail_pos.1 > 0 {
                    1
                } else if cur_head_pos.1 == cur_tail_pos.1 {
                    0
                } else {
                    -1
                };
                let hori_direction = if cur_head_pos.0 - cur_tail_pos.0 > 0 {
                    1
                } else if cur_head_pos.0 == cur_tail_pos.0 {
                    0
                } else {
                    -1
                };
                cur_tail_pos.0 += hori_direction;
                cur_tail_pos.1 += vert_direction;
                if i == in_rope.len() - 2 {
                    tail_history.insert(cur_tail_pos);
                }
            }
            in_rope[i + 1] = cur_tail_pos;
        }
    }
}
pub fn d9_p2() {
    let in_content = std::fs::read_to_string("./in/d9p1/input").unwrap(); // for testing quickly "./in/d9p1/tmp_in1"

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut tail_pos_history: BTreeSet<(i32, i32)> = BTreeSet::new();
    tail_pos_history.insert((0, 0));
    for cur_line in in_content.lines() {
        let parsed_nr_of_steps: i32 = cur_line[2..].parse().unwrap();
        if cur_line.as_bytes()[0] == b'R' {
            rope[0].0 += parsed_nr_of_steps;
        } else if cur_line.as_bytes()[0] == b'L' {
            rope[0].0 -= parsed_nr_of_steps;
        } else if cur_line.as_bytes()[0] == b'U' {
            rope[0].1 += parsed_nr_of_steps;
        } else if cur_line.as_bytes()[0] == b'D' {
            rope[0].1 -= parsed_nr_of_steps;
        }
        update_the_entire_rope(&mut rope, &mut tail_pos_history);
    }
    println!("Number of tail movements: {}", tail_pos_history.len());
}

fn get_regx_history(in_content: &str) -> Vec<i32> {
    let mut add_queue: Vec<Option<i32>> = Vec::new();
    for cur_line in in_content.lines() {
        if cur_line.starts_with("addx ") {
            add_queue.push(Some(cur_line["addx ".len()..].parse().unwrap()));
        } else if cur_line.starts_with("noop") {
            add_queue.push(None);
        }
    }
    add_queue.reverse();

    #[derive(PartialEq)]
    enum CpuState {
        ExecAdd(u32, u32, i32),
        Idle,
    }
    let mut cycle_num = 1;
    let mut current_state = match add_queue.pop().unwrap() {
        Some(val) => CpuState::ExecAdd(0, 1, val),
        None => CpuState::Idle,
    };
    let mut current_x: i32 = 1;
    let mut x_history: Vec<i32> = Vec::new();
    while !(add_queue.len() == 0 && current_state == CpuState::Idle) {
        match current_state {
            CpuState::Idle => {
                x_history.push(current_x);
                let next_instr = add_queue.pop();
                if let Some(Some(val)) = next_instr {
                    current_state = CpuState::ExecAdd(0, 1, val);
                }
                cycle_num += 1;
            }
            CpuState::ExecAdd(cur_step, max_step, value) => {
                x_history.push(current_x);
                if cur_step < max_step {
                    current_state = CpuState::ExecAdd(cur_step + 1, max_step, value);
                } else {
                    current_x += value;
                    current_state = match add_queue.pop() {
                        Some(Some(val)) => CpuState::ExecAdd(0, 1, val),
                        Some(None) => CpuState::Idle,
                        None => CpuState::Idle,
                    }
                }
                cycle_num += 1;
            }
        }
    }
    x_history.push(current_x);
    x_history
}

pub fn d10_p1() {
    let in_content = std::fs::read_to_string("./in/d10p1/input").unwrap(); // for testing quickly "./in/d10p1/tmp_in1"
    let x_history = get_regx_history(&in_content);

    let mut sum_of_sstrengths = 0;
    for query_ind in vec![20, 60, 100, 140, 180, 220] {
        println!("Value of x at {}: {}", query_ind, x_history[query_ind - 1]);
        sum_of_sstrengths += x_history[query_ind - 1] * query_ind as i32;
    }
    println!("Signal strength: {}", sum_of_sstrengths);
}

fn display_crt_buffer(in_crt_buffer: Vec<Vec<char>>) -> String {
    let mut displayed_string = String::new();
    for char_vec in in_crt_buffer {
        for cur_char in char_vec {
            displayed_string.push(cur_char);
        }
        displayed_string.push('\n');
    }

    return displayed_string;
}

pub fn d10_p2() {
    let in_content = std::fs::read_to_string("./in/d10p1/input").unwrap(); // for testing quickly "./in/d10p1/tmp_in1"
    let x_history = get_regx_history(&in_content);
    let mut crt_display_buffer: Vec<Vec<char>> = vec![vec!['?'; 40]; 6];
    for i in 0..crt_display_buffer.len() {
        for j in 0..crt_display_buffer[i].len() {
            let cur_cycle = i * crt_display_buffer[i].len() + j;
            let cycles_x_value = x_history[cur_cycle];
            if cycles_x_value == 0 && (j == 0 || j == 1) {
                crt_display_buffer[i][j] = '#';
            } else if cycles_x_value > 0
                && (j as i32 >= cycles_x_value - 1 && j as i32 <= cycles_x_value + 1)
            {
                crt_display_buffer[i][j] = '#';
            } else {
                crt_display_buffer[i][j] = '.';
            }
        }
    }
    println!("{}", display_crt_buffer(crt_display_buffer));
}
