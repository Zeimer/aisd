use std::ops::Index;
use std::f64;

// A binary heap implemented using an array (or rather a vector).
#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    array: Vec<T>
}

impl<T: PartialOrd> Heap<T> {
    /// Create an empty `Heap`.
    pub fn new() -> Heap<T> {
        Heap {array: vec![]}
    }

    /// Get a reference to the heap's inner array.
    /// Time: O(1)
    pub fn arr(&self) -> &Vec<T> {
        &self.array
    }

    /// Compute the number of elements in the heap.
    pub fn size(&self) -> usize {
        self.array.len()
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

    /// Insert a new element into the rightmost leaf of the heap.
    /// Time: O(height of the heap)
    pub fn insert(&mut self, elem: T) {
        self.array.push(elem);
        let l = self.size() - 1; // Doesn't overflow because of previous line.
        self.fix_heap_property_bottom_up(l);
    }

    /// Like `insert`, but allows chaining.
    pub fn ins(&mut self, elem: T) -> &mut Self {
        self.insert(elem);
        self
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

    /// Remove the least element from the heap and return it.
    /// Time: O(height of the heap)
    pub fn del_min(&mut self) -> Option<T> {
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

    /// Return a reference to the least element in the heap.
    /// Time: O(1)
    pub fn min(&self) -> Option<&T> {
        if self.size() == 0 {
            None
        } else {
            Some(self.array.index(0))
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
    pub fn wut_make_heap(v: Vec<T>) -> Heap<T> {
        if v.len() == 0 {
            Heap::new()
        } else {
            let mut h = Heap {array: v};
            let end = if h.size() % 2 == 0 {h.size() - 2} else {h.size() - 1};
            
            for i in (2 .. end).filter(|i| i % 2 == 0).rev() {
                let min = if h.array[i] < h.array[i - 1] {i} else {i - 1};
                let parent = (i - 1)/2;

                if h.array[min] < h.array[parent] {
                    h.fix_heap_property_top_down(parent);
                }
            }

            let s = h.size();

            if s % 2 == 0 {
                h.fix_heap_property_bottom_up(s - 1);
            }

            h

        }
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

/// A heap is also an iterator (`next` is `del_min`).
impl<T: PartialOrd> Iterator for Heap<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.del_min()
    }
}