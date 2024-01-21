pub fn solve_hanoi(rod_a: &mut Vec<char>, rod_b: &mut Vec<char>, rod_c: &mut Vec<char>) {
    print_state(rod_a, rod_c, rod_b);

    let depth = rod_a.len() - 1;
    let mut iteration = 0;
    hanoi(rod_a, rod_b, rod_c, depth, &mut iteration);
}

fn hanoi(rod_a: &mut Vec<char>, rod_b: &mut Vec<char>, rod_c: &mut Vec<char>, depth: usize, iteration: &mut i32) {
    if rod_a.len() == 1 || depth == 0 {
        return;
    }

    hanoi(rod_a, rod_c, rod_b, depth -1, iteration);

    let disk = rod_a.pop().unwrap();
    rod_b.push(disk);

    print_move(iteration, disk, rod_a.first().unwrap(), rod_b.first().unwrap());
    print_state(rod_a, rod_c, rod_b);

    hanoi(rod_c, rod_b, rod_a, depth -1, iteration);
}

fn print_move (iteration: &mut i32, disk: char, from: &char, to: &char){
    *iteration += 1;
    println!("{} - Moving {} from {} to {}:", iteration, disk, from, to);
}
fn print_state (rod_a: &Vec<char>, rod_b: &Vec<char>, rod_c: &Vec<char>){
    println!("Current state: ");
    for c in vec!['A', 'B', 'C'] {
        for rod in &vec![rod_a, rod_b, rod_c] {
            if *rod.first().unwrap() == c {
                print!("{}: ", c);
                for r in (*rod).iter().skip(1) {
                    print!("{}, ", r);
                }
                println!();
            }
        }
    }
    println!("----------------------------------------");
}

