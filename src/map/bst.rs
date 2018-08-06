use map::Map;

use quickcheck::Arbitrary;
use quickcheck::Gen;

#[derive(Clone, Debug)]
pub enum Tree<K, V> {
    E,
    N(K, V, Box<Tree<K, V>>, Box<Tree<K, V>>)
}

use self::Tree::{E, N};

use std::cmp::Ord;
use std::cmp::Ordering::*;

impl<K, V> Tree<K, V> {
    fn size(&self) -> usize {
        match self {
            E => 0,
            N(_, _, l, r) => 1 + l.size() + r.size()
        }
    }
}

impl<K: Ord, V> Map for Tree<K, V> {
    type Key = K;
    type Value = V;

    fn new() -> Tree<K, V> {
        Tree::E
    }

    fn find(&self, key: &K) -> Option<&V> {
        match self {
            E => None,
            N(k, v, l, r) => match Ord::cmp(key, &k) {
                Less => l.find(key),
                Equal => Some(&v),
                Greater => r.find(key)
            }
        }
    }

    fn ins(&mut self, key: K, value: V) -> &mut Self {
        match self {
            E => {*self = N(key, value, Box::new(E), Box::new(E));},
            N(k, v, l, r) => match key.cmp(k) {
                Less => {l.ins(key, value);}
                Equal => {*v = value;}
                Greater => {r.ins(key, value);}
            }
        }

        self
    }

    fn del(&mut self, key: &K) -> Option<V> {
        None
    }
}

/*impl<T: Clone> Clone for Tree<T> {
    fn clone(&self) -> Self {
        match self {
            E => E,
            N(v, l, r) => N(v.clone(), l.clone(), r.clone())
        }
    }
}*/

impl<K: Ord + Arbitrary, V: Arbitrary> Arbitrary for Tree<K, V> {
    fn arbitrary<G: Gen>(g: &mut G) -> Tree<K, V> {
        let data: Vec<(K, V)> = Arbitrary::arbitrary(g);
        
        let mut t = E;
        for (k, v) in data {
            t.ins(k, v);
        }

        t
    }
}

#[cfg(test)]
mod tests {
    use map::Map;
    use super::Tree;
    use super::Tree::*;

    quickcheck! {
        fn find_ins(t: Tree<usize, usize>, k: usize, v: usize) -> bool {
            let mut t = t.clone();
            t.ins(k, v);
            t.find(&k) == Some(&v)
        }

        fn del_ins(t: Tree<usize, usize>, k: usize, v: usize) -> bool {
            let mut t = t.clone();
            t.ins(k, v);
            t.del(&k) == Some(v)
        }

        fn find_is_del(t: Tree<usize, usize>, k: usize) -> bool {
            let mut t1 = t.clone();
            let mut t2 = t.clone();

            let f = t1.find(&k);
            let d = t2.del(&k);

            match (f, d) {
                (Some(&v1), Some(v2)) => v1 == v2,
                (_, _) => false
            }
        }

        fn find_del(t: Tree<usize, usize>, k: usize) -> bool {
            let mut t = t.clone();
            t.del(&k);

            t.find(&k) == None
        }

        fn find_new(k: usize) -> bool {
            (Tree::new() as Tree<usize, usize>).find(&k) == None
        }

        fn del_new(k: usize) -> bool {
            (Tree::new() as Tree<usize, usize>).del(&k) == None
        }

        /*fn ins_new(k: usize, v: usize) -> bool {
            *(Tree::new() as Tree<usize, usize>).ins(k, v) ==
            N(k, v, Box::new(E), Box::new(E))
        }*/

        fn size_new() -> bool {
            (Tree::new() as Tree<usize, usize>).size() == 0
        }

        // źle
        fn size_ins(t: Tree<usize, usize>, k: usize, v: usize) -> bool {
            let mut t = t.clone();
            let n = t.size();

            //n + 1 == t.ins(k, v).size()
            t.ins(k, v).size() >= n
        }

        fn size_del(t: Tree<usize, usize>, k: usize) -> bool {
            let mut t = t.clone();
            let n = t.size();
            t.del(&k);

            t.size() <= n
        }
    }
}