//! A crate with various algorithms (don't expect much).

pub mod pq;
pub mod depq;

pub mod coin_change;

pub mod union_by_size;
pub mod union_by_rank;

pub mod map;

#[macro_use]
extern crate quickcheck;

extern crate rand;