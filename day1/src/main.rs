use std::fs;

use color_eyre::{eyre::eyre, eyre::Context};
use itertools::Itertools;

struct ElfsCalories {
    top: i64,
    top_3_sum: i64,
}

fn compute_calories(filepath: &str) -> color_eyre::Result<ElfsCalories> {
    let mut top_3_calories = fs::read_to_string(filepath)
        .wrap_err(format!("reading {}", filepath))?
        .split("\n\n")
        .map(|by_elf| {
            by_elf
                .lines()
                .flat_map(|str| str.parse::<i64>())
                .sum::<i64>()
        })
        .sorted_by(|x, y| y.cmp(x))
        .take(3);

    let most_calorific = top_3_calories
        .next()
        .ok_or(eyre!("No elves calories found"))?;
    let top_3_calories_sum = most_calorific + top_3_calories.sum::<i64>();

    Ok(ElfsCalories {
        top: most_calorific,
        top_3_sum: top_3_calories_sum,
    })
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let calories = compute_calories("src/input.txt")?;

    println!(
        "most calorific: {}, sum of top 3 calories: {}",
        calories.top, calories.top_3_sum
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::compute_calories;

    #[test]
    fn it_works() {
        let calories = compute_calories("src/input.txt").unwrap();
        assert_eq!(calories.top, 74711);
        assert_eq!(calories.top_3_sum, 209481);
    }

    #[test]
    fn it_does_not_works() {
        assert!(compute_calories("src/fake_input.txt").is_err());
    }
}
