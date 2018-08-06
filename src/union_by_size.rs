//! The classical data structure for the disjoint-set problem, also known as union-find,
//! where union is weighted by size.

use quickcheck::Arbitrary;
use quickcheck::Gen;

use rand;
use rand::Rng;

/// The classical data structure for the disjoint-set problem, also known as union-find.
///
/// # Example
/// 
/// ```
/// extern crate aisd;
/// use aisd::union_by_size::UnionFind;
/// 
/// // Create a new structure containing the numbers {0, ..., 41}.
/// let mut uf = UnionFind::new(42);
/// 
/// // We can look up to which set a number belongs by calling `find`. If a number is not
/// // present in the structure, `None` is returned.
/// assert_eq!(uf.find(100500), None);
/// 
/// // Otherwise `Some(p)` is returned, where `p` is the representative of the set to which
/// // the number belongs. `uf.find(i) == Some(j)` may be read as "i belongs to the same
/// // set as j". At the beginning, each number belongs to its own singleton set.
/// assert_eq!(uf.find(13), Some(13));
/// 
/// // We may join sets together by calling `union`. `uf.union(i, j)` may be read as "join
/// // together the sets to which i and j belong".
/// uf.union(13, 0);
/// 
/// // Now 13 belongs to the same set as 0, which is reflected by `find`.
/// assert_eq!(uf.find(13), Some(0));
/// 
/// // However, when looking up 0, `uf` won't tell us that it belongs to the same set as
/// // 13. This is because each set has one and only one representative.
/// assert_eq!(uf.find(0), Some(0));
/// 
/// // To make sure that two numbers belongs to the same set, we have to compare their
/// // respective sets' representatives
/// assert_eq!(uf.find(0), uf.find(13));
/// ```
#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>
}

impl UnionFind {
    /// Creates a new `UnionFind` structure of the given `size`.
    pub fn new(size: usize) -> UnionFind {
        let mut parents = vec![];
        let mut sizes = vec![];

        for i in 0 .. size {
            parents.push(i);
            sizes.push(1);
        }

        UnionFind {
            parents,
            sizes
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
                if self.sizes[pi] <= self.sizes[pj] {
                    self.parents[pi] = pj;
                    self.sizes[pj] += self.sizes[pi];
                    self.sizes[pi] = 0;
                } else {
                    self.parents[pj] = pi;
                    self.sizes[pi] += self.sizes[pj];
                    self.sizes[pj] = 0;
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
    use union_by_size::*;

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

        // Sizes of all the sets sum up to the number of elements of the
        // `UnionFind` structure.
        fn sum_of_sizes(uf: UnionFind) -> bool {
            let mut sum = uf.size();
            for size in uf.sizes {
                sum -= size;
            }

            sum == 0
        }
    }
}