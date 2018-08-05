use quickcheck::Arbitrary;
use quickcheck::Gen;

use rand;
use rand::Rng;

/// The idea is that, for union-find to work on elements of an arbitrary type T,
/// we need to make them easily convertible with `usize` so as to be able to
/// use the standard union-find algorithm. Therefore if somebody wants to use our
/// algorithms, he needs to implement this trait.
pub trait Findable {
    fn get_id(&self) -> usize;
    fn set_id(&mut self);
}

pub trait UnionFind {
    type Item;

    fn new() -> Self;
    fn ins(&mut self, x: &mut Self::Item);
    fn union(&mut self, x: &Self::Item, y: &Self::Item);
    fn find(&mut self, x: &Self::Item) -> Option<&Self::Item>;
}

pub struct UF<'a, T: 'a + Findable> {
    next_id: usize,
    items: Vec<&'a T>,
    parents: Vec<usize>,
    sizes: Vec<usize>
}

impl<'a, T: Findable> UnionFind for UF<'a, T> {
    type Item = T;

    fn new() -> UF<'a, T> {
        UF {
            next_id: 0,
            items: vec![],
            parents: vec![],
            sizes: vec![]
        }
    }

    fn ins(&mut self, x: &mut T) {

    }

    fn union(&mut self, x: &T, y: &T) {

    }

    fn find(&mut self, x: &T) -> Option<&T> {
        None
    }
}

impl<'a, T: Findable + Clone> Clone for UF<'a, T> {
    fn clone(&self) -> UF<'a, T> {
        UF {
            next_id: self.next_id,
            items: self.items.clone(),
            parents: self.parents.clone(),
            sizes: self.sizes.clone()
        }
    }
}

impl<T : Findable + Arbitrary + Clone + Sync> Arbitrary for UF<'static, T> {
    fn arbitrary<G: Gen>(g: &mut G) -> UF<'static, T> {
        let items: Vec<T> = Arbitrary::arbitrary(g);
        let n = items.len();

        let mut refs = Vec::new();
        for i in 0 .. n {
            refs.push(&items[i]);
        }

        let mut parents = Vec::new();
        for i in 0 .. n {
            parents.push(i);
        }

        let mut uf = UF {
            next_id: n,
            items: refs,
            parents,
            sizes: vec![1; n]
        };

        let mut rng = rand::thread_rng();

        for _ in 0 .. rng.gen_range(0, n) {
            let i = rng.gen_range(0, n);
            let j = rng.gen_range(0, n);
            
            let ti = &uf.items[i];
            let tj = &uf.items[j];
            uf.union(ti, tj);
        }

        uf
    }
}

