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

fn main() {
    let mut counter = Counter::new(3);
    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
    println!("count: {:?}", counter.next());
}
