use super::{Node, Link};
use super::rust::*;


pub struct Iter<'a, T:'a> {
    head : *const Link,
    tail : *const Link,
    len  : usize,
    mark : PhantomData<&'a Node<T>>,
}


impl<'a, T:'a> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    #[inline] fn next( &mut self ) -> Option<&'a Node<T>> {
        if self.head.is_null() {
            None
        } else { unsafe {
            let node = self.head;
            self.head = if self.head == self.tail {
                null()
            } else {
                (*node).next
            };
            self.len -= 1;
            Some( &*( node as *mut Node<T> ))
        }}
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) { ( self.len, Some( self.len ))}
}


impl<'a,T> ExactSizeIterator for Iter<'a, T> {}
impl<'a,T> FusedIterator for Iter<'a, T> {}

impl<'a, T:'a> Iter<'a, T> {
    #[inline] pub(crate) fn new( head: *const Link, tail: *const Link, len: usize ) -> Self {
        Iter{ head, tail, len, mark: PhantomData }
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

pub struct IterMut<'a, T:'a> {
    head : *mut Link,
    tail : *mut Link,
    len  : usize,
    mark : PhantomData<Pin<&'a mut Node<T>>>,
}

impl<'a, T:'a> Iterator for IterMut<'a, T> {
    type Item = Pin<&'a mut Node<T>>;

    #[inline] fn next( &mut self ) -> Option<Pin<&'a mut Node<T>>> {
        if self.head.is_null() {
            None
        } else { unsafe {
            let node = self.head;
            self.head = if self.head == self.tail {
                null_mut()
            } else {
                (*node).next
            };
            self.len -= 1;
            Some( Pin::new_unchecked( &mut *( node as *mut Node<T> )))
        }}
    }

    #[inline] fn size_hint( &self ) -> ( usize, Option<usize> ) { ( self.len, Some( self.len ))}
}

impl<'a,T> ExactSizeIterator for IterMut<'a, T> {}
impl<'a, T> FusedIterator for IterMut<'a, T> {}

impl<'a, T:'a> IterMut<'a, T> {
    #[inline] pub(crate) fn new( head: *mut Link, tail: *mut Link, len: usize ) -> Self {
        IterMut{ head, tail, len, mark: PhantomData }
    }
}

