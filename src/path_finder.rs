use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn is_in_range(size: (usize, usize), node: &(i32, i32)) -> bool {
    (node.0 >= 0 && node.0 < size.0 as i32) && (node.1 >= 0 && node.1 < size.1 as i32)
}
fn calc_cost(start: &(i32, i32), end: &(i32, i32)) -> i32 {
    let displacement = ((end.0 - start.0).abs(), (end.1 - start.1).abs());

    if displacement.0 > displacement.1 {
        (14 * displacement.1) + (10 * (displacement.0 - displacement.1))
    } else {
        (14 * displacement.0) + (10 * (displacement.1 - displacement.0))
    }
}

fn neighbor(node: &(i32, i32), size: (usize, usize)) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let new_node = (node.0 + i, node.1 + j);
            if is_in_range(size, &new_node) {
                res.push((node.0 + i, node.1 + j))
            }
        }
    }
    res
}

fn reconstruct_path(
    path_map: HashMap<(i32, i32), (i32, i32)>,
    end: (i32, i32),
    start: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut path = Vec::from([end]);
    let mut current = end;
    while let Some(&next) = path_map.get(&current) {
        path.push(next);
        current = next;
        if current == start {
            break;
        }
    }
    path.reverse();
    path
}

#[derive(Eq, Debug)]
struct Node(i32, (i32, i32));

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.0.cmp(&other.0) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        })
    }
}

pub fn a_star(
    size: (usize, usize),
    start: (i32, i32),
    end: (i32, i32),
    block: HashSet<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut open_list = BinaryHeap::from([Node(0i32, start)]);

    let mut closed_list = HashSet::<(i32, i32)>::new();
    let mut path_map = HashMap::<(i32, i32), (i32, i32)>::new();

    let mut g_cost: HashMap<(i32, i32), i32> = HashMap::new();

    while let Some(current_node) = open_list.pop() {
        if current_node.1 == end {
            return reconstruct_path(path_map, current_node.1, start);
        }

        let current_node = current_node.1;
        closed_list.insert(current_node);

        for next in neighbor(&current_node, size) {
            if closed_list.contains(&next) || block.contains(&next) {
                continue;
            }

            let current_node_g_cost = g_cost.get(&current_node).cloned().unwrap_or_default();
            let next_g_cost = current_node_g_cost + calc_cost(&current_node, &next);

            let in_open = open_list.iter().find(|&n| n.1 == next);
            let is_in_open = in_open.is_some();
            let found_cheaper = if in_open.is_some()
                && next_g_cost < g_cost.get(&next).cloned().unwrap_or_default()
            {
                open_list = open_list
                    .into_vec()
                    .into_iter()
                    .filter_map(|n| if n.1 == next { None } else { Some(n) })
                    .collect();
                true
            } else {
                false
            };

            if !is_in_open || found_cheaper {
                g_cost.insert(next, next_g_cost);
                let next_h_cost = calc_cost(&next, &end);
                open_list.push(Node(next_g_cost + next_h_cost, next));
                path_map.insert(next, current_node);
            }
        }
    }

    reconstruct_path(path_map, start, start)
}

#[cfg(test)]
mod tests {
    use crate::path_finder::calc_cost;

    #[test]
    fn test_cost_calc() {
        assert_eq!(calc_cost(&(10, 10), &(10, 15)), 50);
        assert_eq!(calc_cost(&(10, 10), &(12, 15)), 58);
    }
}
