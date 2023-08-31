fn main() {
    let input = include_str!("input.txt");
    let groups = input.split("\n\n");
    let mut totals : Vec<_> = groups.map(|group| get_group_total(group)).collect();

    // sort the totals in descending order
    totals.sort_by(|a, b| b.cmp(a));

    let highest = totals.iter().next().expect("The highest total could not be read");
    println!("{}", highest);

    let top3 : u32 = totals.iter().take(3).sum();
    println!("{}", top3);
}

// calculate the total for each elf
fn get_group_total(group: &str) -> u32 {
    group
        .lines()
        .map(|line| line.trim().parse::<u32>().expect("unable to parse int"))
        .sum()
}