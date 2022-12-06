use color_eyre::{eyre::eyre, eyre::Context};
use std::fs;

fn parse_as_ints(str: &str) -> Vec<u32> {
    str.split('-')
        .flat_map(|x| x.parse::<u32>())
        .collect::<Vec<u32>>()
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum ResultType {
    FullyContained,
    Overlapping,
}

fn find_matching(a: &str, b: &str, result_type: ResultType) -> bool {
    let x = parse_as_ints(a);
    let y = parse_as_ints(b);

    match result_type {
        ResultType::FullyContained => x[0] >= y[0] && x[1] <= y[1] || y[0] >= x[0] && y[1] <= x[1],
        ResultType::Overlapping => x[0] <= y[0] && x[1] >= y[0] || y[0] <= x[0] && y[1] >= x[0],
    }
}

fn find_matching_assignments(filepath: &str, result_type: ResultType) -> color_eyre::Result<i32> {
    let result: i32 = fs::read_to_string(filepath)
        .wrap_err(format!("reading {}", filepath))?
        .lines()
        .flat_map(|line| match line.split(',').collect::<Vec<&str>>()[..] {
            [a, b] => Ok(i32::from(find_matching(a, b, result_type))),
            _ => Err(eyre!("Invalid input")),
        })
        .sum();

    Ok(result)
}

fn main() -> color_eyre::Result<()> {
    let result_1 = find_matching_assignments("src/input.txt", ResultType::FullyContained)?;
    println!("result 1: {}", result_1);

    let result_2 = find_matching_assignments("src/input.txt", ResultType::Overlapping)?;
    println!("result 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_matching, find_matching_assignments, ResultType};

    #[test]
    fn it_is_not_fully_contained() {
        assert!(!find_matching("2-4", "6-8", ResultType::FullyContained));
    }

    #[test]
    fn it_is_fully_contained() {
        assert!(find_matching(
            "2-8",
            "3-7",
            crate::ResultType::FullyContained
        ));
    }

    #[test]
    fn it_is_not_overlapping() {
        assert!(!find_matching("2-4", "6-8", ResultType::Overlapping));
    }

    #[test]
    fn it_is_overlapping() {
        assert!(find_matching("5-7", "7-9", ResultType::Overlapping));
    }

    #[test]
    fn it_find_fully_contained_assignments() {
        assert_eq!(
            find_matching_assignments("src/input-test.txt", ResultType::FullyContained).unwrap(),
            2
        );
    }

    #[test]
    fn it_find_overlapping_assignments() {
        assert_eq!(
            find_matching_assignments("src/input-test.txt", ResultType::Overlapping).unwrap(),
            4
        );
    }
}
