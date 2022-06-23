use toy_vec::ToyVec;


fn main() {
    let mut v = ToyVec::new();

    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());

    let mut iter = v.iter();

    // iterが不変の参照なので、pushは可変の参照を得られない
    // v.push("Hill Mynah".to_string());

    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
    v.push("Canary".to_string());
}
