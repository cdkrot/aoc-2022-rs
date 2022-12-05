pub(crate) fn main() {
    let top: Vec<String> = std::io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .take_while(|s| !s.is_empty())
        .collect();

    let mut stacks = vec!();

    for (i, ch) in top[top.len() - 1].chars().enumerate() {
        if ch != ' ' {
            let stack_id = stacks.len();
            stacks.push(vec!() as Vec<char>);

            for j in (0..top.len()-1).rev() {
                let char_i = top[j].as_bytes()[i] as char;
                if char_i != ' ' {
                    stacks[stack_id].push(char_i);
                }
            }
        }
    }

    let mut stacks_partb = stacks.clone();

    for instruction in std::io::stdin().lines().map(|x| x.unwrap()) {
        if instruction.trim() == "" {
            continue;
        }

        let pieces: Vec<&str> = instruction.split(' ').collect();

        let num: usize = pieces[1].parse().unwrap();
        let from = pieces[3].parse::<usize>().unwrap() - 1;
        let to = pieces[5].parse::<usize>().unwrap() - 1;

        for i in 0..num {
            let new_element = stacks[from].pop().unwrap();
            stacks[to].push(new_element);

            let new_element_b = stacks_partb[from][stacks_partb[from].len()  - num + i];
            stacks_partb[to].push(new_element_b);
        }

        let new_size_of_from_stack = stacks_partb[from].len() - num;
        stacks_partb[from].resize(new_size_of_from_stack, ' ');
    }

    let mut result = String::new();
    for ch in stacks.iter().map(|x| x.last().unwrap()) {
        result.push(*ch);
    }

    let mut result_partb = String::new();
    for ch in stacks_partb.iter().map(|x| x.last().unwrap()) {
        result_partb.push(*ch);
    }

    println!("{}", result);
    println!("{}", result_partb);
}