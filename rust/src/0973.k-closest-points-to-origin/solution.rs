// Created by Po-Yeh Chen at 2024/12/24 18:10
// leetgo: 1.4.11
// https://leetcode.com/problems/k-closest-points-to-origin/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

fn distance((x, y): (i32, i32)) -> f64 {
    ((x.pow(2) + y.pow(2)) as f64).sqrt()
}

fn median(arr: &mut [(i32, i32)]) -> (i32, i32) {
    arr.sort_by(|&x, &y| distance(x).partial_cmp(&distance(y)).unwrap());
    arr[arr.len() >> 1]
}

fn pivot(points: &mut Vec<(i32, i32)>, l: usize, r: usize, k: usize) -> (i32, i32) {
    let n = r - l + 1;
    if k > 0 && k <= n {
        let mut meds = vec![(0, 0); (n + 4) / 5];
        let mut i = 0;
        for chnk in points[l..=r].chunks_mut(5) {
            meds[i] = median(chnk);
            i += 1;
        }

        let med_of_med = if i == 1 {
            meds[0]
        } else {
            pivot(&mut meds, 0, i - 1, i >> 1)
        };

        let pos = partition(points, l, r, med_of_med);
        if pos - l == k - 1 {
            points[pos]
        } else if pos - l > k - 1 {
            pivot(points, l, pos - 1, k)
        } else {
            pivot(points, pos + 1, r, k - pos + l - 1)
        }
    } else {
        (i32::MAX, i32::MAX)
    }
}

fn partition(arr: &mut Vec<(i32, i32)>, l: usize, r: usize, piv: (i32, i32)) -> usize {
    for i in l..r {
        if arr[i] == piv {
            arr.swap(i, r);
            break;
        }
    }

    let mut i = l;

    for j in l..r {
        if distance(arr[j]) <= distance(piv) {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, r);
    i
}

// fn distance_vec(p: &Vec<i32>) -> f64 {
//     ((p[0] * p[0] + p[1] * p[1]) as f64).sqrt()
// }

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let size = points.len() - 1;
        if size == 0 {
            return points;
        } else if k as usize == points.len() {
            return points;
        }
        // Built-in, 3ms
        // let (lesser, _, _) = points.select_nth_unstable_by(k as usize, |x, y| {
        //     distance_vec(x).partial_cmp(&distance_vec(y)).unwrap()
        // });
        // lesser.to_vec()

        // Median of median impl, 4ms
        let size = points.len() - 1;
        if size == 0 {
            return points;
        } else if k as usize == points.len() {
            return points;
        }
        let mut mpoints: Vec<(i32, i32)> = points.into_iter().map(|v| (v[0], v[1])).collect();

        let piv = pivot(&mut mpoints, 0, size, k as usize);

        let pos = partition(&mut mpoints, 0, size, piv);

        mpoints[..pos + 1]
            .into_iter()
            .map(|&(x, y)| vec![x, y])
            .collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let points: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::k_closest(points, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
