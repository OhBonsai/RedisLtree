use super::rust::*;
use super::bfs::{BfsTree, Splitted, Split};
use super::{Tree,Forest,Iter,IterMut,OntoIter,Size};


pub struct Link {
    pub(crate) next   : *mut Link, // next sibling
    pub(crate) child  : *mut Link, // last child
    pub(crate) prev   : *mut Link, // previous sibling
    pub(crate) parent : *mut Link,
    pub(crate) size   : Size,
}

#[repr(C)]
pub struct Node<T> {
    pub(crate) link : Link,
    pub        data : T,
}

impl<T> Deref for Node<T> {
    type Target = Link;
    fn deref( &self ) -> &Link { &self.link }
}


impl Link {
    #[inline] pub(crate) fn set_parent( &mut self, parent: *mut Self ) { self.parent = parent; }
    #[inline] pub(crate) fn reset_parent( &mut self ) { self.parent = null_mut(); }

    #[inline] pub(crate) fn set_child( &mut self, child: *mut Self ) { self.child = child; }
    #[inline] pub(crate) fn reset_child( &mut self ) { self.set_child( null_mut() ); }
    #[inline] pub(crate) fn is_leaf( &self ) -> bool { self.child.is_null() }
    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn set_sib( &mut self, prev: *mut Self, next: *mut Self ) { self.prev = prev; self.next = next; }
    #[inline] pub(crate) fn reset_sib( &mut self ) { self.prev  = self as *mut Self; self.next = self as *mut Self; }
    #[inline] pub(crate) fn has_no_sib( &self ) -> bool { self.prev as *const Self == self as *const Self && self.next as *const Self == self as *const Self }

    #[inline] pub(crate) unsafe fn head( &self ) -> *mut Self { (*self.child).next }

    #[inline] pub(crate) fn tail( &self ) -> *mut Self { self.child }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Self { (*self.head()).next }
    #[inline] pub(crate) unsafe fn new_tail( &self ) -> *mut Self { (*self.tail()).prev  }

    #[inline] pub(crate) unsafe fn adopt( &mut self, begin: *mut Self, end: *mut Self ) { (*self.head()).prev  = begin; (*self.tail()).next = end; }

    #[inline] pub(crate) fn inc_sizes( &mut self, degree: u32, node_cnt: u32 ) {
        self.size.degree += degree;
        let mut link = self as *mut Self;
        while !link.is_null() {
            unsafe {
                (*link).size.node_cnt += node_cnt;
                link = (*link).parent;
            }
        }
    }

    #[inline] pub(crate) fn dec_sizes( &mut self, degree: u32, node_cnt: u32 ) {
        self.size.degree -= degree;
        let mut link = self as *mut Self;
        while !link.is_null() {
            unsafe {
                (*link).size.node_cnt -= node_cnt;
                link = (*link).parent;
            }
        }
    }
}


impl<T> Node<T> {
    #[inline]
    pub(crate) fn is_leaf(&self) -> bool {self.link.is_leaf()}

    #[inline]
    pub(crate) fn plink(&mut self) -> *mut Link { &mut self.link as *mut Link}

    #[inline]
    pub fn degree(&self) -> usize {self.size.degree as usize}

    #[inline]
    pub fn node_count(&self) -> usize {self.size.node_cnt as usize}


    #[inline]
    pub fn forest(&self) -> &Forest<T> {
        unsafe { &*( &self.link as *const Link as *const Forest<T>) }
    }

