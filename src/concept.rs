use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Concept {
    pub extents: Vec<String>,
    pub intents: Vec<String>,
}

impl Ord for Concept {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.intents.iter().all(|i| other.intents.contains(i)) {
            Ordering::Less
        } else if other.intents.iter().all(|i| self.intents.contains(i)) {
            Ordering::Greater
        } else {
            if self.extents.iter().all(|e| other.extents.contains(e)) {
                Ordering::Greater
            } else if other.extents.iter().all(|e| self.extents.contains(e)) {
                Ordering::Less
            } else {
                self.intents
                    .len()
                    .cmp(&other.intents.len())
                    .then(other.extents.len().cmp(&self.extents.len()))
            }
        }
    }
}

impl PartialOrd for Concept {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::Concept;

    #[test]
    fn ordered() {
        let input = vec![
            Concept {
                intents: vec!["d".to_string()],
                extents: vec!["1".to_string(), "5".to_string()],
            },
            Concept {
                intents: Vec::new(),
                extents: vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                ],
            },
            Concept {
                intents: vec!["b".to_string()],
                extents: vec![
                    "1".to_string(),
                    "2".to_string(),
                    "4".to_string(),
                    "6".to_string(),
                ],
            },
            Concept {
                intents: vec!["e".to_string()],
                extents: vec!["2".to_string(), "7".to_string()],
            },
            Concept {
                intents: vec!["b".to_string(), "e".to_string()],
                extents: vec!["2".to_string()],
            },
            Concept {
                intents: vec!["c".to_string()],
                extents: vec!["3".to_string(), "4".to_string(), "6".to_string()],
            },
            Concept {
                intents: vec!["b".to_string(), "d".to_string()],
                extents: vec!["1".to_string()],
            },
            Concept {
                intents: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                ],
                extents: Vec::new(),
            },
            Concept {
                intents: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                extents: vec!["4".to_string()],
            },
            Concept {
                intents: vec!["b".to_string(), "c".to_string()],
                extents: vec!["4".to_string(), "6".to_string()],
            },
        ];

        let mut heap = BinaryHeap::from(input);

        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                ],
                extents: Vec::new(),
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                extents: vec!["4".to_string()],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["b".to_string(), "d".to_string()],
                extents: vec!["1".to_string()],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["b".to_string(), "e".to_string()],
                extents: vec!["2".to_string()],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["b".to_string(), "c".to_string()],
                extents: vec!["4".to_string(), "6".to_string()],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["e".to_string()],
                extents: vec!["2".to_string(), "7".to_string()],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["d".to_string()],
                extents: vec!["1".to_string(), "5".to_string()],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["c".to_string()],
                extents: vec!["3".to_string(), "4".to_string(), "6".to_string()],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: vec!["b".to_string()],
                extents: vec![
                    "1".to_string(),
                    "2".to_string(),
                    "4".to_string(),
                    "6".to_string(),
                ],
            })
        );
        assert_eq!(
            heap.pop(),
            Some(Concept {
                intents: Vec::new(),
                extents: vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                ],
            })
        );
        assert_eq!(heap.pop(), None);
    }
}
