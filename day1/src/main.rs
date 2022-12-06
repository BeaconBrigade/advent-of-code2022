use std::fs;

#[cfg(windows)]
const SEP: &str = "\r\n\r\n";
#[cfg(target_family = "unix")]
const SEP: &str = "\n\n";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = fs::read_to_string("input.txt")?;
    let mut res = input
        .split(SEP)
        .map(|l| l.lines().flat_map(|s| s.parse::<u64>().ok()).sum::<u64>())
        .collect::<Vec<_>>();
    res.sort();
    let res: u64 = res.iter().rev().take(3).sum();
    println!("{res}");

    Ok(())
}