    // 这里Pin住是为了防止指针所指向的T被固定，因为涉及到*Link -> *Forest的转换
    // 这里为什么不要Option？ 实际上就是返回self，只不过类型转换为Forest<T>, Forest一定存在
    #[inline] pub fn forest_mut(&mut self) -> Pin<&mut Forest<T>> {
        unsafe{ Pin::new_unchecked( &mut *(self.plink() as *mut Forest<T>) )}
    }


    #[inline]
    pub fn first(&self) -> Option<&Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some(&*(self.head() as *const Node<T>))}
        }
    }

    pub fn first_mut( &mut self ) -> Option<Pin<&mut Node<T>>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( Pin::new_unchecked( &mut *( self.head() as *mut Node<T> )))}
        }
    }

    pub fn last( &self ) -> Option<&Node<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( &*( self.tail() as *const Node<T> ))}
        }
    }

    pub fn last_mut( &mut self ) -> Option<Pin<&mut Node<T>>> {
        if self.is_leaf() {
            None
        } else {
            unsafe { Some( Pin::new_unchecked( &mut *( self.tail() as *mut Node<T> )))}
        }
    }

    pub fn parent( &self ) -> Option<&Node<T>> {
        if self.parent.is_null() {
            None
        } else { unsafe {
            Some( &*( self.parent as *mut Node<T> ))
        }}
    }

    pub fn push_front( &mut self, mut tree: Tree<T> ) {
        unsafe {
            // 把自己设置为这个tree的爹
            tree.link_mut().set_parent(self.plink());

            let tree_root = tree.root_mut_().plink();
            if self.is_leaf() {
                self.link.set_child(tree_root)
            } else {
                tree.link_mut().set_sib(self.tail(), self.head());
                self.link.adopt(tree_root, tree_root)
            }
        }
        self.link.inc_sizes(1, tree.root().size.node_cnt);
        tree.clear()
    }

    pub fn pop_front(&mut self) -> Option<Tree<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe {
                let front = self.head();
                if self.has_only_one_child() {
                    self.link.reset_child()
                } else {
                    (*self.new_head()).prev = self.tail();
                    (*self.tail()).next = self.new_head();
                }

                (*front).reset_parent();
                (*front).reset_sib();
                self.link.dec_sizes( 1, (*front).size.node_cnt );
                Some( Tree::from( front ))
            }
        }
    }

    pub fn push_back(&mut self, mut tree: Tree<T>) {
        unsafe {
            tree.link_mut().set_parent(self.plink());
            let tree_root = tree.root_mut_().plink();
            if !self.is_leaf() {
                tree.link_mut().set_sib(self.tail(), self.head());
                self.link.adopt(tree_root, tree_root);
            }
            self.link.set_child(tree_root);
        }

        self.link.inc_sizes(1, tree.root().size.node_cnt);
        tree.clear();
    }

    pub fn pop_back(& mut self) -> Option<Tree<T>> {
        if self.is_leaf() {
            None
        } else {
            unsafe {
                let back = self.tail();
                if self.has_only_one_child() {
                    self.link.reset_child();
                } else {
                    let new_tail = self.new_tail();
                    (*new_tail).next = self.head();
                    (*self.head()).prev = new_tail;
                    self.link.set_child(new_tail);
                }

                (*back).reset_parent();
                (*back).reset_sib();
                self.link.dec_sizes(1, (*back).size.node_cnt);
                Some(Tree::from(back))
            }
        }
    }

    pub fn prepend(&mut self, mut forest: Forest<T>) {
        if !forest.is_empty() {
            forest.set_parent(self.plink());
            if self.is_leaf() {
                self.link.set_child(forest.tail());
            } else {
                unsafe {
                    let forest_head = forest.head();
                    forest.set_sib(self.tail(), self.head());
                    self.link.adopt(forest.tail(), forest_head);
                }
            }
            self.link.inc_sizes(forest.size.degree, forest.size.node_cnt);
            forest.clear();
        }
    }

    pub fn append(&mut self, mut forest: Forest<T>) {
        if !forest.is_empty() {
            forest.set_parent(self.plink());
            if self.is_leaf() {
               self.link.set_child(forest.tail());
            } else { unsafe{
                let forest_head = forest.head();
                forest.set_sib(self.tail(), self.head());
                self.link.adopt(self.tail(), forest_head);
                self.link.set_child(forest.tail());
            }}

            self.link.inc_sizes(forest.size.degree, forest.size.node_cnt);
            forest.clear();
        }
    }

    pub fn iter<'a, 's: 'a>(&'s self) -> Iter<'a, T> {
        if self.is_leaf() {
            Iter::new(null_mut(), null_mut(), 0)
        } else {
            unsafe {
                Iter::new(self.head(), self.tail(), self.size.degree as usize)
            }

        }
    }

    pub fn iter_mut<'a, 's:'a>(&'s mut self) -> IterMut<'a, T> {
        if self.is_leaf() {
            IterMut::new(null_mut(), null_mut(), 0)
        } else {
            unsafe {
                IterMut::new(self.head(), self.tail(), self.size.degree as usize)
            }
        }
    }


    pub fn onto_iter<'a, 's:'a>(&'s mut self) -> OntoIter<'a, T> {
        unsafe {
            if self.is_leaf() {
                OntoIter {
                    next: null_mut(),
                    curr: null_mut(),
                    prev: null_mut(),
                    child: null_mut(),
                    parent: self.plink(),
                    mark: PhantomData,
                }
            } else {
                OntoIter {
                    next   : self.head(),
                    curr   : null_mut(),
                    prev   : self.child,
                    child  : self.child,
                    parent : self.plink(),
                    mark   : PhantomData,
                }
            }
        }
    }

    pub fn bfs( &self) -> BfsTree<Splitted<Iter<T>>> {
        BfsTree::from(self, Size{
            degree:1,
            node_cnt: self.link.size.node_cnt
        })
    }

    pub fn bfs_mut(&mut self) -> BfsTree<Splitted<IterMut<T>>> {
        let size = Size {
            degree: 1,
            node_cnt: self.link.size.node_cnt
        };
        BfsTree::from(unsafe{
            Pin::new_unchecked(self)
        }, size)

    }
}


