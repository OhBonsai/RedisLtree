use super::{Tree, Forest, Node};
use super::rust::*;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Visit<'a, T:'a> {
    Begin(&'a Node<T>),
    End(&'a Node<T>),
    Leaf(&'a Node<T>),
}


impl<'a, T:'a> Visit<'a, T> {
    pub fn node(&self) -> &Node<T> {
        match *self {
            Visit::Begin( node ) => node,
            Visit::End  ( node ) => node,
            Visit::Leaf ( node ) => node,
        }
    }
}

enum VisitType {None, Begin, End, Leaf}

struct Nodes<T> {
    node: *const Node<T>,
    sentinel: *const Node<T>,
}


impl<T> Nodes<T> {
    fn this(node: *const Node<T>) -> Self {
        Nodes{
            node,
            sentinel: unsafe{(*node).next as *const Node<T>}
        }
    }

    fn sibs(node: *const Node<T>) -> Self {
        Nodes{
            node,
            sentinel: node
        }
    }
}



enum Direction {
    Up, Down, Right
}


struct Walk<T> {
    path: Vec<Nodes<T>>,
    direction: Direction,
    visit_type: VisitType,
    origin: *const Node<T>,
}


impl<T> Walk<T> {

    #[inline] fn reset( &mut self ) {
        self.path.clear();
        self.direction = Direction::Down;
        self.visit_type = VisitType::None;
    }

    #[inline] fn init_visit( &mut self ) {
        self.visit_type =
            if let Some( nodes ) = self.path.last() {
                unsafe {
                    if (*nodes.node).is_leaf() {
                        VisitType::Leaf
                    } else {
                        VisitType::Begin
                    }
                }
            } else {
                VisitType::None
            };
    }

    #[inline] fn on_node( &mut self, node: *const Node<T> ) {
        self.reset();
        self.path.push( Nodes::this( node ));
        self.init_visit();
        self.origin = node;
    }

    #[inline] fn on_forest( &mut self, head: *const Node<T> ) {
        self.reset();
        self.path.push( Nodes::sibs( head ));
        self.init_visit();
        self.origin = head;
    }

    #[inline] fn revisit(&mut self) {
        if !self.origin.is_null() {
            match self.visit_type {
                VisitType::None => self.path.push(Nodes::sibs(self.origin)),
                _ => (),
            }
            self.direction = Direction::Down;
            self.init_visit();
        }
    }

    #[inline] fn get( &self ) -> Option<Visit<T>> {
        if let Some( nodes ) = self.path.last() {
            unsafe { match self.visit_type {
                VisitType::Begin => Some( Visit::Begin( &*nodes.node )),
                VisitType::End   => Some( Visit::End  ( &*nodes.node )),
                VisitType::Leaf  => Some( Visit::Leaf ( &*nodes.node )),
                VisitType::None  => None,
            }}
        } else {
            None
        }
    }

    #[inline] fn forward( &mut self) {
        loop {
            match self.direction {
                Direction::Up => {
                    self.path.pop();
                    if self.path.last().is_some() {
                        self.direction = Direction::Right;
                        self.visit_type = VisitType::End;
                    } else {
                        self.direction = Direction::Down;
                        self.visit_type = VisitType::None;
                    }
                    break;
                },

                Direction::Down => {
                    let new_nodes;
                    if let Some(nodes) = self.path.last_mut() {
                        let node  = unsafe {&*nodes.node};
                        if node.is_leaf() {
                            self.direction = Direction::Right;
                            continue;
                        } else {
                            let head = unsafe{node.head()};
                            new_nodes = Some(Nodes::sibs(head as *const Node<T>));
                            self.visit_type = if unsafe { (*head).is_leaf() } {VisitType::Leaf} else {VisitType::Begin};
                        }
                    } else {
                        break;
                    }
                    new_nodes.map(|nodes| self.path.push(nodes));
                    break;
                },

                Direction::Right => {
                    if let Some( nodes ) = self.path.last_mut() {
                        nodes.node = unsafe{ (*nodes.node).next as *const Node<T> };
                        if nodes.node == nodes.sentinel {
                            self.direction = Direction::Up;
                            continue;
                        } else {
                            if unsafe{ (*nodes.node).is_leaf() } {
                                self.visit_type = VisitType::Leaf;
                            } else {
                                self.visit_type = VisitType::Begin;
                                self.direction = Direction::Down;
                            }
                            break;
                        }
                    }
                }
            }
        }

    }

    #[inline] fn next( &mut self ) -> Option<Visit<T>> {
        self.forward();
        self.get()
    }

    #[inline] fn to_parent( &mut self ) -> Option<Visit<T>> {
        if self.path.last().is_some() {
            self.path.pop();
            if self.path.last().is_some() {
                self.direction = Direction::Right;
                self.visit_type = VisitType::End;
                return self.get();
            }
        }
        self.direction = Direction::Down;
        self.visit_type = VisitType::None;
        None
    }

    #[inline] fn get_parent( &self ) -> Option<&Node<T>> {
        if self.path.len() >= 2 {
            self.path.get( self.path.len()-2 ).map( |parent| unsafe{ &*parent.node })
        } else {
            None
        }
    }

    #[inline] fn to_sib( &mut self, n: usize ) -> Option<Visit<T>> {
        if let Some( nodes ) = self.path.last_mut() {
            for _ in 0..n {
                nodes.node = unsafe{ (*nodes.node).next as *const Node<T> };
                if nodes.node == nodes.sentinel {
                    self.direction = Direction::Up;
                    return None;
                }
            }
            if unsafe{ (*nodes.node).is_leaf() } {
                self.visit_type = VisitType::Leaf;
            } else {
                self.visit_type = VisitType::Begin;
                self.direction = Direction::Down;
            }
        } else {
            return None;
        }
        return self.get();
    }

    #[inline] fn to_child( &mut self, n: usize ) -> Option<Visit<T>> {
        let new_nodes;
        if let Some( nodes ) = self.path.last_mut() {
            let node = unsafe{ &*nodes.node };
            if node.is_leaf() {
                self.direction = Direction::Right;
                return None;
            } else {
                let head = unsafe{ node.head() };
                new_nodes = Some( Nodes::sibs( head as *const Node<T> ));
                self.visit_type = if unsafe{ (*head).is_leaf() } { VisitType::Leaf } else { VisitType::Begin };
            }
        } else {
            return None;
        }
        new_nodes.map( |nodes| self.path.push( nodes ));
        self.to_sib( n )
    }
}

