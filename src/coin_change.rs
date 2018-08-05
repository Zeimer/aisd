//! Greedy algorithm for the change making problem.

use std::collections::HashSet;
use std::u32;

/// We have the following problem: we have some coins and we want to select the
/// smallest subset that sums to the given amount. This function implements a
/// greedy algorithm that works only for the so-called canonical coin systems.
/// Note that the coins have to be sorted in ascending order.
/// 
/// # Example
/// 
/// ```
/// extern crate aisd;
/// use aisd::coin_change::make_change;
/// 
/// // For this coin system, we will get optimal solutions.
/// let mut coins = vec![1, 1, 2, 2, 2, 5, 5, 10, 10, 10];
/// 
/// assert_eq!(make_change(coins.clone(), 27), Some(vec![10, 10, 5, 2]));
/// assert_eq!(make_change(coins, 49), None);
/// 
/// // Note that the coins are considered to be unique.
/// let mut coins = vec![10];
/// 
/// assert_eq!(make_change(coins, 20), None);
/// 
/// // For a non-canonical coin system, we get suboptimal solutions.
/// let mut coins = vec![3, 5, 9, 9, 10];
/// 
/// // The optimal solution is Some(vec![9, 9]).
/// assert_eq!(make_change(coins, 18), Some(vec![10, 5, 3]));
/// ```
pub fn make_change(mut coins: Vec<u32>, mut amount: u32) -> Option<Vec<u32>> {
    let mut v = vec![];

    loop {
        match coins.pop() {
            Some(c) if c <= amount => {
                v.push(c);
                amount -= c;
            },
            Some(_) => continue,
            None => break
        }
    }

    if amount == 0 {Some(v)} else {None}
}

/// Like `make_change`, but coins are not considered to be unique.
/// 
/// # Example
/// 
/// ```
/// extern crate aisd;
/// use aisd::coin_change::make_change2;
/// 
/// let mut coins = vec![10];
/// 
/// assert_eq!(make_change2(coins, 20), Some(vec![10, 10]));
/// ```
pub fn make_change2(coins: Vec<u32>, mut amount: u32) -> Option<Vec<u32>> {
    let mut v = vec![];

    for mut i in 0 .. coins.len() {
        while coins[i] <= amount {
            v.push(coins[i]);
            amount -= coins[i];
        }
    }

    if amount == 0 {Some(v)} else {None}
}

/// Computes the least number of coins from the given set that sum up to the
/// given amount.
/// 
/// # Example
/// 
/// ```
/// extern crate aisd;
/// use aisd::coin_change::make_change_count;
/// 
/// use std::collections::HashSet;
/// 
/// let mut coins = HashSet::new();
/// coins.insert(2);
/// coins.insert(5);
/// 
/// assert_eq!(make_change_count(&coins, 7), Some(2));
/// assert_eq!(make_change_count(&coins, 3), None);
/// ```
pub fn make_change_count(coins: &HashSet<usize>, amount: usize) -> Option<usize> {
    let mut dp = vec![];
    dp.push(Some(0));

    for i in 1 .. (amount + 2) {
        //println!("{:?}", dp);
        if coins.contains(&i) {
            dp.push(Some(1));
            continue;
        } else {
            dp.push(None);
        }
        for j in 0 .. i {
            match dp[j] {
                Some(vj) if coins.contains(&(i - j)) => {
                    match dp[i] {
                        Some(vi) => {
                            if vj + 1 < vi {
                                dp[i] = Some(vj + 1);
                            }
                        },
                        None => {
                            dp[i] = Some(vj + 1);
                        }
                    }
                },
                _ => {}
            }
        }
    }

    dp[amount]
}