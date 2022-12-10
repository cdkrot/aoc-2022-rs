use crate::utils;

pub(crate) fn main() {
    let lines = utils::read_all_lines();

    let mut register = vec!(1);

    for line in lines {
        let line = line.trim();
        let cur_value = *register.last().unwrap();

        match line {
            "noop" => {
                register.push(cur_value);
            }
            _ => {
                let tokens = line.split_once(' ').unwrap();
                let delta = tokens.1.parse::<i32>().unwrap();

                register.push(cur_value);
                register.push(cur_value + delta);
            }
        }
    }

    let mut sum = 0;
    let mut i = 20;
    while i <= 220 {
        sum += (i as i64) * (register[i - 1] as i64);
        i += 40;
    }

    println!("Sum is {}", sum);

    let mut rows = vec![vec!['.'; 40]; 6];

    for cycle in 1..241 {
        let char_pos = (cycle - 1) % 40;

        if (register[cycle - 1] - (char_pos as i32)).abs() <= 1 {
            rows[(cycle - 1) / 40][char_pos] = '#';
        }
    }

    for row in rows {
        println!("{}", row.iter().collect::<String>());
    }
}