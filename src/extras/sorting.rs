use super::*;
fn record(e: &mut Exercise, a: &[i64], active: &[usize], note: String) {
    let d = model::Diagram {
        nodes: a
            .iter()
            .enumerate()
            .map(|(i, n)| model::Node {
                label: n.to_string(),
                note: format!("bar · index {i}"),
                x: 65.0 + i as f64 * 75.0,
                y: 170.0,
                active: active.contains(&i),
            })
            .collect(),
        edges: vec![],
        rows: vec![],
        width: a.len() as f64 * 75.0 + 50.0,
        height: 270.0,
    };
    diagram(e, d, note);
}
pub(super) fn run(kind: usize, mut a: Vec<i64>, e: &mut Exercise) {
    record(
        e,
        &a,
        &[],
        "Initial input. Compare the values before moving them.".into(),
    );
    match kind {
        0 => {
            for end in (1..a.len()).rev() {
                let mut changed = false;
                for i in 0..end {
                    if a[i] > a[i + 1] {
                        a.swap(i, i + 1);
                        changed = true;
                        record(
                            e,
                            &a,
                            &[i, i + 1],
                            format!("Swap inverted neighbors at {i} and {}.", i + 1),
                        );
                    }
                }
                if !changed {
                    break;
                }
            }
        }
        1 => {
            for i in 1..a.len() {
                let mut j = i;
                while j > 0 && a[j - 1] > a[j] {
                    a.swap(j - 1, j);
                    j -= 1;
                    record(e,
&a,
&[j,
j+1],
format!("Move the next value left into the sorted prefix; prefix grows through index {i}."));
                }
            }
        }
        2 => {
            for i in 0..a.len() {
                let mut min = i;
                for j in i + 1..a.len() {
                    if a[j] < a[min] {
                        min = j;
                    }
                }
                a.swap(i, min);
                record(
                    e,
                    &a,
                    &[i, min],
                    format!("Minimum of the suffix is placed at index {i}."),
                );
            }
        }
        3 => merge(&mut a, e),
        4 => quick(&mut a, e),
        5 => {
            let n = a.len();
            for i in (0..n / 2).rev() {
                sift(&mut a, i, n, e);
            }
            for end in (1..n).rev() {
                a.swap(0, end);
                record(
                    e,
                    &a,
                    &[0, end],
                    format!("Place the largest remaining value at index {end}."),
                );
                sift(&mut a, 0, end, e);
            }
        }
        _ => (),
    }
    record(
        e,
        &a,
        &[],
        "Finished. Every adjacent pair is in ascending order.".into(),
    );
    e.answer = format!("Sorted values: {a:?}");
}
fn merge(a: &mut [i64], e: &mut Exercise) {
    // Operate on full snapshots through an internal range helper.
    fn range(a: &mut [i64], lo: usize, hi: usize, e: &mut Exercise) {
        if hi - lo < 2 {
            return;
        }
        let mid = (lo + hi) / 2;
        range(a, lo, mid, e);
        range(a, mid, hi, e);
        let left = a[lo..mid].to_vec();
        let right = a[mid..hi].to_vec();
        let (mut i, mut j) = (0, 0);
        for cell in &mut a[lo..hi] {
            if j == right.len() || i < left.len() && left[i] <= right[j] {
                *cell = left[i];
                i += 1;
            } else {
                *cell = right[j];
                j += 1;
            }
        }
        record(
            e,
            a,
            &(lo..hi).collect::<Vec<_>>(),
            format!(
                "Merge sorted ranges [{lo}, {mid}) and [{mid}, {hi}). Equal values \
                        prefer the left range."
            ),
        );
    }
    range(a, 0, a.len(), e);
}
fn quick(a: &mut [i64], e: &mut Exercise) {
    fn range(a: &mut [i64], lo: usize, hi: usize, e: &mut Exercise) {
        if hi - lo < 2 {
            return;
        }
        let pivot = a[hi - 1];
        let mut boundary = lo;
        for j in lo..hi - 1 {
            if a[j] <= pivot {
                a.swap(j, boundary);
                record(
                    e,
                    a,
                    &[j, boundary, hi - 1],
                    format!("Value at {j} belongs on the <= {pivot} side of the pivot."),
                );
                boundary += 1;
            }
        }
        a.swap(boundary, hi - 1);
        record(
            e,
            a,
            &[boundary],
            format!(
                "Pivot {pivot} reaches final index {boundary}. Recurse on both remaining regions."
            ),
        );
        range(a, lo, boundary, e);
        range(a, boundary + 1, hi, e);
    }
    range(a, 0, a.len(), e);
}
fn sift(a: &mut [i64], mut root: usize, end: usize, e: &mut Exercise) {
    loop {
        let left = root * 2 + 1;
        if left >= end {
            break;
        }
        let right = left + 1;
        let child = if right < end && a[right] > a[left] {
            right
        } else {
            left
        };
        if a[root] >= a[child] {
            break;
        }
        a.swap(root, child);
        record(
            e,
            a,
            &[root, child],
            format!("Restore max-heap order by moving the larger child up; heap size {end}."),
        );
        root = child;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_sort_matches_the_standard_sort_including_duplicates() {
        for kind in 0..6 {
            for case in 0..10 {
                let input = super::super::example(kind, case);
                let e = super::super::trace(kind, &input, case).unwrap();
                let mut expected: Vec<i64> = model::tokens(&input)
                    .iter()
                    .map(|s| s.parse().unwrap())
                    .collect();
                expected.sort();
                assert_eq!(e.answer, format!("Sorted values: {expected:?}"));
                assert!(e.frames.len() < 300);
            }
        }
    }
}
