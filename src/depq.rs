//! Double-ended priority queues.

use std::u32;

use quickcheck::Arbitrary;
use quickcheck::Gen;

use pq::Heap;

/// **DEPQ** stands for **Double-ended priority queue**. It is a priority queue
/// that provides access to both the minimal and maximal elements, but not
/// to the middle ones. It is possible to implement it so that minimal and
/// maximal elements can be accessed in constant time and removing them, as
/// well as inserting new elements, takes logarithmic time.
/// 
/// # Example
/// 
/// ```
/// extern crate aisd;
/// use aisd::depq::*;
/// 
/// // A new queue has to be created using the implementing type's new() method.
/// let mut q = DoubleHeap::new();
/// 
/// // We can insert elements into the queue by calling `ins`.
/// q.ins(1);
/// 
/// // We can insert faster by chaining calls to `ins`.
/// q.ins(2).ins(3).ins(4);
/// 
/// // We can insert many items at once by calling `ins_all`.
/// q.ins_all(vec![5, 6, 7]);
/// 
/// // All of these can be chained:
/// q.ins(8).ins_all(vec![9, 10, 11]).ins(12);
/// 
/// // Alternatively, you can use [make_heap] to turn a vector into a heap. Note that
/// // using [make_heap] is faster than calling [ins_all] on an empty heap.
/// 
/// let mut q = DoubleHeap::make_heap(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
/// 
/// // When we have some queue, we can check if it's empty, see how many
/// // elements it has and retrieve and remove the minimal and maximal
/// // elements.
/// assert!(!q.is_empty());
/// assert_eq!(q.size(), 12);
/// assert_eq!(*q.min().unwrap(), 1);
/// assert_eq!(*q.max().unwrap(), 12);
/// ```
pub trait DEPQ {
    type Item;

    /// Check if the queue is empty.
    fn is_empty(&self) -> bool;

    /// Get the number of items in the queue.
    fn size(&self) -> usize;

    /// Inserts an item into the queue. Allows chaining calls.
    fn ins(&mut self, item: Self::Item) -> &mut Self;

    /// Insert all elements from the vector by repeatedly calling `ins`.
    fn ins_all(&mut self, v: Vec<Self::Item>) -> &mut Self {
        for x in v {
            self.ins(x);
        }
        self
    }

    /// Get a reference to the minimal element of the queue.
    fn min(&self) -> Option<&Self::Item>;

    /// Get a reference to the maximal element of the queue.
    fn max(&self) -> Option<&Self::Item>;

    /// Remove the minimal element from the queue.
    fn del_min(&mut self) -> Option<Self::Item>;

    /// Remove the maximal element from the queue.
    fn del_max(&mut self) -> Option<Self::Item>;
}

/// A data structure made from a min-heap and a max-heap.
/// Each of the heaps contains all elements in the double heap.
/// Each element in each heap points to its position in the other heap.
#[derive(Debug)]
pub struct DoubleHeap<T> {
    min_array: Vec<(T, usize)>,
    max_array: Vec<(T, usize)>
}

impl<T: PartialOrd + Clone> DoubleHeap<T> {
    /// Creates a new `DoubleHeap`.
    pub fn new() -> DoubleHeap<T> {
        DoubleHeap {
            min_array: vec![],
            max_array: vec![]
        }
    }

    /// Swap two elements in the left heap while maintaining pointers in the right heap.
    fn swap(l: &mut Vec<(T, usize)>, r: &mut Vec<(T, usize)>, i: usize, j: usize) {
        r[l[i].1].1 = j;
        r[l[j].1].1 = i;
        l.swap(i, j);
    }

