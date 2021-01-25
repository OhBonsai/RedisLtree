use super::size::Size;
use super::rust::*;


#[derive(Debug, PartialEq, Eq)]
pub struct Visit<T> {
    pub data: T,
    pub size: Size,
}

pub enum Bfs<Iter> {
    Tree( BfsTree<Iter>),
    Forest( BfsForest<Iter>)
}

pub struct BfsTree<Iter> {
    pub iter: Iter,
    pub size: Size
}

pub struct BfsForest<Iter> {
    pub iter : Iter,
    pub size : Size,
}

pub struct Splitted<Iter> {
    pub(crate) iters: VecDeque<Iter>
}

pub trait Split {
    type Item;
    type Iter: ExactSizeIterator;

    fn split( self ) -> ( Self::Item, Self::Iter, u32 );
}


impl<Item,Iter> BfsTree<Splitted<Iter>>
    where Iter: Iterator<Item=Item>
{
    pub fn from<Treelike>( treelike: Treelike, size: Size ) -> Self
        where Treelike: IntoIterator<Item=Item,IntoIter=Iter>
    {
        Self{ iter: Splitted::<Iter>::from( treelike ), size }
    }
}

impl<Iter> BfsTree<Iter> {
    pub fn wrap( self ) -> Bfs<Iter> { Bfs::Tree( self )}
}



impl<Item,Iter> BfsForest<Splitted<Iter>>
    where Iter: Iterator<Item=Item>
{
    pub fn from<Treelike>( treelike: Treelike, size: Size ) -> Self
        where Treelike: IntoIterator<Item=Item,IntoIter=Iter>
    {
        Self{ iter: Splitted::<Iter>::from( treelike ), size }
    }
}

impl<Iter> BfsForest<Iter> {
    pub fn wrap( self ) -> Bfs<Iter> { Bfs::Forest( self )}
}


impl <T, Iter> Bfs<Iter>
    where Iter: Iterator<Item=Visit<T>> {

    pub fn iter(self) -> Iter {
        match self {
            Bfs::Tree(tree) => tree.iter,
            Bfs::Forest(forest) => forest.iter,
        }

    }

    pub fn iter_and_size( self ) -> ( Iter, Size ) {
        match self {
            Bfs::Tree(   tree   ) => (tree.iter,   tree.size),
            Bfs::Forest( forest ) => (forest.iter, forest.size),
        }
    }

    pub fn tree_iter( self ) -> Option<Iter> {
        match self {
            Bfs::Tree( tree ) => Some( tree.iter ),
            _ => None,
        }
    }

    pub fn forest_iter( self ) -> Option<Iter> {
        match self {
            Bfs::Forest(forest) => Some(forest.iter),
            _ => None,
        }
    }
}

impl<TreeLike, Iter, Item> From<TreeLike> for Splitted<Iter>
    where TreeLike: IntoIterator<Item=Item, IntoIter=Iter>,
                Iter: Iterator<Item=Item>{
    fn from(treelike: TreeLike) -> Self {
        let mut iters = VecDeque::new();
        iters.push_back(treelike.into_iter());
        Splitted{iters}
    }
}


impl<T,Item,Iter> Iterator for Splitted<Iter>
    where Iter : ExactSizeIterator<Item=Item>
    , Item : Split<Iter=Iter,Item=T>
{
    type Item = Visit<T>;

    #[inline] fn next( &mut self ) -> Option<Self::Item> {
        loop {
            let next_item =
                if let Some( ref mut iter ) = self.iters.front_mut() {
                    iter.next()
                } else {
                    return None;
                };
            if let Some( item ) = next_item {
                let ( data, iter, node_cnt ) = item.split();
                let degree = iter.len();
                self.iters.push_back( iter );
                return Some( Visit{ data, size: Size{ degree: degree as u32, node_cnt }});
            } else {
                self.iters.pop_front();
            }
        }
    }
}

