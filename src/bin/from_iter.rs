use std::iter::FromIterator;

#[derive(Debug, Default)]
struct Split {
    half1: Vec<u32>,
    half2: Vec<u32>,
}

impl FromIterator<u32> for Split {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u32>,
    {
        let mut split = Split::default();

        for item in iter {
            if split.half1.len() > split.half2.len() {
                split.half2.push(item);
            } else {
                split.half1.push(item);
            }
        }

        split
    }
}

fn main() {
    let split: Split = (0..10).into_iter().collect();
    println!("{:#?}", split);
}