impl<T> Default for Walk<T> {
    #[inline] fn default() -> Self {
        Walk{ path: Vec::default(), direction: Direction::Down, visit_type: VisitType::None, origin: null() }
    }
}

pub struct TreeWalk<T> {
    tree : Tree<T>,
    walk : Walk<T>,
}


impl<T> TreeWalk<T> {
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn get( &self ) -> Option<Visit<T>> { self.walk.get() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn forward( &mut self ) { self.walk.forward(); }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn next( &mut self ) -> Option<Visit<T>> { self.walk.next() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn to_parent( &mut self ) -> Option<Visit<T>> { self.walk.to_parent() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn get_parent( &self ) -> Option<&Node<T>> { self.walk.get_parent() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn to_child( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_child(n) }

    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn to_sib( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_sib(n) }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn revisit( &mut self ) { self.walk.revisit(); }
}

impl<T> From<Tree<T>> for TreeWalk<T> {
    fn from( tree: Tree<T> ) -> Self {
        let mut walk = Walk::<T>::default();
        walk.on_node( tree.root );
        TreeWalk{ tree, walk }
    }
}

impl<T> Into<Tree<T>> for TreeWalk<T> { fn into( self ) -> Tree<T> { self.tree }}


#[derive( Default )]
pub struct ForestWalk<T> {
    forest : Forest<T>,
    walk   : Walk<T>,
}


impl<T> ForestWalk<T> {
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn get( &self ) -> Option<Visit<T>> { self.walk.get() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn forward( &mut self ) { self.walk.forward(); }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn next( &mut self ) -> Option<Visit<T>> { self.walk.next() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn to_parent( &mut self ) -> Option<Visit<T>> { self.walk.to_parent() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn get_parent( &self ) -> Option<&Node<T>> { self.walk.get_parent() }

    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn to_child( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_child(n) }

    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn to_sib( &mut self, n: usize ) -> Option<Visit<T>> { self.walk.to_sib(n) }

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[inline] pub fn revisit( &mut self ) { self.walk.revisit(); }
}

impl<T> From<Forest<T>> for ForestWalk<T> {
    fn from( forest: Forest<T> ) -> Self {
        let mut walk = Walk::<T>::default();
        if !forest.is_empty() {
            walk.on_forest( unsafe{ forest.head() as *const Node<T> });
        }
        ForestWalk{ forest, walk }
    }
}

impl<T> Into<Forest<T>> for ForestWalk<T> { fn into( self ) -> Forest<T> { self.forest }}



