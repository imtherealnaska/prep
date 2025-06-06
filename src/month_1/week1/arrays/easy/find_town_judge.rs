pub fn find_town_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;

    let mut in_degree = vec![0; n + 1];
    let mut out_degree = vec![0; n + 1];

    for relation in &trust {
        let a = relation[0] as usize;
        let b = relation[1] as usize;

        out_degree[a] += 1;
        in_degree[b] += 1;
    }

    for person in 1..=n {
        if in_degree[person] == n - 1 && out_degree[person] == 0 {
            return person as i32;
        }
    }
    -1
}

// Single pass
pub fn single_find_town_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut net_trust = vec![0; n + 1];

    for relation in &trust {
        let a = relation[0] as usize;
        let b = relation[1] as usize;

        net_trust[a] -= 1;
        net_trust[b] += 1;
    }

    for person in 1..=n {
        if net_trust[person] == (n - 1) as i32 {
            return person as i32;
        }
    }
    -1
}
