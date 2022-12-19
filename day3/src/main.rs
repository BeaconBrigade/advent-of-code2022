fn main() {
    let input = include_str!("input.txt");
    let sacks: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(Group::parse)
        .map(|g| g.priority())
        .sum();
    println!("sum of badge priorities is {}", sacks);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group<'a> {
    sacks: [Rucksack<'a>; 3],
}

impl<'a> Group<'a> {
    fn parse(input: &[&'a str]) -> Self {
        let sacks: Vec<_> = input.iter().copied().take(3).map(Rucksack::parse).collect();
        Self {
            sacks: sacks.try_into().unwrap(),
        }
    }

    fn duplicate(&self) -> char {
        let [one, two, three] = self.sacks.clone().map(|r| r.to_string());
        one.chars()
            .find(|&c| two.contains(c) && three.contains(c))
            .unwrap()
    }

    fn priority(self) -> usize {
        priority(self.duplicate())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack<'a> {
    compartments: (&'a str, &'a str),
}

impl<'a> Rucksack<'a> {
    fn parse(input: &'a str) -> Self {
        let (l, r) = input.split_at(input.len() / 2);

        Self {
            compartments: (l, r),
        }
    }

    #[allow(unused)]
    fn duplicate(&self) -> char {
        self.compartments
            .0
            .chars()
            .find(|&c| self.compartments.1.contains(c))
            .unwrap()
    }

    #[allow(unused)]
    fn priority(&self) -> usize {
        priority(self.duplicate())
    }
}

impl<'a> ToString for Rucksack<'a> {
    fn to_string(&self) -> String {
        self.compartments.0.to_owned() + self.compartments.1
    }
}

fn priority(c: char) -> usize {
    let offset = if c.is_uppercase() { 27 } else { 1 };
    c.to_lowercase().next().unwrap() as usize - 'a' as usize + offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_duplicate() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let sack = Rucksack::parse(input);
        assert_eq!(sack.duplicate(), 'p');
    }

    #[test]
    fn find_priority() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let sack = Rucksack::parse(input);
        assert_eq!(sack.priority(), 16);
    }
}
