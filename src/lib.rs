use std::fmt;

struct Context {
    num_objects: usize,
    num_attributes: usize,
}

impl Context {
    pub fn new(objects: usize, attributes: usize) -> Self {
        Self {
            num_objects: objects,
            num_attributes: attributes,
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print headers
        write!(f, "  ")?;
        for a in 1..=self.num_attributes {
            write!(f, "| {} ", a)?;
        }
        // Print each row
        for r in 1..=self.num_objects {
            writeln!(f)?;

            write!(f, "{} ", r)?;
            for _ in 1..=self.num_attributes {
                write!(f, "|   ")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn display() {
        let context = Context::new(4, 3);

        let actual = format!("{}", context);
        let expected = r#"  | 1 | 2 | 3 
1 |   |   |   
2 |   |   |   
3 |   |   |   
4 |   |   |   "#;

        assert_eq!(actual, expected);
    }
}
