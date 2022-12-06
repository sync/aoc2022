use color_eyre::{eyre::eyre, eyre::Context};
use std::fs;

fn priority_for_letter(letter: char) -> color_eyre::Result<u64> {
    match letter {
        'a'..='z' => Ok(letter as u64 - 96),
        'A'..='Z' => Ok(letter as u64 - 38),
        _ => Err(eyre!("Invalid letter: {}", letter)),
    }
}

fn find_duplicate_letter_in_group(group: Vec<&str>) -> Option<char> {
    match group[..] {
        [a, b] => a.chars().find(|&letter| b.chars().any(|x| x == letter)),
        [a, b, c] => a
            .chars()
            .find(|&letter| b.chars().any(|x| x == letter) && c.chars().any(|x| x == letter)),
        _ => None,
    }
}

fn get_sum_of_priorities(filepath: &str, chunks: Option<usize>) -> color_eyre::Result<u64> {
    let input = fs::read_to_string(filepath).wrap_err(format!("reading {}", filepath))?;

    let by_group = if let Some(chunks) = chunks {
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(chunks)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<&str>>>()
    } else {
        input
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(left, right)| vec![left, right])
            .collect::<Vec<Vec<&str>>>()
    };

    let sum = by_group
        .into_iter()
        .filter_map(find_duplicate_letter_in_group)
        .flat_map(priority_for_letter)
        .sum();

    Ok(sum)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let result_1 = get_sum_of_priorities("src/input.txt", None)?;
    println!("result 1: {}", result_1);

    let result_2 = get_sum_of_priorities("src/input.txt", Some(3))?;
    println!("result 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_duplicate_letter_in_group, get_sum_of_priorities, priority_for_letter};

    #[test]
    fn it_get_a_priority_for_lower_a() {
        assert_eq!(priority_for_letter('a').unwrap(), 1);
    }

    #[test]
    fn it_get_a_priority_for_upper_z() {
        assert_eq!(priority_for_letter('Z').unwrap(), 52);
    }

    #[test]
    fn it_find_the_duplicate_letter_lower_p() {
        assert_eq!(
            find_duplicate_letter_in_group(vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"]).unwrap(),
            'p'
        );
    }

    #[test]
    fn it_find_the_duplicate_letter_upper_l() {
        assert_eq!(
            find_duplicate_letter_in_group(vec!["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"]).unwrap(),
            'L'
        );
    }

    #[test]
    fn it_find_the_duplicate_letter_in_group() {
        assert_eq!(
            find_duplicate_letter_in_group(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ])
            .unwrap(),
            'r'
        );
    }

    #[test]
    fn it_get_sum_of_priorities() {
        assert_eq!(
            get_sum_of_priorities("src/input-test.txt", None).unwrap(),
            157
        );
    }

    #[test]
    fn it_get_sum_of_priorities_with_chunks() {
        assert_eq!(
            get_sum_of_priorities("src/input-test.txt", Some(3)).unwrap(),
            70
        );
    }
}
