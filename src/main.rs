mod hanoi;

use crate::hanoi::solve_hanoi;

fn main() {

    let mut start = vec!['A', '7', '6', '5', '4', '3', '2', '1'];
    let mut finish = vec!['B'];
    let mut helper = vec!['C'];

    solve_hanoi(&mut start, &mut finish, &mut helper);
}
