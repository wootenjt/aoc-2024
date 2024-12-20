use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Rule {
    before: u16,
    after: u16,
}

impl TryFrom<String> for Rule {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut nums = value.split('|');
        let before = nums
            .next()
            .ok_or(String::from("no first num"))?
            .parse::<u16>()
            .map_err(|_| String::from("couldnt parse first"))?;

        let after = nums
            .next()
            .ok_or(String::from("no second num"))?
            .parse::<u16>()
            .map_err(|_| String::from("couldnt parse second"))?;

        Ok(Rule { before, after })
    }
}

pub fn read_input<P>(path: P) -> (HashSet<Rule>, Vec<Vec<u16>>)
where
    P: AsRef<std::path::Path>,
{
    let mut lines = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());

    let mut rules: HashSet<Rule> = HashSet::new();

    while let Some(line) = lines.next().filter(|line| !line.is_empty()) {
        rules.insert(line.try_into().unwrap());
    }
    let revisions = lines
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules, revisions)
}

pub fn relevant_pages(page: &u16, rules: &HashSet<Rule>) -> (Vec<u16>, Vec<u16>) {
    let (mut befores, mut afters) = (vec![], vec![]);

    for rule in rules {
        if rule.before == *page {
            afters.push(rule.after);
        }
        if rule.after == *page {
            befores.push(rule.before);
        }
    }

    (befores, afters)
}

pub fn valid_revision(revision: &[u16], rules: &HashSet<Rule>) -> bool {
    for (i, page) in revision.iter().enumerate() {
        let (befores, afters) = relevant_pages(page, rules);
        if revision.iter().take(i).any(|edit| afters.contains(edit)) {
            return false;
        }
        if revision
            .iter()
            .skip(i + 1)
            .any(|edit| befores.contains(edit))
        {
            return false;
        }
    }
    true
}

fn compare_edits(edit_a: u16, edit_b: u16, rules: &HashSet<Rule>) -> Ordering {
    if rules.iter().any(|rule| {
        *rule
            == Rule {
                before: edit_a,
                after: edit_b,
            }
    }) {
        Ordering::Less
    } else if rules.iter().any(|rule| {
        *rule
            == Rule {
                before: edit_b,
                after: edit_a,
            }
    }) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn correct_revision(revision: &[u16], rules: &HashSet<Rule>) -> Vec<u16> {
    let mut out = revision.to_owned();
    out.sort_by(|a, b| compare_edits(*a, *b, rules));
    out
}
