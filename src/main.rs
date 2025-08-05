use std::collections::HashSet;

struct Graph {
    matrix: Vec<Vec<f32>>,
}

impl Graph {
    fn new(matrix: Vec<Vec<f32>>) -> Option<Self> {
        let size = matrix.len();
        for row in &matrix {
            if row.len() != size {
                return None;
            }
        }
        Some(Graph { matrix })
    }

    fn dijkstra(&self, start: usize, end: usize) -> (f32, Vec<usize>) {
        let mut s: HashSet<usize> = vec![start].into_iter().collect();
        let mut l = vec![f32::MAX; self.matrix.len()];
        let mut p = vec![start; self.matrix.len()];

        l[start] = 0.0;
        let mut last_s = start;

        while last_s != end {
            for (i, connection) in
                (&self.matrix[last_s]).into_iter().enumerate()
            {
                if s.contains(&i) {
                    continue;
                }

                let possible_value = l[last_s] + connection;
                if l[i] > possible_value {
                    l[i] = possible_value;
                    p[i] = last_s;
                }
            }

            let mut min_index = usize::MAX;
            for (i, vertex) in (&l).into_iter().enumerate() {
                if s.contains(&i) {
                    continue;
                }
                if min_index == usize::MAX {
                    min_index = i;
                    continue;
                }
                if vertex < &l[min_index] {
                    min_index = i;
                }
            }

            s.insert(min_index);
            last_s = min_index;
        }

        let mut path = Vec::new();
        let mut last_parent = end;
        while last_parent != start {
            path.push(last_parent);
            last_parent = p[last_parent];
        }
        path.push(start);

        (l[end], path.into_iter().rev().collect())
    }
}

fn main() {
    let inf = f32::MAX;
    let matrix: Vec<Vec<f32>> = vec![
        //  v0    v1   v2   v3   v4
        vec![inf, 3.0, 1.0, 1.2, inf],
        vec![3.0, inf, inf, 1.0, 0.5],
        vec![1.0, inf, inf, 0.5, inf],
        vec![1.2, 1.0, 0.5, inf, 1.5],
        vec![inf, 0.5, inf, 1.5, inf],
    ];

    let g = Graph::new(matrix).unwrap();
    let (length, path) = g.dijkstra(0, 1);
    println!("Length: {}", length);
    println!("Path: {:?}", path);
}
