use std::collections::BinaryHeap;

pub fn lost_stone_weight(stones : Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones) ; 

    while heap.len() > 1 {
        let heaviest  = heap.pop().unwrap() ;
        let second_heaviest = heap.pop().unwrap() ; 

        if heaviest !=second_heaviest {
            let new_height  = heaviest - second_heaviest ; 
            heap.push(new_height);
        }
    }
    heap.pop().unwrap()
}
