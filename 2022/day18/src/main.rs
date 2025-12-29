use std::collections::{HashSet, VecDeque};

type Pos = (i32, i32, i32);

fn read_file(path: &str) -> HashSet<Pos> {
    let s = std::fs::read_to_string(path).unwrap();
    s.lines()
        .map(|ln| {
            let cs: Vec<i32> = ln.split(",").map(|s| s.parse().unwrap()).collect();
            (cs[0], cs[1], cs[2])
        })
        .collect()
}

fn get_neighbors(p: &Pos) -> Vec<Pos> {
    let (x, y, z) = *p;
    vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn solve1(pos: &HashSet<Pos>) -> usize {
    pos.iter()
        .flat_map(get_neighbors)
        .filter(|p| !pos.contains(p))
        .count()
}

fn get_outsides(pos: &HashSet<Pos>) -> HashSet<Pos> {
    let min_x = pos.iter().map(|p| p.0).min().unwrap() - 1;
    let max_x = pos.iter().map(|p| p.0).max().unwrap() + 1;
    let min_y = pos.iter().map(|p| p.1).min().unwrap() - 1;
    let max_y = pos.iter().map(|p| p.1).max().unwrap() + 1;
    let min_z = pos.iter().map(|p| p.2).min().unwrap() - 1;
    let max_z = pos.iter().map(|p| p.2).max().unwrap() + 1;

    let mut outside: HashSet<Pos> = HashSet::new();
    let mut candidates: VecDeque<Pos> = VecDeque::new();
    candidates.push_back((min_x, min_y, min_z));

    while !candidates.is_empty() {
        let p = candidates.pop_front().unwrap();
        let ns: HashSet<Pos> = get_neighbors(&p)
            .iter()
            .filter(|&&np| {
                let (x, y, z) = np;
                x >= min_x
                    && x <= max_x
                    && y >= min_y
                    && y <= max_y
                    && z >= min_z
                    && z <= max_z
                    && !pos.contains(&np)
                    && !candidates.contains(&np)
                    && !outside.contains(&np)
            })
            .cloned()
            .collect();
        ns.iter().for_each(|&p| candidates.push_back(p));
        outside.insert(p);
    }
    outside
}

fn solve2(pos: &HashSet<Pos>) -> usize {
    let outside = get_outsides(pos);
    pos.iter()
        .flat_map(get_neighbors)
        .filter(|p| !pos.contains(p) && outside.contains(p))
        .count()
}

fn main() {
    let pos = read_file("input.txt");
    println!("{}", solve1(&pos));
    println!("{}", solve2(&pos));
}

#[test]
fn test() {
    let pos = read_file("test.txt");
    assert_eq!(solve1(&pos), 64);
    assert_eq!(solve2(&pos), 58);
}
