//! Rust has really powerful generics that can be used to make it more functional in terms of it's
//! composition. Included in this crate are helper functions often times used in languages like
//! Haskell.

/// Given a Tuple return it's first element
pub fn fst<T,A>(x:(T,A)) -> T {
    x.0
}

/// Given a Tuple return it's second element
pub fn snd<T,A>(x:(T,A)) -> A {
    x.1
}

fn foldl<T: Fn(A) -> B , A: Iterator, B>(func: T,iter: A) -> B {
    func(iter)
}

#[test]
fn fst_works() {
    assert_eq!(fst((1,"Hello")),1);
}

#[test]
fn snd_works() {
    assert_eq!(snd((1,"Hello")),"Hello");
}
