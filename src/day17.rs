use crate::utils;

struct Rock<'a> {
    // (row, column).
    coords: &'a [(i32, i32)],
    #[allow(dead_code)]
    width: i32,
    height: i32,
}

fn make_rock(coords: &[(i32, i32)]) -> Rock {
    let width = coords.iter().map(|p| p.1).max().unwrap() + 1;
    let height = coords.iter().map(|p| p.0).max().unwrap() + 1;

    return Rock {
        coords,
        width,
        height,
    }
}

const VEC_COORDS: [&[(i32, i32)]; 5] = [
    &[(0,0), (0, 1), (0, 2), (0, 3)],
    &[(1,0), (1, 1), (1, 2), (0, 1), (2, 1)],
    &[(0,0), (0, 1), (0, 2), (1, 2), (2, 2)],
    &[(0,0), (1, 0), (2, 0), (3, 0)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

fn get_rocks() -> Vec<Rock<'static>> {
    VEC_COORDS.iter().map(|coords| make_rock(coords)).collect()
}

pub(crate) fn main() {
    let rocks = get_rocks();
    let pattern: Vec<char> = utils::read_line().trim().chars().collect();
    let mut p_patern = 0;

    let mut get_movement = || {
        if p_patern == pattern.len() {
            p_patern = 0;
        }
        let ch = pattern[p_patern];
        p_patern += 1;

        match ch {
            '<' => -1,
            '>' => 1,
            _ => panic!("Unknown char {}", ch)
        }
    };

    let mut state = vec![];
    fn remove_empty_rows(state: &mut Vec<Vec<char>>) {
        while !state.is_empty() &&
            state.last().unwrap().iter().all(|ch| *ch == ' ') {
            state.pop();
        }
    }

    for i in 0..2022 {
        remove_empty_rows(&mut state);
        let rock = &rocks[i % rocks.len()];

        let mut rock_row_offset = (state.len() + 3) as i32;
        let mut rock_col_offset = 2;

        state.resize_with(state.len() + 3 + (rock.height as usize), || vec!(' '; 7));

        assert!(fits(rock, rock_row_offset, rock_col_offset, &state));

        /* // debugging
        place(rock, rock_row_offset, rock_col_offset, &mut state, '@', true);

        if i <= 15 {
            for row in (0..state.len()).rev() {
                println!("|{}|", state[row].iter().collect::<String>());
            }
            println!("====");
        }
        place(rock, rock_row_offset, rock_col_offset, &mut state, ' ', false);
        */

        loop {
            let delta_col = get_movement();
            if fits(rock, rock_row_offset, rock_col_offset + delta_col, &state) {
                rock_col_offset += delta_col;
            }

            if fits(rock, rock_row_offset - 1, rock_col_offset, &state) {
                rock_row_offset -= 1;
            } else {
                break;
            }
        }

        place(rock, rock_row_offset, rock_col_offset, &mut state, '#', true);
    }

    remove_empty_rows(&mut state);
    println!("Filled rows: {}", state.len());
}

fn fits(rock: &Rock, row_offset: i32, col_offset: i32, state: &Vec<Vec<char>>) -> bool {
    for (r, c) in rock.coords {
        let r = *r + row_offset;
        let c = *c + col_offset;

        if r < 0 || r >= (state.len() as i32) {
            return false;
        }
        if c < 0 || c >= (state[r as usize].len() as i32) {
            return false;
        }

        if state[r as usize][c as usize] != ' ' {
            return false;
        }
    }

    return true;
}

fn place(rock: &Rock, rock_row_offset: i32, rock_col_offset: i32, state: &mut Vec<Vec<char>>, with: char,
         verify: bool) {
    for (r, c) in rock.coords {
        let r = *r + rock_row_offset;
        let c = *c + rock_col_offset;
        if verify {
            assert_eq!(state[r as usize][c as usize], ' ');
        }
        state[r as usize][c as usize] = with;
    }
}