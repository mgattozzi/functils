//! Rust has really powerful generics that can be used to make it more functional in terms of it's
//! composition. Included in this crate are helper functions often times used in languages like
//! Haskell.
#![allow(dead_code)]

// ReExporting traits for various stdlib items so that they can be used implicitly
pub use std::iter::FromIterator;

/// Given a Tuple return it's first element
pub fn fst<A,B>(x:(A,B)) -> A {
    x.0
}

/// Given a Tuple return it's second element
pub fn snd<A,B>(x:(A,B)) -> B {
    x.1
}

/// Given a value return itself. This is like the mathematical identity function
pub fn identity<A>(x:A) -> A {
    x
}

/// Works like Haskell's bool, this is a way to do an inline if then else statement
/// if b then x else y
pub fn ifte<A>(x: A, y: A, b:bool) -> A {
    if b { x } else { y }
}

/// Given a structure that can be indexed return it's first item if it exists and the rest of
/// the structure without that item. If the first item does not exist a None value will be returned
/// Since
pub fn uncons<A, B>(x: A) -> Option<(B,A)>
    where A: IntoIterator<Item = B> + FromIterator<B> {
    let mut iter = x.into_iter();
    if let Some(one) = iter.next() {
        Some((one, iter.collect()))
    } else {
        None
    }
}

#[test]
fn fst_works() {
    assert_eq!(fst((1,"Hello")),1);
}

#[test]
fn snd_works() {
    assert_eq!(snd((1,"Hello")),"Hello");
}

#[test]
fn identity_works() {
    assert_eq!(identity(1), 1);
    assert_eq!(identity(1.0), 1.0);
    assert_eq!(identity("Hi"), "Hi");
}

#[test]
fn ifte_works() {
    assert_eq!(ifte("Hi","Bye", true), "Hi");
    assert_eq!(ifte("Hi","Bye", false), "Bye");
}

#[test]
fn uncons_works() {
    let vec = vec![1, 2, 3];
    let vec_comp = vec![2, 3];
    if let Some(x) = uncons(vec) {
        x.eq(&(1,vec_comp));
    } else {
        assert_eq!(1,2);
    }
}
