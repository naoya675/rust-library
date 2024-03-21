// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use proconio::input;
use std::cmp::min;

use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut st = SegmentTree::new(n, |a, b| min(a, b), (1 << 31) - 1);
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            0 => {
                input! {
                    x: usize,
                    y: usize,
                }
                st.set(x, y);
            }
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                println!("{}", st.prod(x, y + 1));
            }
            _ => unreachable!(),
        }
    }
}
