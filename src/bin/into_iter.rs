#[derive(Debug)]
struct ThreeThings {
    first: u32,
    second: u32,
    third: u32,
}

struct ThreeThingsIter {
    things: ThreeThings,
    index: u8,
}

impl IntoIterator for ThreeThings {
    type Item = u32;
    type IntoIter = ThreeThingsIter;

    fn into_iter(self) -> Self::IntoIter {
        ThreeThingsIter {
            things: self,
            index: 0,
        }
    }
}

impl Iterator for ThreeThingsIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let thing = match self.index {
            0 => self.things.first,
            1 => self.things.second,
            2 => self.things.third,
            _ => return None,
        };

        self.index += 1;
        Some(thing)
    }
}

fn main() {
    let things = ThreeThings {
        first: 11,
        second: 22,
        third: 33,
    };

    let mut iter = things.into_iter();
    println!("thing: {:?}", iter.next());
    println!("thing: {:?}", iter.next());
    println!("thing: {:?}", iter.next());
    println!("thing: {:?}", iter.next());
}
