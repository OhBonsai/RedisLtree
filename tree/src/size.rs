use super::rust::*;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Size {
    pub degree   : u32, // count of children node
    pub node_cnt : u32, // count of all nodes, including itself and all its descendants
}

impl Add for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Size {
            degree: self.degree + rhs.degree,
            node_cnt: self.node_cnt + rhs.node_cnt
        }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        *self = Size {
            degree: self.degree + rhs.degree,
            node_cnt: self.node_cnt + rhs.degree,
        }
    }
}


impl Sub for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Size {
            degree: self.degree - rhs.degree,
            node_cnt: self.node_cnt - rhs.node_cnt
        }
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Size {
            degree: self.degree - rhs.degree,
            node_cnt: self.node_cnt - rhs.degree,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Size;
    

    #[test]
    fn test_add() {
        let mut s1 = Size{
            degree: 0,
            node_cnt: 0
        };

        let s2 = Size {
            degree: 2,
            node_cnt: 3
        };

        assert_eq!((s1 + s2).node_cnt, 3);
        assert_eq!((s1 + s2).degree, 2);

        s1 += s2;
        assert_eq!(s1.node_cnt, 3);
        assert_eq!(s1.degree, 2);

    }
}