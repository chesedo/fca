#![feature(drain_filter)]

mod context;
mod implications;

pub use context::Context;

pub fn next_closure<'a, F>(
    set: &[&'a str],
    subset: Vec<&'a str>,
    closure: F,
) -> Option<Vec<&'a str>>
where
    F: Fn(&[&str]) -> Option<Vec<&'a str>>,
{
    let mut subset = subset;

    for (index, m) in set.iter().enumerate() {
        if subset.contains(m) {
            subset.pop();
            continue;
        }

        subset.push(m);

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

fn lexical_m(m: &[&str], a: &[&str], b: &[&str]) -> Option<usize> {
    let mut new = Vec::from(b);
    new.retain(|n| !a[0..a.len() - 1].contains(n));

    let pos = m.iter().position(|&s| s == new[0])?;

    Some(pos)
}

#[cfg(test)]
mod tests {
    use super::{next_closure, Context};

    #[test]
    fn next_closures() {
        let context = Context::from_csv(
            r#"      ,running,artificial,small
                pond ,       ,  X       , X
                river, x     ,          ,
                canal, X     ,  X       ,"#,
        )
        .unwrap();

        let set: Vec<_> = context.attributes().rev().collect();

        assert_eq!(
            next_closure(&set, vec![], |n| context.closure_extents(n)),
            Some(vec!["artificial"])
        );

        assert_eq!(
            next_closure(&set, vec!["artificial"], |n| context.closure_extents(n)),
            Some(vec!["artificial", "small"])
        );

        assert_eq!(
            next_closure(&set, vec!["artificial", "small"], |n| context
                .closure_extents(n)),
            Some(vec!["running"])
        );

        assert_eq!(
            next_closure(&set, vec!["running", "artificial", "small"], |n| context
                .closure_extents(n)),
            None
        );
    }
}
