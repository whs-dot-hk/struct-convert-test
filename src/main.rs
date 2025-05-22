use struct_convert::Convert;

#[derive(Convert)]
#[convert(into = "B")]
struct A {
    value: C,
}

struct B {
    value: D,
}

enum C {
    A,
    B,
}

#[derive(Debug)]
enum D {
    A,
    B,
}

impl From<C> for D {
    fn from(c: C) -> Self {
        match c {
            C::A => D::A,
            C::B => D::B,
        }
    }
}

fn main() {
    let a = A { value: C::A };
    let b: B = a.into();
    println!("{:?}", b.value);
}
