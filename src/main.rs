pub mod heap;

use heap::Heap;

fn main() {
    let v = vec![6, 4, 1, 7, 9, 3, 1];
    let h1 = Heap::make_heap_bottom_up(v.clone());
    let h2 = Heap::make_heap_top_down(v.clone());
    println!("h1 = {:?}", h1);
    println!("h2 = {:?}", h2);
    assert!(Heap::is_heap(h1.arr()));
    assert!(Heap::is_heap(h2.arr()));
}
