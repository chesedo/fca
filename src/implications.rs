struct Implication<'a> {
    premise: &'a [&'a str],
    conclusion: &'a [&'a str],
}

fn preclosure_operator<'a>(basis: &'a [&Implication], set: &'a [&str]) -> Vec<&'a str> {
    if set.is_empty() {
        return Vec::new();
    }

    let mut x = set.to_vec();
    let mut stable = false;
    let mut basis = basis.to_vec();

    while !stable {
        let mut new: Vec<&str> = basis
            .drain_filter(|i| i.premise.iter().all(|p| x.contains(p)))
            .flat_map(|i| i.conclusion)
            .map(|s| *s)
            .collect();

        if new.is_empty() {
            stable = true;
        } else {
            x.append(&mut new);
        }
    }

    x
}

#[cfg(test)]
mod tests {
    use super::{preclosure_operator, Implication};

    #[test]
    fn preclosure_operator_empty() {
        let basis = [&Implication {
            premise: &["a"],
            conclusion: &["b"],
        }];
        let set = [];

        let actual = preclosure_operator(&basis, &set);
        let expected: [&str; 0] = [];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_non_matching() {
        let basis = [&Implication {
            premise: &["a"],
            conclusion: &["b"],
        }];
        let set = ["c"];

        let actual = preclosure_operator(&basis, &set);
        let expected = ["c"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_matching() {
        let basis = [&Implication {
            premise: &["a"],
            conclusion: &["b"],
        }];
        let set = ["a"];

        let actual = preclosure_operator(&basis, &set);
        let expected = ["a", "b"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn preclosure_operator_recursive() {
        let basis = [
            &Implication {
                premise: &["a"],
                conclusion: &["b"],
            },
            &Implication {
                premise: &["a", "b"],
                conclusion: &["c", "d"],
            },
        ];
        let set = ["a"];

        let actual = preclosure_operator(&basis, &set);
        let expected = ["a", "b", "c", "d"];

        assert_eq!(actual, expected);
    }
}
