#![feature(drain_filter)]

mod context;
mod implications;

pub use context::Context;

pub fn next_closure<'a, F>(set: &[String], subset: &[String], closure: F) -> Option<Vec<String>>
where
    F: Fn(&[String]) -> Option<Vec<String>>,
{
    let mut subset = subset.to_vec();

    for (index, m) in set.iter().enumerate() {
        if subset.contains(m) {
            subset.pop();
            continue;
        }

        subset.push(m.to_string());

        let next = closure(&subset)?;
        let m = lexical_m(set, &subset, &next)?;

        // Set is already reversed, therefore this boolean is not < (but its complement)
        if m <= index {
            return Some(next);
        }

        // Pop from subset if not valid since the add was only to test
        subset.pop();
    }

    None
}

fn lexical_m(m: &[String], a: &[String], b: &[String]) -> Option<usize> {
    let mut new = Vec::from(b);
    new.retain(|n| !a[0..a.len() - 1].contains(n));

    let pos = m.iter().position(|s| s == &new[0])?;

    Some(pos)
}

#[cfg(test)]
mod tests {
    use super::next_closure;

    #[test]
    fn next_closure_jump() {
        let set = &[
            "small".to_string(),
            "artificial".to_string(),
            "running".to_string(),
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
            "small".to_string(),
            "artificial".to_string(),
            "running".to_string(),
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
            "small".to_string(),
            "artificial".to_string(),
            "running".to_string(),
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
            "small".to_string(),
            "artificial".to_string(),
            "running".to_string(),
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
