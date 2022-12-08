use color_eyre::{eyre::eyre, eyre::Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::BTreeMap, fs};

fn get_starting_stacks(str: String) -> color_eyre::Result<BTreeMap<usize, Vec<String>>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("[A-Z]").unwrap();
    }

    let stacks = str.lines().fold(BTreeMap::new(), |mut acc, line| {
        for (i, part) in line
            .as_bytes()
            .chunks(4)
            .flat_map(std::str::from_utf8)
            .enumerate()
        {
            let stack = acc.entry(i + 1).or_insert_with(Vec::new);

            if let Some(cap) = RE.captures(part) {
                stack.push(cap[0].to_string());
            }
        }
        acc
    });

    Ok(stacks)
}

fn get_movements(str: String) -> color_eyre::Result<Vec<(usize, usize, usize)>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }

    let movements = str
        .lines()
        .flat_map(|line| {
            match RE
                .captures_iter(line)
                .flat_map(|cap| cap[0].parse::<usize>())
                .collect::<Vec<_>>()[..]
            {
                [qty, from, to] => Some((qty, from, to)),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    Ok(movements)
}

fn get_parts(filepath: &str) -> color_eyre::Result<(String, String)> {
    match fs::read_to_string(filepath)
        .wrap_err(format!("reading {}", filepath))?
        .split("\n\n")
        .collect::<Vec<&str>>()[..]
    {
        [stacks, movements] => Ok((String::from(stacks), String::from(movements))),
        _ => Err(eyre!("Invalid parts input")),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CrateMoverModel {
    CrateMover9000,
    CrateMover9001,
}

fn get_ending_top_crates(
    filepath: &str,
    crate_mover: CrateMoverModel,
) -> color_eyre::Result<String> {
    let (stacks, movements) = get_parts(filepath)?;

    let crates_on_top = get_movements(movements)?
        .into_iter()
        .fold(get_starting_stacks(stacks)?, |mut acc, (qty, from, to)| {
            if let Some(from_stack) = acc.get_mut(&from) {
                // let mut tmp = vec![];
                let mut collected: Vec<_> = from_stack.drain(0..qty).collect();
                match crate_mover {
                    CrateMoverModel::CrateMover9000 => (),
                    CrateMoverModel::CrateMover9001 => collected.reverse(),
                }

                if let Some(to_stack) = acc.get_mut(&to) {
                    for item in collected {
                        to_stack.insert(0, item.to_string());
                    }
                }
            }
            acc
        })
        .values()
        .into_iter()
        .fold(String::new(), |mut acc, stack| {
            acc.push_str(&stack[0]);
            acc
        });

    Ok(crates_on_top)
}

fn main() -> color_eyre::Result<()> {
    let ending_stack_1 = get_ending_top_crates("src/input.txt", CrateMoverModel::CrateMover9000)?;
    println!("ending 1: {:?}", ending_stack_1);

    let ending_stack_2 = get_ending_top_crates("src/input.txt", CrateMoverModel::CrateMover9001)?;
    println!("ending 2: {:?}", ending_stack_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_ending_top_crates, CrateMoverModel};

    #[test]
    fn it_get_ending_top_crates() {
        assert_eq!(
            get_ending_top_crates("src/input-test.txt", CrateMoverModel::CrateMover9000).unwrap(),
            "CMZ"
        );
    }

    #[test]
    fn it_get_reversed_ending_top_crates() {
        assert_eq!(
            get_ending_top_crates("src/input-test.txt", CrateMoverModel::CrateMover9001).unwrap(),
            "MCD"
        );
    }
}
