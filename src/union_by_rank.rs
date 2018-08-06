//! The classical data structure for the disjoint-set problem, also known as union-find,
//! where union is weighted by rank.

use quickcheck::Arbitrary;
use quickcheck::Gen;

use rand;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>
}

impl UnionFind {
    /// Creates a new `UnionFind` structure of the given `size`.
    pub fn new(size: usize) -> UnionFind {
        let mut parents = vec![];
        let mut ranks = vec![];

        for i in 0 .. size {
            parents.push(i);
            ranks.push(0);
        }

        UnionFind {
            parents,
            ranks
        }
    }

    /// Returns the number of elements of the structure (not the number of distinct sets!).
    pub fn size(&self) -> usize {
        self.parents.len()
    }

    /// Joins together the sets to which `i` and `j` belong.    
    pub fn union(&mut self, i: usize, j: usize) {
        match (self.find(i), self.find(j)) {
            (Some(pi), Some(pj)) if pi != pj => {
                if self.ranks[pi] < self.ranks[pj] {
                    self.parents[pi] = pj;
                } else {
                    self.parents[pj] = pi;
                    if self.ranks[pi] == self.ranks[pj] {
                        self.ranks[pi] += 1;
                    }
                }
            },
            _ => {}
        }
    }

    /// Finds the representative of the set to which `i` belongs.
    pub fn find(&mut self, i: usize) -> Option<usize> {
        if i >= self.size() {
            None
        } else {
            loop {
                if self.parents[i] == self.parents[self.parents[i]] {
                    return Some(self.parents[i]);
                } else {
                    self.parents[i] = self.parents[self.parents[i]];
                }
            }
        }
    }
}

impl Arbitrary for UnionFind {
    fn arbitrary<G: Gen>(g: &mut G) -> UnionFind {
        let size: usize = Arbitrary::arbitrary(g);
        let mut uf = UnionFind::new(size);
        
        let mut rng = rand::thread_rng();

        if size != 0 {
            for _ in 0 .. rng.gen_range(0, size) {
                let i = rng.gen_range(0, size);
                let j = rng.gen_range(0, size);
            
                uf.union(i, j);
            }
        }

        uf
    }
}

#[cfg(test)]
mod tests {
    use union_by_rank::*;

    // Interface tests.
    quickcheck! {
        fn union_find(uf: UnionFind) -> bool {
            let mut uf = uf.clone();

            if uf.size() == 0 {
                true
            } else {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0, uf.size());
                let j = rng.gen_range(0, uf.size());

                uf.union(i, j);
                uf.find(i) == uf.find(j)
            }
        }
    }

    // Implementation tests.
    quickcheck! {
        // The size of a new structure is given by the argument.
        fn size_new(size: usize) -> bool {
            let uf = UnionFind::new(size);

            uf.size() == size
        }

        // Calling `union` doesn't change the structure's size.
        fn size_union(uf: UnionFind) -> bool {
            let mut uf = uf.clone();
            let size = uf.size();

            if size == 0 {
                true
            } else {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0, uf.size());
                let j = rng.gen_range(0, uf.size());

                uf.union(i, j);
                uf.size() == size
            }

        }

        // Calling `union` doesn't change the structure's size.
        fn size_union2(uf: UnionFind, i: usize, j: usize) -> bool {
            let mut uf = uf.clone();
            let size = uf.size();

            if size == 0 {
                true
            } else {

                uf.union(i, j);
                uf.size() == size
            }
        }

        // Calling `find` doesn't change the structure's size.
        fn size_find(uf: UnionFind, i: usize) -> bool {
            let mut uf = uf.clone();
            let size = uf.size();

            uf.find(i);
            uf.size() == size
        }

        // Looking an element up in a brand new `UnionFind` structure returns it as
        // the representative of its set.
        fn find_new(size: usize) -> bool {
            let mut uf = UnionFind::new(size);

            if size == 0 {
                true
            } else {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0, uf.size());

                uf.find(i) == Some(i)
            }
        }
    }
}