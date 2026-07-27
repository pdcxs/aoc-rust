use std::collections::HashMap;

type Bin = usize; // one or zero
type State = Vec<Bin>; // State of lights
type Button = Vec<Bin>; // connect or not to each light
type Value = usize; // Light Value
type LightValues = Vec<Value>;
type Problem = (State, Vec<Button>, LightValues);
type PushTimes = usize;
type Push = Vec<PushTimes>;
type Record = HashMap<(usize, LightValues), PushTimes>;

fn parse(input: &str) -> Vec<Problem> {
    input.lines().map(parse_line).collect()
}

fn parse_line(ln: &str) -> Problem {
    let mut it = ln.split_whitespace();
    let first = it.next().unwrap();
    let others = it.collect::<Vec<_>>();
    let state = get_state(first);
    let light_num = state.len();
    let nums: Vec<_> = others.into_iter().map(get_num).collect();
    let (target, buttons) = nums.split_last().unwrap();
    (
        state,
        buttons.iter().map(|b| decode(b, light_num)).collect(),
        target.clone(),
    )
}

fn decode(input: &Vec<usize>, light_num: usize) -> Button {
    let mut btn = vec![0; light_num];
    for &light_id in input {
        btn[light_id] = 1;
    }
    btn
}

fn rm_ends(input: &str) -> &str {
    let n = input.len();
    &input[1..n - 1]
}

fn get_state(input: &str) -> State {
    rm_ends(input)
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect()
}

fn get_num(input: &str) -> Vec<usize> {
    rm_ends(input)
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_push_combs(problem: &Problem) -> Vec<Push> {
    let button_num = problem.1.len();
    (0..2_usize.pow(button_num as u32))
        .map(|n| (0..button_num).map(|b| (n >> b) & 1).collect())
        .collect()
}

fn get_push_result(problem: &Problem, push: &Push) -> LightValues {
    let buttons = &problem.1;
    buttons
        .iter()
        .zip(push)
        .map(|(btn, push_times)| btn.iter().map(|b| b * push_times).collect::<Vec<_>>())
        .fold(vec![0; problem.0.len()], |acc, elem| {
            acc.iter().zip(elem).map(|(n, m)| n + m).collect()
        })
}

fn solution1(problem: &Problem) -> PushTimes {
    let combs = get_push_combs(problem);
    let target_state = &problem.0;
    let push_times: Vec<usize> = combs.iter().map(|c| c.iter().sum()).collect();
    let push_results: Vec<LightValues> = combs
        .iter()
        .map(|push| get_push_result(problem, push))
        .collect();
    *push_times
        .iter()
        .zip(push_results)
        .filter(|(_, rs)| rs.iter().map(|v| v % 2).collect::<Vec<usize>>() == *target_state)
        .min()
        .unwrap()
        .0
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let problems = parse(&input);
    println!("{:?}", problems.iter().map(solution1).sum::<usize>());
}
