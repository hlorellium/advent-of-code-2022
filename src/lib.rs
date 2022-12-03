use std::{collections::HashSet, error::Error, fs};

/// First task
pub fn find_most_calories(path: &str) -> Result<u64, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let max = contents
        .split("\n\n")
        .map(|bag| {
            bag.lines()
                .fold(0, |acc, line| acc + line.parse::<u64>().unwrap_or_default())
        })
        .max();

    Ok(max.unwrap())
}

/// Second task
pub fn find_top_n_most_calories(path: &str, count: usize) -> Result<u64, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let mut calories = contents
        .split("\n\n")
        .map(|bag| {
            bag.lines()
                .fold(0, |acc, line| acc + line.parse::<u64>().unwrap_or_default())
        })
        .collect::<Vec<u64>>();

    calories.sort_unstable();

    println!("{:#?}", calories[calories.len() - count..].to_vec());

    let max = calories[calories.len() - count..].iter().sum::<u64>();

    Ok(max)
}

/// Third task
pub fn count_total(path: &str) -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let total = contents.lines().fold(0, |acc, line| {
        let signs = line.split(" ").collect::<Vec<&str>>();

        println!(
            "a: {}, b: {}, total: {}",
            signs[0],
            signs[1],
            count_round(signs[0], signs[1])
        );

        acc + count_round(signs[0], signs[1])
    });

    Ok(total)
}

pub fn count_round(a: &str, b: &str) -> i32 {
    match b {
        "X" => {
            0 + match a {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0,
            }
        }
        "Y" => {
            3 + match a {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0,
            }
        }
        "Z" => {
            6 + match a {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0,
            }
        }
        _ => 0,
    }
}

/// Day 3. part 1
pub fn priorities_sum(path: &str) -> Result<u32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let contents = contents.lines();

    let res = contents
        .into_iter()
        .map(|line| {
            let line = line.chars().collect::<Vec<char>>();

            let first = &line[..line.len() / 2];
            let second = &line[line.len() / 2..];

            let duplicated: String = first.into_iter().filter(|c| second.contains(c)).fold(
                String::new(),
                |acc, char| {
                    if acc.contains(*char) {
                        acc
                    } else {
                        format!("{}{}", acc, char)
                    }
                },
            );

            println!("duplicated: {:#?}", duplicated);

            duplicated
        })
        .collect::<String>();

    let res = res
        .chars()
        .into_iter()
        .map(|char| {
            println!("{char}");
            if char.is_ascii_uppercase() {
                char as u32 - 38
            } else {
                char as u32 - 96
            }
        })
        .collect::<Vec<u32>>();

    println!("bytes: {:#?}", res);

    Ok(res.iter().sum())
}

/// Day 3. part 2
pub fn priorities_sum_3(path: &str) -> Result<u32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let res = contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .into_iter()
        .map(|w| {
            let common_char = w[0].chars().find(|c| {
                let c = c.to_string();
                w[1].contains(&c) && w[2].contains(&c)
            });

            let common = common_char.unwrap();

            if common.is_ascii_uppercase() {
                return common as u32 - 38;
            } else {
                return common as u32 - 96;
            }
        })
        .sum::<u32>();

    return Ok(res);
}
