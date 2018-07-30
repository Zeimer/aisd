//! Greedy algorithm for the change making problem.

/// We have the following problem: we have some coins and we want to make from
/// them the given amount using as few coins as possible. This function implements
/// a greedy algorithm that works only for the so-called canonical coin systems.
/// Note that the coins have to be sorted in descending order.
pub fn make_change(coins: &mut Vec<u32>, amount: u32) -> Option<Vec<u32>> {
    let mut v = vec![];
    let mut remaining = amount;

    loop {
        match coins.pop() {
            Some(c) if c <= remaining => {
                v.push(c);
                remaining -= c;
            },
            Some(_) => continue,
            None => break
        }
    }

    if remaining == 0 {Some(v)} else {None}
}