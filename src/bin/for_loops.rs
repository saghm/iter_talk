#[derive(Debug)]
struct Number(u32);

#[derive(Debug)]
enum Boo<'a> {
    Borrowed(&'a Number),
    Owned(Number),
}

impl<'a> From<&'a Number> for Boo<'a> {
    fn from(num: &'a Number) -> Self {
        Boo::Borrowed(num)
    }
}

impl<'a> From<Number> for Boo<'a> {
    fn from(num: Number) -> Self {
        Boo::Owned(num)
    }
}

fn main() {
    let vec = vec![Number(1), Number(2), Number(3)];

    println!("Explicitly calling `iter`:");
    for num in vec.iter() {
        println!("    {:?}", Boo::from(num));
    }

    let vec = vec![Number(1), Number(2), Number(3)];

    println!("\nCoercing to slice:");
    for num in &vec {
        println!("    {:?}", Boo::from(num));
    }

    let vec = vec![Number(1), Number(2), Number(3)];

    println!("\nExplicitly calling `into_iter`:");
    for num in vec.into_iter() {
        println!("    {:?}", Boo::from(num));
    }

    let vec = vec![Number(1), Number(2), Number(3)];

    println!("\nImplicitly coercing to iterator:");
    for num in vec {
        println!("    {:?}", Boo::from(num));
    }
}
