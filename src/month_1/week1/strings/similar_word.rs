use std::collections::HashSet;

fn similar_word(
    sentence1: Vec<String>,
    sentence2: Vec<String>,
    similar_pairs: Vec<Vec<String>>,
) -> bool {
    let mut similar_set = HashSet::new();

    for pair in similar_pairs {
        let w1 = &pair[0];
        let w2 = &pair[1];

        similar_set.insert((w1.clone(), w2.clone()));
        similar_set.insert((w2.clone(), w1.clone()));
    }

    for i in 0..sentence1.len() {
        // Words are similar if they are identical OR
        // they have been in the similar set

        let w1 = &sentence1[i];
        let w2 = &sentence2[i];

        if w1 != w2 && !similar_set.contains(&(w1.clone(), w2.clone())) {
            return false;
        }
    }
    true
}
