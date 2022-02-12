use anyhow::Result;
use csv::Reader;
use std::fmt;

use ndarray::{Array, Array2, ArrayView, Axis};

struct Context {
    array: Array2<bool>,
    objects: Vec<String>,
    attributes: Vec<String>,
}

impl Context {
    pub fn new(objects: usize, attributes: usize) -> Self {
        Self {
            objects: (1..=objects).map(|i| i.to_string()).collect(),
            attributes: (1..=attributes).map(|i| i.to_string()).collect(),
            array: Array::default((objects, attributes)),
        }
    }

    pub fn from_csv(data: &str) -> Result<Self> {
        let mut reader = Reader::from_reader(data.as_bytes());

        let mut headers = reader.headers()?.iter();
        // First column is for objects
        headers.next();
        let headers: Vec<String> = headers.map(|h| h.trim().to_string()).collect();
        let num_attributes = headers.len();

        let mut objects: Vec<String> = Vec::new();
        let mut array = Array::default((0, num_attributes));

        for record in reader.into_records() {
            let record = record?;

            let mut iter = record.iter();
            objects.push(
                iter.next()
                    .map_or_else(|| (objects.len() + 1).to_string(), |v| v.trim().to_string()),
            );
            let result = iter
                .map(|v| v.trim().to_uppercase() == "X")
                .collect::<Vec<_>>();

            array.append(
                Axis(0),
                ArrayView::from_shape((1, num_attributes), &result)?,
            )?;
        }

        Ok(Self {
            objects,
            attributes: headers,
            array,
        })
    }

    pub fn get_intents(&self, object: &str) -> Option<Vec<&str>> {
        let index = self.objects.iter().position(|o| o == object)?;
        let row = self.array.row(index);

        let intents = self
            .attributes
            .iter()
            .zip(row)
            .filter_map(
                |(attribute, has)| {
                    if *has {
                        Some(attribute.as_str())
                    } else {
                        None
                    }
                },
            )
            .collect();

        Some(intents)
    }

    pub fn get_extents(&self, attribute: &str) -> Option<Vec<&str>> {
        let index = self.attributes.iter().position(|a| a == attribute)?;
        let col = self.array.column(index);

        let extents = self
            .objects
            .iter()
            .zip(col)
            .filter_map(|(object, belongs)| {
                if *belongs {
                    Some(object.as_str())
                } else {
                    None
                }
            })
            .collect();

        Some(extents)
    }

    pub fn object_has_attribute(&self, object: &str, attribute: &str) -> Option<bool> {
        let object_index = self.objects.iter().position(|o| o == object)?;
        let attribute_index = self.attributes.iter().position(|o| o == attribute)?;

        self.array.get((object_index, attribute_index)).map(|b| *b)
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print headers
        write!(f, "  ")?;
        let shape = self.array.shape();
        let objects = shape[0];
        let attributes = shape[1];

        for a in 0..attributes {
            write!(f, "| {} ", self.attributes[a])?;
        }
        // Print each row
        for r in 0..objects {
            writeln!(f)?;

            write!(f, "{} ", self.objects[r])?;
            for a in 0..attributes {
                let symbol = if self.array[[r, a]] { "X" } else { " " };
                write!(f, "| {} ", symbol)?;
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
        let mut context = Context::new(4, 3);

        context.array[[2, 2]] = true;

        let actual = format!("{}", context);
        let expected = r#"  | 1 | 2 | 3 
1 |   |   |   
2 |   |   |   
3 |   |   | X 
4 |   |   |   "#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_csv() {
        let context = Context::from_csv(
            r#",running,   artificial
                pond,,X
                river, x ,"#,
        )
        .unwrap();

        assert!(
            context.array[[0, 1]],
            "ponds should be artificial {}",
            context
        );
        assert_eq!(
            context.get_intents("pond"),
            Some(vec!["artificial"]),
            "ponds should be artificial {}",
            context
        );
        assert_eq!(context.object_has_attribute("river", "running"), Some(true));
    }

    #[test]
    fn intents() {
        let context = Context::from_csv(
            r#",running,   artificial,small
                pond,,X,X
                river, x ,,"#,
        )
        .unwrap();

        let actual = context.get_intents("pond");
        let expected = Some(vec!["artificial", "small"]);

        assert_eq!(actual, expected);

        assert_eq!(context.get_intents("missing"), None);
    }

    #[test]
    fn extents() {
        let context = Context::from_csv(
            r#",running,   artificial,inland
                pond,,X,X
                river, x ,,X"#,
        )
        .unwrap();

        let actual = context.get_extents("inland");
        let expected = Some(vec!["pond", "river"]);

        assert_eq!(actual, expected);

        assert_eq!(context.get_extents("missing"), None);
    }
}
