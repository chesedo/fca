use std::collections::BinaryHeap;

use crate::Concept;

#[cfg_attr(test, derive(Debug, PartialEq))]
struct Node {
    concept: Concept,
    lower_neighbour_indices: Vec<usize>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Lattice {
    nodes: Vec<Node>,
}

impl From<Vec<Concept>> for Lattice {
    fn from(concepts: Vec<Concept>) -> Self {
        let mut nodes: Vec<Node> = Vec::with_capacity(concepts.len());
        let mut heap = BinaryHeap::from(concepts);

        while let Some(concept) = heap.pop() {
            let mut subconcept_neighbours = Vec::new();

            let mut subconcept_indices: Vec<usize> = nodes
                .iter()
                .enumerate()
                .filter_map(|(i, node)| {
                    if concept
                        .intents
                        .iter()
                        .all(|intent| node.concept.intents.contains(intent))
                    {
                        subconcept_neighbours.append(&mut node.lower_neighbour_indices.clone());
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();

            subconcept_indices.retain(|s| !subconcept_neighbours.contains(s));

            let node = Node {
                concept,
                lower_neighbour_indices: subconcept_indices,
            };

            nodes.push(node);
        }

        Self { nodes }
    }
}

#[cfg(test)]
mod tests {
    use crate::Concept;

    use super::{Lattice, Node};

    #[test]
    fn from_concepts() {
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

        let actual = Lattice::from(input);
        let nodes = vec![
            Node {
                lower_neighbour_indices: Vec::new(),
                concept: Concept {
                    intents: vec![
                        "a".to_string(),
                        "b".to_string(),
                        "c".to_string(),
                        "d".to_string(),
                        "e".to_string(),
                    ],
                    extents: Vec::new(),
                },
            },
            Node {
                lower_neighbour_indices: vec![0],
                concept: Concept {
                    intents: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    extents: vec!["4".to_string()],
                },
            },
            Node {
                lower_neighbour_indices: vec![0],
                concept: Concept {
                    intents: vec!["b".to_string(), "d".to_string()],
                    extents: vec!["1".to_string()],
                },
            },
            Node {
                lower_neighbour_indices: vec![0],
                concept: Concept {
                    intents: vec!["b".to_string(), "e".to_string()],
                    extents: vec!["2".to_string()],
                },
            },
            Node {
                lower_neighbour_indices: vec![1],
                concept: Concept {
                    intents: vec!["b".to_string(), "c".to_string()],
                    extents: vec!["4".to_string(), "6".to_string()],
                },
            },
            Node {
                lower_neighbour_indices: vec![3],
                concept: Concept {
                    intents: vec!["e".to_string()],
                    extents: vec!["2".to_string(), "7".to_string()],
                },
            },
            Node {
                lower_neighbour_indices: vec![2],
                concept: Concept {
                    intents: vec!["d".to_string()],
                    extents: vec!["1".to_string(), "5".to_string()],
                },
            },
            Node {
                lower_neighbour_indices: vec![4],
                concept: Concept {
                    intents: vec!["c".to_string()],
                    extents: vec!["3".to_string(), "4".to_string(), "6".to_string()],
                },
            },
            Node {
                lower_neighbour_indices: vec![2, 3, 4],
                concept: Concept {
                    intents: vec!["b".to_string()],
                    extents: vec![
                        "1".to_string(),
                        "2".to_string(),
                        "4".to_string(),
                        "6".to_string(),
                    ],
                },
            },
            Node {
                lower_neighbour_indices: vec![5, 6, 7, 8],
                concept: Concept {
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
            },
        ];

        let expected = Lattice { nodes };

        assert_eq!(actual, expected);
    }
}
