use anyhow::Result;
use csv::Reader;
use std::{borrow::Cow, fmt};

use ndarray::{iter::Lanes, Array, Array2, ArrayView, Axis, Dim};

use crate::{
    implications::{preclosure_operator, Implication},
    next_closure,
};

pub struct Context {
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

    pub fn objects(&self) -> Cow<[String]> {
        Cow::Borrowed(&self.objects)
    }

    pub fn attributes(&self) -> Cow<[String]> {
        Cow::Borrowed(&self.attributes)
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

    pub fn intents(&self, objects: &[String]) -> Option<Vec<String>> {
        Self::der(&self.objects, objects, self.array.rows(), &self.attributes)
    }

    pub fn extents(&self, attributes: &[String]) -> Option<Vec<String>> {
        Self::der(
            &self.attributes,
            attributes,
            self.array.columns(),
            &self.objects,
        )
    }

    pub fn closure_intents(&self, objects: &[String]) -> Option<Vec<String>> {
        let attributes = self.intents(objects)?;
        self.extents(&attributes)
    }

    pub fn closure_extents(&self, attributes: &[String]) -> Option<Vec<String>> {
        let objects = self.extents(attributes)?;
        self.intents(&objects)
    }

    fn der(
        inputs_named: &[String],
        inputs: &[String],
        set: Lanes<bool, Dim<[usize; 1]>>,
        outputs_named: &[String],
    ) -> Option<Vec<String>> {
        // Empty set is always all the attributes / objects
        if inputs.is_empty() {
            return Some(outputs_named.to_vec());
        }

        let indices: Vec<_> = inputs_named
            .iter()
            .enumerate()
            .filter_map(|(i, o)| if inputs.contains(o) { Some(i) } else { None })
            .collect();

        let der = set
            .into_iter()
            .enumerate()
            .filter_map(|(i, r)| {
                if indices.contains(&i) {
                    Some(r.into_iter().map(ToOwned::to_owned).collect::<Vec<bool>>())
                } else {
                    None
                }
            })
            .reduce(bitand)?;

        let output = outputs_named
            .iter()
            .zip(der)
            .filter_map(|(attribute, has)| {
                if has {
                    Some(attribute.to_string())
                } else {
                    None
                }
            })
            .collect();

        Some(output)
    }

    pub fn object_has_attribute(&self, object: &str, attribute: &str) -> Option<bool> {
        let object_index = self.objects.iter().position(|o| o == object)?;
        let attribute_index = self.attributes.iter().position(|o| o == attribute)?;

        self.array.get((object_index, attribute_index)).copied()
    }

    pub fn concepts(&self) -> Vec<Vec<String>> {
        let a: Vec<String> = self
            .attributes()
            .into_iter()
            .rev()
            .map(|a| a.to_string())
            .collect();

        let mut concepts = Vec::new();
        let mut current = self.closure_extents(&[]);

        while current != None {
            let c = current.unwrap();
            concepts.push(c.clone());
            current = next_closure(&a[..], &c[..], |n| self.closure_extents(n));
        }

        concepts
    }

    pub fn canonical_basis(&self) -> Vec<Implication> {
        let mut l = Vec::new();
        let mut a = Vec::new();
        let m: Vec<_> = self
            .attributes()
            .into_iter()
            .rev()
            .map(|a| a.to_string())
            .collect();

        while a != m {
            let closure = self.closure_extents(&a).unwrap();
            if a != closure {
                l.push(Implication {
                    premise: a.clone(),
                    conclusion: closure,
                });
            }

            let next = next_closure(&m[..], &a, |n| Some(preclosure_operator(&l, n)));

            if next.is_none() {
                break;
            }

            a = next.unwrap();
        }

        l
    }
}

fn bitand(accum: Vec<bool>, new: Vec<bool>) -> Vec<bool> {
    accum.iter().zip(new).map(|(a, n)| a & n).collect()
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
    use crate::implications::Implication;

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
            context.intents(&["pond".to_string()]),
            Some(vec!["artificial".to_string()]),
            "ponds should be artificial {}",
            context
        );
        assert_eq!(context.object_has_attribute("river", "running"), Some(true));
    }

    #[test]
    #[should_panic(expected = "found record with 3 fields")]
    fn from_csv_more_attributes() {
        Context::from_csv(
            r#",running,   artificial,extra
                pond,,X
                river, x ,"#,
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "found record with 3 fields")]
    fn from_csv_less_attributes() {
        Context::from_csv(
            r#",running
                pond,,X
                river, x ,"#,
        )
        .unwrap();
    }

    #[test]
    fn intents() {
        let context = Context::from_csv(
            r#",running,   artificial,small
                pond,,X,X
                river, x ,,
                canal,X,X,"#,
        )
        .unwrap();

        let actual = context.intents(&["pond".to_string()]);
        let expected = Some(vec!["artificial".to_string(), "small".to_string()]);

        assert_eq!(actual, expected);

        let actual = context.intents(&["canal".to_string(), "river".to_string()]);
        let expected = Some(vec!["running".to_string()]);

        assert_eq!(actual, expected);

        assert_eq!(context.intents(&["missing".to_string()]), None);
        assert_eq!(
            context.intents(&[]),
            Some(vec![
                "running".to_string(),
                "artificial".to_string(),
                "small".to_string()
            ])
        );
    }

    #[test]
    fn extents() {
        let context = Context::from_csv(
            r#",running,   artificial,inland
                pond,,X,X
                river, x ,,X"#,
        )
        .unwrap();

        let actual = context.extents(&["inland".to_string()]);
        let expected = Some(vec!["pond".to_string(), "river".to_string()]);

        assert_eq!(actual, expected);

        let actual = context.extents(&["inland".to_string(), "running".to_string()]);
        let expected = Some(vec!["river".to_string()]);

        assert_eq!(actual, expected);

        assert_eq!(context.extents(&["missing".to_string()]), None);
        assert_eq!(
            context.extents(&[]),
            Some(vec!["pond".to_string(), "river".to_string()])
        );
    }

    #[test]
    fn closure_intents() {
        let context = Context::from_csv(
            r#"      ,running,artificial,small
                pond ,       ,  X       , X
                river, x     ,          ,
                canal, X     ,  X       ,"#,
        )
        .unwrap();

        let actual = context.closure_intents(&["pond".to_string()]);
        let expected = Some(vec!["pond".to_string()]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn closure_extents() {
        let context = Context::from_csv(
            r#",running,   artificial,inland
                pond,,X,X
                river, x ,,X"#,
        )
        .unwrap();

        let actual = context.closure_extents(&["inland".to_string()]);
        let expected = Some(vec!["inland".to_string()]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn concepts() {
        let context = Context::from_csv(
            r#",a,b,c,d,e,f
              1, ,x,x, ,x,x
              2, ,x,x, , , 
              3, , , , , , 
              4, , , , ,x,x
              5, , ,x, ,x,x
              6, ,x, , , , 
              7, , ,x, ,x,x
              8, , ,x,x, ,x
              9, , ,x, ,x,x
             10, ,x,x,x,x,x"#,
        )
        .unwrap();

        let expected = vec![
            vec![],
            vec!["f"],
            vec!["e", "f"],
            vec!["c"],
            vec!["c", "f"],
            vec!["c", "e", "f"],
            vec!["c", "d", "f"],
            vec!["b"],
            vec!["b", "c"],
            vec!["b", "c", "e", "f"],
            vec!["b", "c", "d", "e", "f"],
            vec!["a", "b", "c", "d", "e", "f"],
        ];

        assert_eq!(context.concepts(), expected);
    }

    #[test]
    fn canonical_basis() {
        let context = Context::from_csv(
            r#",a,b,c,d,e
              1, ,x, ,x, 
              2, ,x, , ,x
              3, , ,x, , 
              4,x,x,x, , 
              5, , , ,x, 
              6, ,x,x, , 
              7, , , , ,x"#,
        )
        .unwrap();

        let expected = vec![
            Implication {
                premise: vec!["d".to_string(), "e".to_string()],
                conclusion: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                ],
            },
            Implication {
                premise: vec!["c".to_string(), "e".to_string()],
                conclusion: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                ],
            },
            Implication {
                premise: vec!["c".to_string(), "d".to_string()],
                conclusion: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                ],
            },
            Implication {
                premise: vec!["a".to_string()],
                conclusion: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            },
        ];

        assert_eq!(context.canonical_basis(), expected);
    }
}
