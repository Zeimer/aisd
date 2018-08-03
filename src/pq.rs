//! Priority queues.

use std::ops::Index;
use std::f64;

use quickcheck::Arbitrary;
use quickcheck::Gen;

pub trait PriorityQueue {
    type Item;

    /// Check whether the queue is empty.
    fn is_empty(&self) -> bool;

    /// Compute the number of elements in the queue.
    fn size(&self) -> usize;

    /// Insert a new element into the queue.
    fn insert(&mut self, item: Self::Item);

    /// Like `insert`, but allows chaining.
    /// Time: like `insert` (unless reimplemented).
    fn ins(&mut self, item: Self::Item) -> &mut Self {
        self.insert(item);
        self
    }

    /// Return a reference to the least element in the queue.
    fn min(&self) -> Option<&Self::Item>;

    /// Remove the minimal element from the queue and return it.
    fn del_min(&mut self) -> Option<Self::Item>;
}

// A binary heap implemented implicitly using a Vec.
#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    array: Vec<T>
}

impl<T: PartialOrd> Heap<T> {
    /// Create an empty priority queue.
    /// Time: O(1)
    pub fn new() -> Heap<T> {
        Heap {array: vec![]}
    }

    /// Get a reference to the heap's inner array.
    /// Time: O(1)
    pub fn arr(&self) -> &Vec<T> {
        &self.array
    }

    /// Compute the height of the heap. The empty heap has height 0
    /// and a nonempty heap of size n has height ceiling(log_2(n)) + 1.
    /// Time: O(1)
    pub fn height(&self) -> u32 {
        let s = self.size();
        if s == 0 {
            0
        } else {
            (self.size() as f64).log(2.0).ceil() as u32 + 1
        }
    }