    /// Make sure that heap property is satisfied on the path from the i-th
    /// element of the heap (counting breadth-first) to the root.
    fn fix_heap_property_bottom_up_aux
        (l: &mut Vec<(T, usize)>, r: &mut Vec<(T, usize)>,
         i: usize, cmp: fn(&T, &T) -> bool) {

        // Start from the i-th element.
        let mut current = i;
        while current != 0 {
            // Compute parent's position.
            let parent = (current - 1)/2;

            if cmp(&l[current].0, &l[parent].0) {
                DoubleHeap::swap(l, r, current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }
    
    /// Like above, but fix both heaps.
    fn fix_heap_property_bottom_up(&mut self, min_i: usize, max_i: usize) {
        DoubleHeap::fix_heap_property_bottom_up_aux(
            &mut self.min_array, &mut self.max_array, min_i, PartialOrd::lt);
        DoubleHeap::fix_heap_property_bottom_up_aux(
            &mut self.max_array, &mut self.min_array, max_i, PartialOrd::gt);
    }

    /// Sink the i-th node in the left heap towards leafs while maintaining pointers
    /// in the right heap.
    fn fix_heap_property_top_down_aux
        (l: &mut Vec<(T, usize)>, r: &mut Vec<(T, usize)>, i: usize,
         lt: fn(&T, &T) -> bool, gt: fn(&T, &T) -> bool) {

        // Start from the i-th node.
        let mut current = i;
        loop {
            // Compute children's positions.
            let left = 2 * current + 1;
            let right = 2 * current + 2;

            // Current node has both children.
            if l.len() > right {
                // Check which child is smaller.
                let child = if lt(&l[left].0, &l[right].0) {left} else {right};

                // If the child is smaller, swap it with current node.
                if gt(&l[current].0, &l[child].0) {
                    DoubleHeap::swap(l, r, current, child);
                    current = child;
                } else {
                    break;
                }
            // Current node has only left child.
            } else if l.len() > left {
                // If left child is smaller, swap it with current node.
                if gt(&l[current].0, &l[left].0) {
                    DoubleHeap::swap(l, r, current, left);
                    current = left;
                } else {
                    break;
                }
            // No children - we're at the bottom of the heap, so we can terminate.
            } else {
                break;
            }
        }
    }
    
    /// Remove left heap's least element from both heaps.
    fn del_aux(
        l: &mut Vec<(T, usize)>, r: &mut Vec<(T, usize)>,
        cmpl: fn(&T, &T) -> bool, cmpr: fn(&T, &T) -> bool) -> Option<T>
    {
        // Cases: 0, 1 or more elements.
        if l.len() == 0 {
            None
        } else if l.len() == 1 {
            let result = Some(l.pop().unwrap().0);
            *l = vec![];
            *r = vec![];

            result
        } else {
            // Compute the position of the last element.
            let last = l.len() - 1;
            
            // Swap the first and the last elements in the left heap.
            DoubleHeap::swap(l, r, 0, last);

            // Remove the last (so the old first) element from the left heap.
            let (result, ri) = l.pop().unwrap();

            // Remove the same element from the right heap by swapping it with the last
            // element in this heap.
            if ri == last {
                r.pop();
            } else {
                l[r[last].1].1 = ri;
                r.swap(ri, last);
                r.pop();
            }

            // Restore the heap property at the root.
            DoubleHeap::fix_heap_property_top_down_aux(l, r, 0, cmpl, cmpr);

            // If the deleted element in the right heap wasn't the last one, restore
            // the heap property by going upwards.
            if ri != last {
                DoubleHeap::fix_heap_property_bottom_up_aux(r, l, ri, cmpr);
            }

            Some(result)
        }
    }

    /// Make a `DobuleHeap` from a vector. Note that this is fast than using
    /// the method `ins_all` from DEPQ.
    pub fn make_heap(v: Vec<T>) -> DoubleHeap<T> {
        // Create two identical trees whose elements point to each others copy.
        let mut h = DoubleHeap::new();
        h.min_array = v.clone().into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        h.max_array = v.into_iter().enumerate().map(|(i, x)| (x, i)).collect();

        // Fix the heap property from the current node downwards, going from the last
        // node to the first (root).
        let s = h.size();
        for i in (0 .. s).rev() {
            DoubleHeap::fix_heap_property_top_down_aux
                (&mut h.min_array, &mut h.max_array, i, PartialOrd::lt, PartialOrd::gt);
            DoubleHeap::fix_heap_property_top_down_aux
                (&mut h.max_array, &mut h.min_array, i, PartialOrd::gt, PartialOrd::lt);
        }

        h
    }

    /// A helper method that checks if this `DoubleHeap`'s components are really a
    /// min-heap and a max-heap. Used for testing. 
    fn is_heap(&self) -> bool {
        Heap::is_heap_aux(
            &self.min_array.clone().into_iter().map(|x| x.0).collect(), PartialOrd::gt) &&
        Heap::is_heap_aux(
            &self.max_array.clone().into_iter().map(|x| x.0).collect(), PartialOrd::lt)
    }
}

impl<T: PartialOrd + Clone> DEPQ for DoubleHeap<T> {
    type Item = T;

    fn is_empty(&self) -> bool {
        self.min_array.is_empty()
    }

    fn size(&self) -> usize {
        self.min_array.len()
    }

    fn ins(&mut self, item: T) -> &mut Self {
        let i = self.min_array.len();

        self.min_array.push((item.clone(), i));
        self.max_array.push((item, i));

        self.fix_heap_property_bottom_up(i, i);
        self
    }

    fn min(&self) -> Option<&T> {
        if self.size() == 0 {None} else {Some(&self.min_array[0].0)}
    }

    fn max(&self) -> Option<&T> {
        if self.size() == 0 {None} else {Some(&self.max_array[0].0)}
    }

    fn del_min(&mut self) -> Option<T> {
        DoubleHeap::del_aux(&mut self.min_array, &mut self.max_array,
                            PartialOrd::lt, PartialOrd::gt)
    }

    fn del_max(&mut self) -> Option<T> {
        DoubleHeap::del_aux(&mut self.max_array, &mut self.min_array,
                            PartialOrd::gt, PartialOrd::lt)
    }
}

/// Two `DoubleHeap`s are equal if they have the same elements. Checking this takes
/// O(nlogn) time.
impl<T: PartialOrd + Clone> PartialEq for DoubleHeap<T> {
    fn eq(&self, rhs: &Self) -> bool {
        Heap::sort2(self.min_array.clone().into_iter().map(|x| x.0).collect()) ==
        Heap::sort2(rhs.min_array.clone().into_iter().map(|x| x.0).collect())
    }
}

impl<T: PartialOrd + Clone> Clone for DoubleHeap<T> {
    fn clone(&self) -> Self {
        DoubleHeap {
            min_array: self.min_array.clone(),
            max_array: self.max_array.clone()
        }
    }
}

/// This is used for shrinking `DoubleHeap`s in quickcheck tests.
struct DHIter<T: PartialOrd + Clone>(DoubleHeap<T>);

impl<T: PartialOrd + Clone> Iterator for DHIter<T> {
    type Item = DoubleHeap<T>;

    /// Shrink the `DoubleHeap` by popping from its min-heap and rebuilding.
    fn next(&mut self) -> Option<DoubleHeap<T>> {
        let mut v: Vec<T> = self.0.min_array.clone().into_iter().map(|x| x.0).collect();
        match v.pop() {
            None => None,
            _ => {
                self.0 = DoubleHeap::new();
                self.0.ins_all(v);
                Some(self.0.clone())

            }
        }
    }
}

impl<T: PartialOrd + Arbitrary + Clone> Arbitrary for DoubleHeap<T> {
    fn arbitrary<G : Gen>(g: &mut G) -> Self {
        DoubleHeap::make_heap(Arbitrary::arbitrary(g))
    }

    fn shrink(&self) -> Box<Iterator<Item = Self>> {
        Box::new(DHIter(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use depq::*;

    // Interface tests.
    quickcheck! {
        fn is_empty_size(h: DoubleHeap<u32>) -> bool {
            h.is_empty() == (h.size() == 0)
        }

        fn is_empty_ins(h: DoubleHeap<u32>, item: u32) -> bool {
            let mut h2 = h.clone();
            h2.ins(item);
            h2.is_empty() == false
        }

        fn is_empty_min(h: DoubleHeap<u32>) -> bool {
            (h.is_empty() == false) == (h.clone().min() != Option::None)
        }

        fn is_empty_del_min(h: DoubleHeap<u32>) -> bool {
            (h.is_empty() == false) == (h.clone().del_min() != Option::None)
        }

        fn is_empty_max(h: DoubleHeap<u32>) -> bool {
            (h.is_empty() == false) == (h.clone().max() != Option::None)
        }

        fn is_empty_del_max(h: DoubleHeap<u32>) -> bool {
            (h.is_empty() == false) == (h.clone().del_max() != Option::None)
        }

        fn size_ins(h: DoubleHeap<u32>, i: u32) -> bool {
            h.size() + 1 == h.clone().ins(i).size()
        }
        
        fn size_min(h: DoubleHeap<u32>) -> bool {
            (h.size() != 0) == (h.clone().min() != Option::None)
        }
        
        fn size_max(h: DoubleHeap<u32>) -> bool {
            (h.size() != 0) == (h.clone().max() != Option::None)
        }

        fn size_del_min(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            match h2.del_min() {
                Some(_) => h.size() == h2.size() + 1,
                None => h.size() == h2.size()
            }
        }

        fn size_del_max(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            match h2.del_max() {
                Some(_) => h.size() == h2.size() + 1,
                None => h.size() == h2.size()
            }
        }

        fn ins_min(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();

            h2.ins(0).min() == Some(&0)
        }

        fn ins_max(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();

            h2.ins(u32::MAX).max() == Some(&u32::MAX)
        }

        fn ins_del_min(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            let h3 = h.clone();

            let item = h2.del_min();

            match item {
                Some(i) => *h2.ins(i) == h3,
                None => h2 == h3
            }
        }

        fn ins_del_max(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            let h3 = h.clone();

            let item = h2.del_max();

            match item {
                Some(i) => *h2.ins(i) == h3,
                None => h2 == h3
            }
        }

        fn min_del_min(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            match (h.min(), h2.del_min()) {
                (Some(m1), Some(m2)) => *m1 == m2,
                (None, None) => true,
                (_, _) => false
            }
        }

        fn max_del_max(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            match (h.max(), h2.del_max()) {
                (Some(m1), Some(m2)) => *m1 == m2,
                (None, None) => true,
                (_, _) => false
            }
        }

        fn del_min_least(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            let m = match h2.del_min() {
                Some(m) => m,
                None => {return true;}
            };

            for x in h2.min_array {
                if x.0 < m {return false;}
            }

            true
        }

        fn del_max_greatest(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            let m = match h2.del_max() {
                Some(m) => m,
                None => {return true;}
            };

            for x in h2.max_array {
                if x.0 > m {return false;}
            }

            true
        }
    }

    // Implementation tests.
    quickcheck! {
        
        fn is_empty_new() -> bool {
            (DoubleHeap::new() as DoubleHeap<u32>).is_empty() == true
        }

        fn size_new() -> bool {
            (DoubleHeap::new() as DoubleHeap<u32>).size() == 0
        }

        fn is_heap_new() -> bool {
            (DoubleHeap::new() as DoubleHeap<u32>).is_heap()
        }

        fn is_empty_make_heap(v: Vec<u32>) -> bool {
            let b = v.is_empty();
            let h = DoubleHeap::make_heap(v);
            h.is_empty() == b
        }

        fn size_make_heap(v: Vec<u32>) -> bool {
            let len = v.len();
            let h = DoubleHeap::make_heap(v);
            h.size() == len
        }

        fn is_heap_make_heap(v: Vec<u32>) -> bool {
            let h = DoubleHeap::make_heap(v);
            h.is_heap()
        }

        fn is_heap_arbitrary(h: DoubleHeap<u32>) -> bool {
            h.is_heap()
        }

        fn is_heap_ins(h: DoubleHeap<u32>, i: u32) -> bool {
            h.clone().ins(i).is_heap()
        }

        fn is_heap_del_min(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            h2.del_min();
            h2.is_heap()
        }

        fn is_heap_del_max(h: DoubleHeap<u32>) -> bool {
            let mut h2 = h.clone();
            h2.del_max();
            h2.is_heap()
        }
    }
}