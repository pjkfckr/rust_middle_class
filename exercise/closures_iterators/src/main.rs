#[allow(unused_mut)]
fn main() {
    let square = |x| x * x;
    println!("5 squared is {:?}", square(5));

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    pairs
        .into_iter()
        .map(|(x, y)| (x + 1, y))
        .for_each(|t| println!("{:?}", t));

    let mut numbers = vec![1, 2, 3, 4];

    for x in numbers.iter_mut() {
        *x = *x * 3;
    }
    println!("{:?}", numbers);

    let words = vec![
        "autobot",
        "beach",
        "car",
        "description",
        "energon",
        "frothy",
    ];
    let transformed = words
        .into_iter()
        .filter(|word| !word.contains("h"))
        .map(|x| x.to_uppercase())
        .collect::<Vec<_>>();
    println!("Transformed: {:?}", transformed);
}
