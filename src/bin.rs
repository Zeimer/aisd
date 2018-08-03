extern crate aisd;

//use std::collections::HashSet;

use aisd::pq::Heap;

use aisd::depq::DEPQ;
use aisd::depq::DoubleHeap;

//use aisd::coin_change::*;


fn main() {

    /*let mut s = HashSet::new();
    s.insert(4);
    s.insert(2);
    make_change_count(&s, 10);*/

    let mut h = DoubleHeap::new();
    h.ins_all(vec![99, 97, 95, 94, 96]);
    
    loop {
        println!("{:?}", h);
        println!("is heap l: {}",
            Heap::is_heap(&h.min_array.clone().into_iter().map(|x| x.0).collect()));
        match h.del_max() {
            Some(_) => {},
            None => break
        }
    }

}