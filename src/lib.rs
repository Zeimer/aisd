pub mod heap;

#[cfg(test)]
mod tests {
    use super::heap::Heap;

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
    fn wut_make_heap_is_heap() {
        let h = Heap::wut_make_heap(vec![6, 4, 1, 7, 9, 3, 1]);
        println!("h = {:?}", h);
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