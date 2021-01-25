use super::{Node, Link, Forest};
use super::{heap, Size};
use super::bfs::{BfsTree, Splitted, Split};
use super::forest::IntoIter;
use super::rust::*;


pub struct Tree<T> {
    pub(crate) root : *mut Node<T>,
    mark : heap::Phantom<T>,
}


impl<T> Tree<T> {
    /// Creates a `Tree` with given data on heap.
    #[inline] pub fn new( data: T ) -> Self { Self::from( heap::make_node( data ) as *mut Link )}

    #[inline] pub fn root( &self ) -> &Node<T> { unsafe { & *self.root }}
    #[inline] pub fn root_mut( &mut self ) -> Pin<&mut Node<T>> { unsafe{ Pin::new_unchecked( self.root_mut_() )}}
    #[inline] pub(crate)  fn root_mut_( &mut self ) -> &mut Node<T> { unsafe { &mut *self.root }}
    #[inline] pub(crate) fn link_mut( &mut self ) -> &mut Link { unsafe{ &mut (*self.root).link }}

    #[inline] fn into_data( self ) -> T {
        let data = unsafe{ ptr::read( &self.root().data )};
        self.clear();
        data
    }

    /// Removes and returns the given `Tree`'s children.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::linked::fully::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.abandon().to_string(), "( 1 2 )" );
    /// assert_eq!( tree, tr(0) );
    /// ```
    #[inline] pub fn abandon( &mut self ) -> Forest<T> {
        let forest = Forest::<T>::from( self.root().tail(), self.root().size );
        self.link_mut().reset_child();
        self.link_mut().size.degree = 0;
        self.link_mut().size.node_cnt = 1;
        forest
    }

    /// Provides a forward iterator with owned data in a breadth-first manner
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::{bfs,Size};
    /// use trees::linked::fully::tr;
    ///
    /// let tree = tr(0) /( tr(1)/tr(2)/tr(3) ) /( tr(4)/tr(5)/tr(6) );
    /// let visits = tree.into_bfs().iter.collect::<Vec<_>>();
    /// assert_eq!( visits, vec![
    ///     bfs::Visit{ data: 0, size: Size{ degree: 2, node_cnt: 7 }},
    ///     bfs::Visit{ data: 1, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: 4, size: Size{ degree: 2, node_cnt: 3 }},
    ///     bfs::Visit{ data: 2, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 3, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 5, size: Size{ degree: 0, node_cnt: 1 }},
    ///     bfs::Visit{ data: 6, size: Size{ degree: 0, node_cnt: 1 }},
    /// ]);
    /// ```
    pub fn into_bfs( self ) -> BfsTree<Splitted<IntoIter<T>>> {
        let size = Size{ degree: 1, node_cnt: self.root().link.size.node_cnt };
        BfsTree::from( self, size )
    }

    #[inline] pub(crate) fn from( root: *mut Link ) -> Self { Tree{ root: root as *mut Node<T>, mark: PhantomData }}
    #[inline] pub(crate) fn clear( mut self ) { self.root = null_mut(); }
}

impl<T> IntoIterator for Tree<T> {
    type Item = Tree<T>;
    type IntoIter = IntoIter<T>;

    #[inline] fn into_iter( self ) -> IntoIter<T> {
        let mut forest = Forest::<T>::new();
        forest.push_back( self );
        IntoIter{ forest, marker: PhantomData }
    }
}

impl<T> Borrow<Node<T>> for Tree<T> { fn borrow( &self ) -> &Node<T> { self.root() }}

impl<T> Deref for Tree<T> {
    type Target = Node<T>;
    fn deref( &self ) -> &Node<T> { unsafe { &*self.root }}
}

impl<T:Clone> Clone for Tree<T> { fn clone( &self ) -> Self { self.root().to_owned() }}

impl<T> Drop for Tree<T> {
    fn drop( &mut self ) {
        if !self.root.is_null() {
            while let Some(_) = self.root_mut_().pop_front() {}
            heap::drop_node( self.root );
        }
    }
}

impl<T:Debug> Debug for Tree<T> { fn fmt( &self, f: &mut Formatter ) -> fmt::Result { self.root().fmt(f) }}

impl<T:Display> Display for Tree<T> { fn fmt( &self, f: &mut Formatter ) -> fmt::Result { write!( f, "{}", self.root() )}}

impl<T:PartialEq> PartialEq for Tree<T> {
    fn eq( &self, other: &Self ) -> bool { self.root().eq( other.root() )}
    fn ne( &self, other: &Self ) -> bool { self.root().ne( other.root() )}
}

impl<T:Eq> Eq for Tree<T> {}

impl<T:PartialOrd> PartialOrd for Tree<T> { #[inline] fn partial_cmp( &self, other: &Self ) -> Option<Ordering> { self.root().partial_cmp( other.root() )}}

impl<T:Ord> Ord for Tree<T> { #[inline] fn cmp( &self, other: &Self ) -> Ordering { self.root().cmp( other.root() )}}

impl<T:Hash> Hash for Tree<T> { fn hash<H:Hasher>( &self, state: &mut H ) { self.root().hash( state )}}

unsafe impl<T:Send> Send for Tree<T> {}
unsafe impl<T:Sync> Sync for Tree<T> {}
