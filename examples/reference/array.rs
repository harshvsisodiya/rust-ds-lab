fn main() {
    let mut values = vec![12,
 7,
 24];
    values.push(9);
                 // append at the end
    values.insert(1,
 5);
            // shift later values right
    values[2] = 18;
                 // direct indexed replacement
    values.remove(0);
               // shift later values left
    assert_eq!(values,
 [5,
 18,
 24,
 9]);
    println!("{values:?}");
}