    /// Check if a vector is a valid heap.
    /// Time:O(size of the heap)
    pub fn is_heap_aux(v: &Vec<T>, cmp: fn(&T, &T) -> bool) -> bool {
        if v.len() > 0 {
            let last = v.len() - 1;

            // Iterate over all nodes that have children and check if their
            // values are less than these of their children.
            for i in 0 .. last {
                let l = 2 * i + 1;
                let r = 2 * i + 2;

                if r <= last {
                    if cmp(&v[i], &v[l]) || cmp(&v[i], &v[r]) {
                        return false;
                    }
                } else if l <= last {
                    if cmp(&v[i], &v[l]) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn is_heap(v: &Vec<T>) -> bool {
        Heap::is_heap_aux(v, PartialOrd::gt)
    }

    // Make sure that all nodes on the path from i to
    // root satisfy the heap property. Time: O(height of the heap).
    fn fix_heap_property_bottom_up(&mut self, i: usize) {
        let mut current = i;
        while current != 0 {
            let parent = (current - 1)/2;

            if self.array[current] < self.array[parent] {
                self.array.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }

    // Make sure that the smallest element is at the root by repeatedly swapping
    // the root with the smaller of its children if they're bigger than the root.
    // Time: O(height of the heap)
    fn fix_heap_property_top_down(&mut self, i: usize) {
        let mut current = i;

        loop {
            let left = 2 * current + 1;
            let right = 2 * current + 2;

            if self.size() > right {
                let son = if self.array[left] < self.array[right] {left} else {right};

                if self.array[current] > self.array[son] {
                    self.array.swap(current, son);
                    current = son;
                } else {
                    break;
                }
            } else if self.size() > left {
                if self.array[current] > self.array[left] {
                    self.array.swap(current, left);
                    current = left;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    /// Create a heap from a vector.
    /// Time: O(size of the heap * height of the heap)
    pub fn make_heap_bottom_up(v: Vec<T>) -> Heap<T> {
        let mut h = Heap {array: v};

        for i in 0 .. h.size() {
            h.fix_heap_property_bottom_up(i);
        }

        h
    }

    /// Create a heap from a vector.
    /// Time: O(size of the heap)
    pub fn make_heap_top_down(v: Vec<T>) -> Heap<T> {
        if v.len() == 0 {
            Heap::new()
        } else {
            let mut h = Heap {array: v};

            for i in (0 .. (h.size() - 1)).rev() {
                h.fix_heap_property_top_down(i);
            }

            h
        }
    }
    
    /// Destructive heapsort.
    /// Time: O(nlgn)
    pub fn sort(v: &mut Vec<T>) {
        let mut h = Heap::new();
        
        loop {
            match v.pop() {
                Some(x) => h.insert(x),
                None => break
            }
        }

        loop {
            match h.del_min() {
                Some(x) => v.push(x),
                None => break
            }
        }
    }

    /// Less destructive heapsort.
    /// Time: O(nlgn)
    pub fn sort2(v: Vec<T>) -> Vec<T> {
        let mut h = Heap::new();
        for x in v {
            h.insert(x);
        }
        h.collect()
    }
}

impl<T: PartialOrd> PriorityQueue for Heap<T> {
    type Item = T;

    /// Time: O(1)
    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    /// Time: O(1)
    fn size(&self) -> usize {
        self.array.len()
    }

    /// Time: O(height of the heap)
    fn insert(&mut self, elem: T) {
        self.array.push(elem);
        let l = self.size() - 1; // Doesn't overflow because of previous line.
        self.fix_heap_property_bottom_up(l);
    }
    
    /// Time: O(1)
    fn min(&self) -> Option<&T> {
        if self.size() == 0 {
            None
        } else {
            Some(self.array.index(0))
        }
    }

    /// Time: O(height of the heap)
    fn del_min(&mut self) -> Option<T> {
        if self.size() == 0 {
            None
        } else {
            let last_index = self.size() - 1;
            self.array.swap(0, last_index);

            let result = self.array.pop();

            self.fix_heap_property_top_down(0);

            result
        }
    }
}

/// A heap is also an iterator (`next` is `del_min`).
impl<T: PartialOrd> Iterator for Heap<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.del_min()
    }
}

impl<T: PartialOrd + Clone> PartialEq for Heap<T> {
    fn eq(&self, rhs: &Heap<T>) -> bool {
        Heap::sort2(self.array.clone()) == Heap::sort2(rhs.array.clone())
    }
}

impl<T: PartialOrd + Clone> Clone for Heap<T> {
    fn clone(&self) -> Self {
        let v = self.array.clone();

        Heap {array: v}
    }
}

impl<T: PartialOrd + Arbitrary + Clone> Arbitrary for Heap<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let v: Vec<T> = Arbitrary::arbitrary(g);

        let mut h = Heap::new();
        for x in v {
            h.insert(x);
        }

        h
    }
}

#[cfg(test)]
mod tests {
    use pq::Heap;
    use pq::PriorityQueue;

    fn is_sorted<T: PartialOrd>(v: &Vec<T>) -> bool {
        if v.len() >= 2 {
            for i in 0 .. v.len() - 2 {
                if v[i] > v[i + 1] {return false;}
            }
        }

        true
    }

    // Interface tests.
    quickcheck! {
        fn is_empty_size(h: Heap<u32>) -> bool {
            h.is_empty() == (h.size() == 0)
        }

        fn is_empty_insert(h: Heap<u32>, item: u32) -> bool {
            let mut h2 = h.clone();
            h2.insert(item);
            h2.is_empty() == false
        }

        fn min_is_empty(h: Heap<u32>) -> bool {
            (h.clone().min() != Option::None) == (h.is_empty() == false)
        }

        fn del_min_is_empty(h: Heap<u32>) -> bool {
            (h.clone().del_min() != Option::None) == (h.is_empty() == false)
        }

        fn size_ins(h: Heap<u32>, i: u32) -> bool {
            h.size() + 1 == h.clone().ins(i).size()
        }
        
        fn size_min(h: Heap<u32>) -> bool {
            (h.clone().min() != Option::None) == (h.size() != 0)
        }

        fn size_del_min(h: Heap<u32>) -> bool {
            let mut h2 = h.clone();
            match h2.del_min() {
                Some(_) => h.size() == h2.size() + 1,
                None => h.size() == h2.size()
            }
        }

        fn insert_del_min(h: Heap<u32>) -> bool {
            let mut h2 = h.clone();
            let h3 = h.clone();

            let item = h2.del_min();

            match item {
                Some(i) => *h2.ins(i) == h3,
                None => h2 == h3
            }
        }

        fn insert_min(h: Heap<u32>) -> bool {
            let mut h2 = h.clone();

            h2.ins(0).min() == Some(0)
        }

        fn min_del_min(h: Heap<u32>) -> bool {
            let mut h2 = h.clone();

            h.min() == h2.del_min()
        }

        fn del_min_least(h: Heap<u32>) -> bool {
            let mut h2 = h.clone();
            let m = match h2.del_min() {
                Some(m) => m,
                None => {return true;}
            };

            for &x in h2.arr() {
                if x < m {return false;}
            }

            true
        }
    }

    // Implementation tests.
    quickcheck! {
        
        fn is_empty_new() -> bool {
            (Heap::new() as Heap<u32>).is_empty() == true
        }

        fn size_new() -> bool {
            (Heap::new() as Heap<u32>).size() == 0
        }

        fn height_new() -> bool {
            (Heap::new() as Heap<u32>).height() == 0
        }

        fn is_heap_new() -> bool {
            Heap::is_heap((Heap::new() as Heap<u32>).arr())
        }

        fn is_empty_make_new_heap_bottom_up(v: Vec<u32>) -> bool {
            let b = v.is_empty();
            let h = Heap::make_heap_bottom_up(v);
            h.is_empty() == b
        }

        fn size_make_new_heap_bottom_up(v: Vec<u32>) -> bool {
            let len = v.len();
            let h = Heap::make_heap_bottom_up(v);
            h.size() == len
        }

        fn is_heap_make_new_heap_bottom_up(v: Vec<u32>) -> bool {
            let h = Heap::make_heap_bottom_up(v);
            Heap::is_heap(h.arr())
        }

        fn is_empty_make_new_heap_top_down(v: Vec<u32>) -> bool {
            let b = v.is_empty();
            let h = Heap::make_heap_top_down(v);
            h.is_empty() == b
        }
        fn size_make_new_heap_top_down(v: Vec<u32>) -> bool {
            let len = v.len();
            let h = Heap::make_heap_top_down(v);
            h.size() == len
        }

        fn is_heap_make_new_heap_top_down(v: Vec<u32>) -> bool {
            let h = Heap::make_heap_top_down(v);
            Heap::is_heap(h.arr())
        }

        fn is_heap_arbitrary(h: Heap<u32>) -> bool {
            Heap::is_heap(h.arr())
        }

        fn is_heap_ins(h: Heap<u32>, i: u32) -> bool {
            Heap::is_heap(h.clone().ins(i).arr())
        }

        fn is_heap_del_min(h: Heap<u32>) -> bool {
            let mut h2 = h.clone();
            h2.del_min();
            Heap::is_heap(h2.arr())
        }

        fn sort_is_sorted(v: Vec<u32>) -> bool {
            let mut v = v.clone();
            Heap::sort(&mut v);
            is_sorted(&v)
        }

        fn sort2_is_sorted(v: Vec<u32>) -> bool {
            let v = Heap::sort2(v);
            is_sorted(&v)
        }
    }
}