// Copyright 2016 functils Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;
use std::collections::VecDeque;
//use std::io::BufWriter;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// The basic data structure used in functional programming. A list acts like an array.
/// You can't do indexing on it, but you can add too and grow it by using cons or you can
/// create a `List` with the list!() macro.
pub struct List<A> {
    data: VecDeque<A>,
}

#[macro_export]
/// Given a finite set of inputs produce a list.
///
/// list!(1, 2, 3) = [ 1 2 3 ]
///
/// list!("Hello", "Goodbye", "Goodnight") = [ "Hello" "Goodbye" "Goodnight" ]
macro_rules! list {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_list = List::new();
            $(
                temp_list.put_back($x);
            )*
            temp_list
        }
    };
}

impl<A> List<A> {
    /// Create an empty list
    pub fn new() -> List<A> {
        List::<A> {
            data: VecDeque::new(),
        }
    }

    /// Add a new item to the front of the list
    ///
    /// ##Example
    /// ```
    /// use functils::list::List;
    ///
    /// // []
    /// let mut list1 = List::new();
    ///
    /// // [ 1 ]
    /// list1.cons(1);
    ///
    /// // [ 2 1 ]
    /// list1.cons(2);
    ///
    /// println!("{}",list1);
    /// ```
    pub fn cons(&mut self, item: A) {
        self.data.push_front(item);
    }

    /// Helper function for the list!() macro so that
    /// items can get pushed in the order people expect
    /// when reading code.
    /// i.e. list(1,2,3) == [ 1 2 3 ] and
    /// not [ 3 2 1 ] if cons had been used
    fn put_back(&mut self, item: A) {
        self.data.push_back(item);
    }

    /// Given another `List`, combine them together.
    ///
    /// ##Example
    /// ```
    /// use functils::list::List;
    ///
    /// let mut list1 = List::new();
    /// let mut list2 = List::new();
    /// let mut list_cmp = List::new();
    ///
    /// // list1 = [ 1 ]
    /// list1.cons(1);
    ///
    /// // list2 = [ 3 ]
    /// list2.cons(3);
    ///
    /// // list_cmp = [ 1 3 ]
    /// list_cmp.cons(3);
    /// list_cmp.cons(1);
    ///
    /// // list1 = [ 1 3 ] and now list2 is gone
    /// list1.append(list2);
    ///
    /// assert_eq!(list1, list_cmp);
    /// ```
    pub fn append(&mut self, item: List<A>) {
        for i in item.data {
            self.data.push_back(i);
        }
    }

    /// If the List is empty return true, else false
    ///
    /// ##Example
    /// ```
    /// use functils::list::List;
    ///
    /// let mut list1 = List::new();
    ///
    /// assert!(list1.null());
    /// list1.cons(1);
    /// assert!(!list1.null());
    /// ```
    pub fn null(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the first element of a `List` if it exists
    /// this removes it from the List.
    /// ##Example
    /// ```
    /// use functils::list::List;
    ///
    /// let mut list1 = List::new();
    /// list1.cons(1);
    /// list1.cons(2);
    ///
    /// assert_eq!(list1.head(), Some(2));
    /// ```
    pub fn head(&mut self) -> Option<A> {
        self.data.pop_front()
    }

    /// Tail returns a modified version of a list and consumes the original.
    /// It returns a List with all but the first element in it.
    ///
    /// ##Example
    /// ```
    /// use functils::list::List;
    ///
    /// let mut list1 = List::new();
    /// let mut list_cmp = List::new();
    /// list1.cons(1);
    /// list1.cons(2);
    /// list1.cons(3);
    /// list_cmp.cons(1);
    /// list_cmp.cons(2);
    ///
    /// // [ 2 1 ] == [ 2 1]
    /// assert_eq!(list1.tail(), list_cmp);
    /// ```
    pub fn tail(mut self) -> List<A> {
        self.data.pop_front();
        self
    }

    /// Grabs the head of a list and the tail returned as a tuple. If the
    /// head doesn't exist return None.
    /// ##Example
    /// ```
    /// use functils::list::List;
    ///
    /// let mut list1 = List::new();
    /// list1.cons(3);
    /// list1.cons(2);
    /// list1.cons(1);
    ///
    /// let mut list_cmp = List::new();
    /// list_cmp.cons(3);
    /// list_cmp.cons(2);
    ///
    /// assert_eq!( list1.uncons(), Some((1,list_cmp)) );
    ///
    /// let empty: List<String> = List::new();
    ///
    /// assert_eq!( empty.uncons(), None );
    /// ```
    pub fn uncons(mut self) -> Option<(A,List<A>)> {
        if let Some(x) = self.data.pop_front() {
            Some((x, self))
        } else {
            None
        }
    }
}

impl<A: fmt::Display> fmt::Display for List<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let writer = BufWriter::new(self.data);
        try!(write!(f, "["));
        for i in self.data.iter() {
            try!(write!(f, " {}", i));
        }
        if self.data.is_empty() {
            try!(write!(f, "]"));
        } else {
            try!(write!(f, " ]"));
        }
        Ok(())
    }
}

#[test]
fn list_macro_works() {
    // [ 3 2 1 ]
    let list = list!(3, 2, 1);

    // [ 3 2 1 ]
    let mut list2 = List::new();
    list2.cons(1);
    list2.cons(2);
    list2.cons(3);

    // Should be the same
    assert_eq!(list,list2);
}

#[test]
fn null_works() {
    let list = list!("Hi", "bye");

    assert!(!list.null());
    assert!(List::<u64>::new().null());
}

#[test]
// Acts just like the list macro test really
fn cons_works() {
    // [ Hi bye Hello ]
    let list = list!("Hi", "bye", "Hello");

    let mut list2 = List::new();

    // [ Hello ]
    list2.cons("Hello");
    // [ bye Hello ]
    list2.cons("bye");
    // [ Hi bye Hello ]
    list2.cons("Hi");

    assert_eq!(list,list2);
}

#[test]
fn tail_works() {
    let list1 = list!(3, 2, 1);
    let list_cmp = list!(2, 1);

    // [ 2 1 ] == [ 2 1]
    assert_eq!(list1.tail(), list_cmp);
}

#[test]
fn head_works() {
    let mut list1 = list!(3, 2, 1);

    assert_eq!(list1.head(), Some(3));
    assert_eq!(list1.head(), Some(2));
    assert_eq!(list1.head(), Some(1));
    assert_eq!(list1.head(), None);
}

#[test]
fn append_works() {
    let mut list1 = list!(1, 2, 3);
    let list2 = list!(4, 5, 6);
    let list_cmp = list!(1, 2, 3, 4, 5, 6);

    list1.append(list2);

    assert_eq!(list1, list_cmp);
}

#[test]
fn uncons_works() {
    let list1 = list!(1, 2, 3);
    let list_cmp = list!(2,3);
    let empty: List<String> = List::new();

    assert_eq!( list1.uncons(), Some((1,list_cmp)) );
    assert_eq!( empty.uncons(), None );
}
