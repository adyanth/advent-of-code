use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub struct Edge {
    node: usize,
    cost: usize,
}

#[aoc_generator(day9)]
pub fn generate(input: &str) -> Vec<Vec<Edge>> {
    let mut mapping = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split(" ");
        let from = tokens.next().unwrap();
        if !mapping.contains_key(from) {
            mapping.insert(from.to_string(), mapping.len());
        }
        tokens.next();
        let to = tokens.next().unwrap();
        if !mapping.contains_key(to) {
            mapping.insert(to.to_string(), mapping.len());
        }
    }

    let mut graph: Vec<Vec<Edge>> = (0..mapping.len())
        .enumerate()
        .map(|(i, _)| {
            (0..mapping.len())
                .map(|_| Edge { node: i, cost: 0 })
                .collect()
        })
        .collect();

    for line in input.lines() {
        let mut tokens = line.split(" ");
        let from = *mapping.get(tokens.next().unwrap()).unwrap();
        tokens.next().unwrap();
        let to = *mapping.get(tokens.next().unwrap()).unwrap();
        let dist = tokens.last().unwrap().parse::<usize>().unwrap();

        graph[from][to] = Edge {
            node: to,
            cost: dist,
        };
        graph[to][from] = Edge {
            node: from,
            cost: dist,
        };
    }
    graph
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    mask: usize,
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.node
            .cmp(&other.node)
            .then_with(|| self.mask.cmp(&other.mask))
    }
}

fn shortest_path(graph: &Vec<Vec<Edge>>) -> usize {
    let n = graph.len();
    let mut dists: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..(1 << n)).map(|_| usize::MAX).collect())
        .collect();

    let mut heap = BinaryHeap::new();

    for node in 0..n {
        heap.push(State {
            node: node,
            mask: 1 << node,
        });
        dists[node][1 << node] = 0;
    }

    while let Some(State { node, mask }) = heap.pop() {
        for neighbor in &graph[node] {
            let add = neighbor.cost;
            if mask & 1 << neighbor.node != 0 {
                // Visited
                continue;
            }
            let neighbor_mask = mask | (1 << neighbor.node);
            if dists[neighbor.node][neighbor_mask]
                > dists[node][mask].checked_add(add).unwrap_or(usize::MAX)
            {
                heap.push(State {
                    node: neighbor.node,
                    mask: neighbor_mask,
                });
                dists[neighbor.node][neighbor_mask] = dists[node][mask] + add;
            }
        }
    }

    dists.iter().map(|v| v[(1 << n) - 1]).min().unwrap()
}

#[aoc(day9, part1)]
pub fn part1(graph: &Vec<Vec<Edge>>) -> usize {
    shortest_path(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            part1(&vec![
                vec![Edge { node: 1, cost: 100 }, Edge { node: 2, cost: 10 }],
                vec![Edge { node: 0, cost: 100 }, Edge { node: 2, cost: 10 }],
                vec![Edge { node: 0, cost: 10 }, Edge { node: 1, cost: 10 }]
            ]),
            20
        );
        assert_eq!(
            part1(&vec![
                vec![Edge { node: 1, cost: 5 }, Edge { node: 2, cost: 10 }],
                vec![Edge { node: 0, cost: 5 }, Edge { node: 2, cost: 10 }],
                vec![Edge { node: 0, cost: 10 }, Edge { node: 1, cost: 10 }]
            ]),
            15
        );
    }

    #[test]
    fn test_sq() {
        assert_eq!(
            part1(&vec![
                vec![
                    Edge { node: 1, cost: 10 },
                    Edge { node: 2, cost: 10 },
                    Edge { node: 3, cost: 1 }
                ],
                vec![
                    Edge { node: 0, cost: 10 },
                    Edge { node: 2, cost: 10 },
                    Edge { node: 3, cost: 10 }
                ],
                vec![
                    Edge { node: 0, cost: 10 },
                    Edge { node: 1, cost: 10 },
                    Edge { node: 3, cost: 100 }
                ],
                vec![
                    Edge { node: 0, cost: 1 },
                    Edge { node: 1, cost: 10 },
                    Edge { node: 2, cost: 100 }
                ],
            ]),
            21
        );
        assert_eq!(
            part1(&vec![
                vec![
                    Edge { node: 1, cost: 10 },
                    Edge { node: 2, cost: 100 },
                    Edge { node: 3, cost: 100 }
                ],
                vec![
                    Edge { node: 0, cost: 10 },
                    Edge { node: 2, cost: 1 },
                    Edge { node: 3, cost: 10 }
                ],
                vec![
                    Edge { node: 0, cost: 100 },
                    Edge { node: 1, cost: 1 },
                    Edge { node: 3, cost: 100 }
                ],
                vec![
                    Edge { node: 0, cost: 100 },
                    Edge { node: 1, cost: 10 },
                    Edge { node: 2, cost: 100 }
                ],
            ]),
            111
        );
    }
}
