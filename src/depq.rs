//! Double-ended priority queues.

/// **DEPQ** stands for **Double-ended priority queue**. It is a priority queue
/// that provides access to both the minimal and maximal elements, but not
/// to the middle ones. It is possible to implement it so that minimal and
/// meximal elements can be accessed in constant time and removing them, as
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

    /// Insert a new item into the queue.
    //fn insert(&mut self, item: Self::Item) -> &mut Self;

    /// Like `insert`, but allows chaining calls.
    fn ins(&mut self, item: Self::Item) -> &mut Self; /* {
        self.insert(item);
        self
    }*/

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

#[derive(Debug)]
pub struct DoubleHeap<T> {
    min_array: Vec<(T, usize)>,
    max_array: Vec<(T, usize)>
}

impl<T: PartialOrd + Clone> DoubleHeap<T> {
    pub fn new() -> DoubleHeap<T> {
        DoubleHeap {
            min_array: vec![],
            max_array: vec![]
        }
    }

    fn fix_heap_property_bottom_up_aux
        (l: &mut Vec<(T, usize)>, r: &mut Vec<(T, usize)>,
         i: usize, cmp: fn(&T, &T) -> bool) {

        let mut current = i;
        while current != 0 {
            let parent = (current - 1)/2;

            if cmp(&l[current].0, &l[parent].0) {
                r[l[parent].1].1 = current;
                l.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
        r[i].1 = current;
    }
    
    fn fix_heap_property_bottom_up(&mut self, min_i: usize, max_i: usize) {
        DoubleHeap::fix_heap_property_bottom_up_aux(
            &mut self.min_array, &mut self.max_array, min_i, PartialOrd::lt);
        DoubleHeap::fix_heap_property_bottom_up_aux(
            &mut self.max_array, &mut self.min_array, max_i, PartialOrd::gt);
    }

    fn fix_heap_property_top_down_aux
        (l: &mut Vec<(T, usize)>, r: &mut Vec<(T, usize)>, i: usize,
         lt: fn(&T, &T) -> bool, gt: fn(&T, &T) -> bool) {

        let mut current = i;
        loop {
            let left = 2 * current + 1;
            let right = 2 * current + 2;

            if l.len() > right {
                let son = if lt(&l[left].0, &l[right].0) {left} else {right};

                if gt(&l[current].0, &l[son].0) {
                    r[l[son].1].1 = current;
                    r[l[current].1].1 = son;
                    l.swap(current, son);
                    current = son;
                } else {
                    break;
                }
            } else if l.len() > left {
                if gt(&l[current].0, &l[left].0) {
                    r[l[left].1].1 = current;
                    r[l[current].1].1 = left;
                    l.swap(current, left);
                    current = left;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    
    fn del_aux(
        l: &mut Vec<(T, usize)>, r: &mut Vec<(T, usize)>,
        cmpl: fn(&T, &T) -> bool, cmpr: fn(&T, &T) -> bool) -> Option<T> {
        if l.len() == 0 {
            None
        } else if l.len() == 1 {
            let result = Some(l.pop().unwrap().0);
            *l = vec![];
            *r = vec![];

            result
        } else {
            let last = l.len() - 1;
            
            r[l[last].1].1 = 0;
            l.swap(0, last);

            let (result, max_i) = l.pop().unwrap();

            if max_i == last {
                r.pop();
            } else {
                l[r[last].1].1 = max_i;
                r.swap(max_i, last);
                r.pop();
            }

            DoubleHeap::fix_heap_property_top_down_aux(l, r, 0, cmpl, cmpr);

            if max_i != last {
                DoubleHeap::fix_heap_property_top_down_aux(r, l, 0, cmpr, cmpl);
            }

            Some(result)
        }
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

#[cfg(test)]
mod tests {
    use depq::*;

    #[test]
    fn size_ok() {
        let mut h = DoubleHeap::new();

        h.ins_all(vec![42, 12, 3]);
        assert_eq!(h.size(), 3);
    }
}