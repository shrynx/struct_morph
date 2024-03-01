extern crate struct_morph;

use struct_morph::morph;

#[derive(Clone)]
struct Foo {
    a: u16,
    b: &'static str,
    c: Vec<u32>,
    d: (&'static str, &'static str),
}

#[morph(Foo)]
struct Bar {
    a: u16,
    b: &'static str,
    c: Vec<u32>,
}

#[test]
fn it_works() {
    let my_foo: Foo = Foo {
        a: 10,
        b: "Hello",
        c: vec![1, 3, 4],
        d: ("Good", "Bye"),
    };

    let auto_bar: Bar = Bar::from(my_foo.clone());

    assert_eq!(my_foo.a, auto_bar.a);
    assert_eq!(my_foo.b, auto_bar.b);
    assert_eq!(my_foo.c, auto_bar.c);

}
