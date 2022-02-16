#![feature(drain_filter)]

mod context;
mod implications;

pub use context::Context;

pub fn next_closure<'a, F>(set: &[String], subset: &[String], closure: F) -> Option<Vec<String>>
where
    F: Fn(&[String]) -> Option<Vec<String>>,
{
    let mut subset = subset.to_vec();

    for (index, m) in set.iter().rev().enumerate() {
        if let Some(i) = subset.iter().position(|s| s == m) {
            subset.remove(i);
            continue;
        }

        subset.push(m.to_string());

        let next = closure(&subset)?;
        let m = lexical_m(set, &subset, &next);

        if m >= (set.len() - index) {
            return Some(next);
        }

        // Pop from subset if not valid since the add was only to test
        subset.pop();
    }

    None
}

fn lexical_m(m: &[String], a: &[String], b: &[String]) -> usize {
    let a = a.to_vec();
    let b = b.to_vec();

    for (i, x) in m.iter().enumerate() {
        if a.contains(x) != b.contains(x) {
            return i;
        }
    }

    m.len()
}

#[cfg(test)]
mod tests {
    use super::{lexical_m, next_closure};

    #[test]
    fn lexical_m_ordered() {
        let m = &[
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let a = &["a".to_string(), "b".to_string(), "d".to_string()];
        let b = &["a".to_string(), "b".to_string(), "c".to_string()];

        assert_eq!(lexical_m(m, a, b), 2);
    }

    #[test]
    fn lexical_m_unordered() {
        let m = &[
            "d".to_string(),
            "b".to_string(),
            "a".to_string(),
            "c".to_string(),
        ];
        let a = &["d".to_string(), "b".to_string(), "a".to_string()];
        let b = &["d".to_string(), "b".to_string(), "c".to_string()];

        assert_eq!(lexical_m(m, a, b), 2);
    }

    #[test]
    fn next_closure_jump() {
        let set = &[
            "running".to_string(),
            "artificial".to_string(),
            "small".to_string(),
        ];

        assert_eq!(
            next_closure(set, &[], |n| match n[0].as_str() {
                "small" => Some(vec!["artificial".to_string(), "small".to_string()]),
                "artificial" => Some(vec!["artificial".to_string()]),
                _ => panic!("unexpected input: {:?}", n),
            }),
            Some(vec!["artificial".to_string()])
        );
    }

    #[test]
    fn next_closure_double() {
        let set = &[
            "running".to_string(),
            "artificial".to_string(),
            "small".to_string(),
        ];

        assert_eq!(
            next_closure(set, &["artificial".to_string()], |n| {
                match (n[0].as_str(), n[1].as_str()) {
                    ("artificial", "small") => {
                        Some(vec!["artificial".to_string(), "small".to_string()])
                    }
                    _ => panic!("unexpected input: {:?}", n),
                }
            }),
            Some(vec!["artificial".to_string(), "small".to_string()])
        );
    }

    #[test]
    fn next_closure_single() {
        let set = &[
            "running".to_string(),
            "artificial".to_string(),
            "small".to_string(),
        ];

        assert_eq!(
            next_closure(set, &["artificial".to_string(), "small".to_string()], |n| {
                match n[0].as_str() {
                    "running" => Some(vec!["running".to_string()]),
                    _ => panic!("unexpected input: {:?}", n),
                }
            }),
            Some(vec!["running".to_string()])
        );
    }

    #[test]
    fn next_closure_end() {
        let set = &[
            "running".to_string(),
            "artificial".to_string(),
            "small".to_string(),
        ];

        assert_eq!(
            next_closure(
                set,
                &[
                    "running".to_string(),
                    "artificial".to_string(),
                    "small".to_string()
                ],
                |n| panic!("unexpected input: {:?}", n)
            ),
            None
        );
    }
}
