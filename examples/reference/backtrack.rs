fn subsets(input: &[i32],
 i: usize,
 path: &mut Vec<i32>,
 out: &mut Vec<Vec<i32>>) {
    if i == input.len() {
 out.push(path.clone());
 return;
 }
    subsets(input,
 i + 1,
 path,
 out);
 // exclude
    path.push(input[i]);
 subsets(input,
 i + 1,
 path,
 out);
 // include
    path.pop();
 // restore the prefix before returning
}
fn main() {
    let mut out = Vec::new();
 subsets(&[1,
 2,
 3],
 0,
 &mut Vec::new(),
 &mut out);
    assert_eq!(out.len(),
 8);
 println!("{out:?}");
}
