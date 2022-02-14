use std::collections::BTreeSet;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Implication {
    pub premise: Vec<String>,
    pub conclusion: Vec<String>,
}

pub(crate) fn preclosure_operator(basis: &[Implication], set: &[String]) -> Vec<String> {
    if set.is_empty() {
        return Vec::new();
    }

    let mut x = BTreeSet::from_iter(set.to_vec());
    let mut stable = false;
    let mut basis = basis.to_vec();

    while !stable {
        let new: Vec<_> = basis
            .drain_filter(|i| i.premise.iter().all(|p| x.contains(p)))
            .flat_map(|i| i.conclusion)
            .collect();

        if new.is_empty() {
            stable = true;
        } else {
            x.extend(new);
        }
    }

    x.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{preclosure_operator, Implication};

    #[test]
    fn preclosure_operator_empty() {
        let basis = [Implication {
            premise: vec!["a".to_string()],
            conclusion: vec!["b".to_string()],
        }];
        let set = [];

        let actual = preclosure_operator(&basis, &set);
        let expected: [&str; 0] = [];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_non_matching() {
        let basis = [Implication {
            premise: vec!["a".to_string()],
            conclusion: vec!["b".to_string()],
        }];
        let set = ["c".to_string()];

        let actual = preclosure_operator(&basis, &set);
        let expected = ["c"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_matching() {
        let basis = [Implication {
            premise: vec!["a".to_string()],
            conclusion: vec!["b".to_string()],
        }];
        let set = ["a".to_string()];

        let actual = preclosure_operator(&basis, &set);
        let expected = ["a", "b"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_recursive() {
        let basis = [
            Implication {
                premise: vec!["a".to_string()],
                conclusion: vec!["b".to_string()],
            },
            Implication {
                premise: vec!["a".to_string(), "b".to_string()],
                conclusion: vec!["c".to_string(), "d".to_string()],
            },
        ];
        let set = ["a".to_string()];

        let actual = preclosure_operator(&basis, &set);
        let expected = ["a", "b", "c", "d"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_recursive_multiple() {
        let basis = [
            Implication {
                premise: vec!["a".to_string()],
                conclusion: vec!["b".to_string()],
            },
            Implication {
                premise: vec!["a".to_string(), "b".to_string()],
                conclusion: vec!["c".to_string(), "d".to_string()],
            },
            Implication {
                premise: vec!["c".to_string()],
                conclusion: vec!["d".to_string()],
            },
        ];
        let set = ["a".to_string()];

        let actual = preclosure_operator(&basis, &set);
        // "d" should not appear twice
        let expected = ["a", "b", "c", "d"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_recursive_multiple_unordered() {
        let basis = [
            Implication {
                premise: vec!["a".to_string()],
                conclusion: vec!["b".to_string()],
            },
            Implication {
                premise: vec!["b".to_string(), "a".to_string()],
                conclusion: vec!["d".to_string(), "c".to_string()],
            },
            Implication {
                premise: vec!["c".to_string()],
                conclusion: vec!["d".to_string()],
            },
        ];
        let set = ["a".to_string()];

        let actual = preclosure_operator(&basis, &set);
        // "d" should not appear twice
        let expected = ["a", "b", "c", "d"];

        assert_eq!(actual, expected);
    }
}
