fn main() {
    let input = include_str!("input.txt");

    // part 1
    let part_1_score = input
        .lines()
        .map(|l| calculate_score(l))
        .sum::<i32>();

    // part 2
    let part_2_score = input
        .lines()
        .map(|l| pick_move(l))
        .map(|l| calculate_score(l))
        .sum::<i32>();

    println!("Part 1: {}", part_1_score);
    println!("Part 2: {}", part_2_score);
}

// part 1
// convert an input line into the score for the
// player
fn calculate_score(line: &str) -> i32
{
    match line.trim() {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0
    }
}

// part 2
// transform an input line into a new line
// compatible with part 1
fn pick_move(line: &str) -> &str
{
    match line.trim() {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",
        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        _ => ""
    }
}