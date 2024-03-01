use struct_morph::morph;

#[derive(Clone)]
struct Foo {
    a: u16,
    b: String,
    c: Vec<u32>,
    d: (String, String),
}

#[morph(Foo)]
struct Bar {
    a: u16,
    b: String,
    c: Vec<u32>,
}

#[test]
fn simple() {
    let my_foo: Foo = Foo {
        a: 10,
        b: "Hello".to_string(),
        c: vec![1, 3, 4],
        d: ("Good".to_string(), "Bye".to_string()),
    };

    let auto_bar: Bar = Bar::from(my_foo.clone());

    assert_eq!(my_foo.a, auto_bar.a);
    assert_eq!(my_foo.b, auto_bar.b);
    assert_eq!(my_foo.c, auto_bar.c);
}

#[morph(Foo)]
struct Baz {
    b: String,
    c: Vec<u32>,
    #[morph_field(transform = "foo_d_first")]
    e: String,
    #[morph_field(transform = "foo_d_sec_len")]
    f: usize,
}

fn foo_d_first(value: &Foo) -> String {
    value.d.0.clone()
}

fn foo_d_sec_len(value: &Foo) -> usize {
    value.d.1.len()
}

#[test]
fn with_field_transform() {
    let my_foo: Foo = Foo {
        a: 10,
        b: "Hello".to_string(),
        c: vec![1, 3, 4],
        d: ("Good".to_string(), "Bye".to_string()),
    };

    let auto_baz: Baz = Baz::from(my_foo.clone());

    assert_eq!(my_foo.b, auto_baz.b);
    assert_eq!(my_foo.c, auto_baz.c);
    assert_eq!(foo_d_first(&my_foo), auto_baz.e);
    assert_eq!(foo_d_sec_len(&my_foo), auto_baz.f);
}
