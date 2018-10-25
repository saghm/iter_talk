fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    println!("1 + 2 + 3 + 4 + 5 = {}", vec.iter().sum::<u32>());

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!(
        "2 * 4 * 6 = {}",
        vec1.into_iter()
            .chain(vec2)
            .filter(|i| i % 2 == 0)
            .fold(1, |i, j| i * j)
    );
}
