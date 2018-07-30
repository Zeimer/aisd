//! Priority queues.

use std::ops::Index;
use std::f64;

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
    pub fn is_heap(v: &Vec<T>) -> bool {
        if v.len() > 0 {
            let last = v.len() - 1;

            // Iterate over all nodes that have children and check if their
            // values are less than these of their children.
            for i in 0 .. last {
                let l = 2 * i + 1;
                let r = 2 * i + 2;

                if r <= last {
                    if v[i] > v[l] || v[i] > v[r] {
                        return false;
                    }
                } else if l <= last {
                    if v[i] > v[l] {
                        return false;
                    }
                }
            }
        }

        true
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

    #[test]
    fn size_ok() {
        let mut h = Heap::new();

        h.insert(42);
        h.insert(12);
        h.insert(3);

        assert_eq!(h.size(), 3);
    }

    #[test]
    fn new_height() {
        let h: Heap<u32> = Heap::new();
        assert_eq!(h.height(), 0);
    }

    #[test]
    fn singleton_height() {
        let h = Heap::make_heap_bottom_up(vec![42]);
        assert_eq!(h.height(), 1);
    }

    #[test]
    fn two_height() {
        let h = Heap::make_heap_bottom_up(vec![42, 42]);
        assert_eq!(h.height(), 2);
    }

    #[test]
    fn four_height() {
        let h = Heap::make_heap_bottom_up(vec![42, 42, 42, 42]);
        assert_eq!(h.height(), 3);
    }

    #[test]
    fn new_is_heap() {
        let h: Heap<u32> = Heap::new();
        assert!(Heap::is_heap(h.arr()));
    }

    #[test]
    fn ins_is_heap() {
        let mut h = Heap::new();
        h.ins(6).ins(4).ins(1).ins(7).ins(9).ins(3).ins(1);
        assert!(Heap::is_heap(h.arr()));
    }

    #[test]
    fn make_heap_bottom_up_is_heap() {
        let v = vec![6, 4, 1, 7, 9, 3, 1];
        let h = Heap::make_heap_bottom_up(v);
        assert!(Heap::is_heap(h.arr()));
    }

    #[test]
    fn make_heap_top_down_is_heap() {
        let h = Heap::make_heap_top_down(vec![6, 4, 1, 7, 9, 3, 1]);
        println!("h = {:?}", h);
        assert!(Heap::is_heap(h.arr()));
    }

    #[test]
    fn sort_is_sorted() {
        let mut v = vec![6, 4, 1, 7, 9, 3, 1];
        Heap::sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn sort2_is_sorted() {
        let v = vec![6, 4, 1, 7, 9, 3, 1];
        assert!(is_sorted(&Heap::sort2(v)));
    }
}