use std::collections::HashSet;

#[derive(Debug)]
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.max {
            return None;
        }

        self.count += 1;
        Some(self.count)
    }
}

fn print_set(set: &HashSet<u32>) {
    println!("{:?}", set);
}

fn main() {
    let vec = vec![1, 2, 3];
    println!("vec: {:?}", vec);

    let set: HashSet<_> = vec.into_iter().collect();
    println!("set: {:?}", set);

    println!("\nSet from counter:");
    print_set(&Counter::new(10).into_iter().collect());
}
