#[derive(Debug)]
enum Turn {
    Left(i32),
    Right(i32),
}

fn parse_turn(line: &str) -> Turn {
    let mut chars = line.chars();
    let dir = chars.next().unwrap();
    let num = chars.as_str().parse::<i32>().unwrap();
    match dir {
        'R' => Turn::Right(num),
        'L' => Turn::Left(num),
        _ => panic!("Unknown {}", dir),
    }
}

fn read_file(file_name: &str) -> Vec<Turn> {
    let contents = std::fs::read_to_string(file_name).unwrap();
    contents.lines().map(parse_turn).collect()
}

fn step((count, pos): (i32, i32), turn: &Turn) -> (i32, i32) {
    let pos = match turn {
        Turn::Left(x) => (pos - *x).rem_euclid(100),
        Turn::Right(x) => (pos + *x).rem_euclid(100),
    };
    let count = if pos == 0 { count + 1 } else { count };
    (count, pos)
}

fn solve<F>(step: F, turns: &[Turn]) -> i32
where
    F: Fn((i32, i32), &Turn) -> (i32, i32),
{
    turns.iter().fold((0, 50), step).0
}

fn step2((mut count, mut pos): (i32, i32), turn: &Turn) -> (i32, i32) {
    let (dir, mut num) = match turn {
        Turn::Left(n) => (-1, *n),
        Turn::Right(n) => (1, *n),
    };
    while num > 0 {
        pos = (pos + dir).rem_euclid(100);
        if pos == 0 {
            count += 1;
        }
        num -= 1;
    }
    (count, pos)
}

fn main() {
    let turns = read_file("input.txt");
    println!("{}", solve(step, &turns));
    println!("{}", solve(step2, &turns));
}

#[test]
fn test() {
    let turns = read_file("test.txt");
    assert_eq!(solve(step, &turns), 3);
    assert_eq!(solve(step2, &turns), 6);
}
