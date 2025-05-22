use struct_convert::Convert;

#[derive(Convert)]
#[convert(into = "B")]
struct A {
    value: i64,
}

struct B {
    value: i64,
}

fn main() {
    let a = A { value: 1 };
    let b: B = a.into();
    println!("{}", b.value);
}
