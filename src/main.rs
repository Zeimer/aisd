pub mod pq;
pub mod depq;
pub mod coin_change;

/*use pq::Heap;
use depq::DEPQ;
use depq::DoubleHeap;*/

use coin_change::make_change;

fn main() {
    /*let v = vec![6, 4, 1, 7, 9, 3, 1];
    let h1 = Heap::make_heap_bottom_up(v.clone());
    let h2 = Heap::make_heap_top_down(v.clone());
    println!("h1 = {:?}", h1);
    println!("h2 = {:?}", h2);
    assert!(Heap::is_heap(h1.arr()));
    assert!(Heap::is_heap(h2.arr()));

    // depq
    let mut d: DoubleHeap<u32> = DoubleHeap::new();
    d.insert(1);
    d.insert(2); 
    d.insert(3);
    d.insert(4);
    println!("d = {:?}", d);

    loop {
        match d.del_max() {
            Some(_) => println!("d = {:?}", d),
            None => break
        }
    }*/

    let mut fv = vec![1, 1, 2, 2, 2, 5, 5, 10, 10, 10]; //, 5, 5, 2, 2, 2, 1, 1];
    let k = 27;
    println!("{:?}", make_change(&mut fv, k));
}