impl<'a, T:'a> Split for &'a Node<T> {
    type Item = &'a T;
    type Iter = Iter<'a, T>;

    fn split(self) -> (&'a T, Iter<'a, T>, u32) {
        (&self.data, self.iter(), self.link.size.node_cnt)
    }
}

impl<'a, T:'a> Split for Pin<&'a mut Node<T>> {
    type Item = &'a mut T;
    type Iter = IterMut<'a,T>;

    fn split( self ) -> ( &'a mut T, IterMut<'a,T>, u32 ) {
        let node_cnt = self.link.size.node_cnt;
        unsafe {
            let node_mut = self.get_unchecked_mut();
            let data = &mut *( &mut node_mut.data as *mut T );
            let iter = node_mut.iter_mut();
            ( data, iter, node_cnt )
        } // borrow two mutable references at one time
    }
}

impl<'a, T:'a> IntoIterator for &'a Node<T> {
    type Item = Self;
    type IntoIter = Iter<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        let link = self as *const Node<T> as *const Link;
        Iter::new( link, link, 1 )
    }
}

impl<'a, T:'a> IntoIterator for Pin<&'a mut Node<T>> {
    type Item = Self;
    type IntoIter = IterMut<'a,T>;

    #[inline] fn into_iter( self ) -> Self::IntoIter {
        let link = unsafe{ self.get_unchecked_mut().plink() };
        IterMut::new( link, link, 1 )
    }
}


impl<T:Clone> ToOwned for Node<T> {
    type Owned = Tree<T>;
    fn to_owned( &self ) -> Self::Owned {
        let mut tree = Tree::new( self.data.clone() );
        for child in self.iter() {
            tree.root_mut_().push_back( child.to_owned() );
        }
        tree
    }
}

impl<T> Borrow<Forest<T>> for Tree<T> { fn borrow( &self ) -> &Forest<T> { self.forest() }}

impl<T> Extend<Tree<T>> for Node<T> {
    fn extend<I:IntoIterator<Item=Tree<T>>>( &mut self, iter: I ) {
        for child in iter.into_iter() {
            self.push_back( child );
        }
    }
}

impl Debug for Link {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        write!( f, "{{ @{:?} ←{:?} ↑{:?} ↓{:?} →{:?} ({},{}) }}",
                self as *const _,
                self.prev, self.parent, self.child, self.next,
                self.size.degree, self.size.node_cnt
        )
    }
}

impl<T:Debug> Debug for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            self.data.fmt(f)?;
            self.link.fmt(f)
        } else {
            self.data.fmt(f)?;
            self.link.fmt(f)?;
            write!( f, "( " )?;
            for child in self.iter() {
                child.fmt(f)?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:Display> Display for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            self.data.fmt(f)
        } else {
            self.data.fmt(f)?;
            write!( f, "( " )?;
            for child in self.iter() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:PartialEq> PartialEq for Node<T> {
    fn eq( &self, other: &Self ) -> bool { self.data == other.data && self.iter().eq( other.iter() )}
    fn ne( &self, other: &Self ) -> bool { self.data != other.data || self.iter().ne( other.iter() )}
}

impl<T:Eq> Eq for Node<T> {}

impl<T:PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        match self.data.partial_cmp( &other.data ) {
            None          => None,
            Some( order ) => match order {
                Less    => Some( Less ),
                Greater => Some( Greater ),
                Equal   => self.iter().partial_cmp( other.iter() ),
            },
        }
    }
}

impl<T:Ord> Ord for Node<T> {
    #[inline] fn cmp( &self, other: &Self ) -> Ordering {
        match self.data.cmp( &other.data ) {
            Less    => Less,
            Greater => Greater,
            Equal   => self.iter().cmp( other.iter() ),
        }
    }
}

impl<T:Hash> Hash for Node<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        self.data.hash( state );
        for child in self.iter() {
            child.hash( state );
        }
    }
}


