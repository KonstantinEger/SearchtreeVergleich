pub mod bst;
pub mod treap;

#[derive(Debug)]
pub struct Treap<K, W, V>(TreapNode<K, W, V>);

impl<K, W, V> Treap<K, W, V> {
    pub fn new() -> Self {
        Self(TreapNode::Empty)
    }
}

impl<K: Ord, W: Ord, V> Treap<K, W, V> {
    pub fn insert(&mut self, key: K, weight: W, value: V) {
        self.0.insert(key, weight, value);
    }
}

impl<K: Ord, W, V> Treap<K, W, V> {
    pub fn find<'a, 'b>(&'a self, key: &'b K) -> Option<&'a V> {
        self.0.find(key)
    }
}

#[derive(Debug)]
enum TreapNode<K, W, V> {
    Empty,
    Node {
        key: K,
        weight: W,
        value: V,
        left: Box<TreapNode<K, W, V>>,
        right: Box<TreapNode<K, W, V>>,
    },
}

impl<K: Ord, W: Ord, V> TreapNode<K, W, V> {
    pub fn insert(&mut self, key: K, weight: W, value: V) {
        match self.take() {
            TreapNode::Empty => {
                *self = TreapNode::Node {
                    key,
                    weight,
                    value,
                    left: Box::new(TreapNode::Empty),
                    right: Box::new(TreapNode::Empty),
                }
            }
            TreapNode::Node {
                key: ckey,
                weight: cweight,
                value: cvalue,
                mut left,
                mut right,
            } => {
                let should_restore = if key < ckey {
                    left.insert(key, weight, value);
                    ShouldRestore::Left
                } else if ckey < key {
                    right.insert(key, weight, value);
                    ShouldRestore::Right
                } else {
                    ShouldRestore::No
                };
                *self = TreapNode::Node {
                    key: ckey,
                    weight: cweight,
                    value: cvalue,
                    left,
                    right,
                };
                match should_restore {
                    ShouldRestore::Left => self.restore_left(),
                    ShouldRestore::Right => self.restore_right(),
                    ShouldRestore::No => {}
                }
            }
        }
    }

    fn restore_left(&mut self) {
        let zweight = self.weight();
        let xweight = self.left().and_then(Self::weight);
        let is_resorable = zweight
            .zip(xweight)
            .map(|(zw, xw)| xw < zw)
            .unwrap_or(false);
        if !is_resorable {
            return;
        }

        let TreapNode::Node {
            key: zkey,
            weight: zweight,
            value: zvalue,
            left: mut zleft,
            right: zright
        } = self.take() else { unreachable!() };
        let TreapNode::Node {
            key: xkey,
            weight: xweight,
            value: xvalue,
            left: xleft,
            right: xright
        } = zleft.take() else { unreachable!() };
        let z = TreapNode::Node {
            key: zkey,
            weight: zweight,
            value: zvalue,
            left: xright,
            right: zright,
        };
        let x = TreapNode::Node {
            key: xkey,
            weight: xweight,
            value: xvalue,
            left: xleft,
            right: Box::new(z),
        };
        *self = x;
    }

    fn restore_right(&mut self) {
        let zweight = self.weight();
        let xweight = self.right().and_then(Self::weight);
        let is_resorable = zweight
            .zip(xweight)
            .map(|(zw, xw)| xw < zw)
            .unwrap_or(false);
        if !is_resorable {
            return;
        }

        let TreapNode::Node {
            key: zkey,
            weight: zweight,
            value: zvalue,
            left: zleft,
            right: mut zright
        } = self.take() else { unreachable!() };
        let TreapNode::Node {
            key: xkey,
            weight: xweight,
            value: xvalue,
            left: xleft,
            right: xright
        } = zright.take() else { unreachable!() };
        let z = TreapNode::Node {
            key: zkey,
            weight: zweight,
            value: zvalue,
            left: zleft,
            right: xleft,
        };
        let x = TreapNode::Node {
            key: xkey,
            weight: xweight,
            value: xvalue,
            left: Box::new(z),
            right: xright,
        };
        *self = x;
    }
}

impl<K: Ord, W, V> TreapNode<K, W, V> {
    pub fn find<'a, 'b>(&'a self, search_key: &'b K) -> Option<&'a V> {
        let mut current: &TreapNode<K, W, V> = self;
        loop {
            match current {
                TreapNode::Empty => {
                    return None;
                }
                TreapNode::Node {
                    key,
                    value,
                    left,
                    right,
                    ..
                } => {
                    if *key == *search_key {
                        return Some(value);
                    } else if *key < *search_key {
                        current = right.as_ref();
                    } else {
                        current = left.as_ref()
                    }
                }
            }
        }
    }
}

impl<K, W, V> TreapNode<K, W, V> {
    fn take(&mut self) -> Self {
        std::mem::replace(self, TreapNode::Empty)
    }

    fn weight(&self) -> Option<&W> {
        match self {
            TreapNode::Node { weight, .. } => Some(weight),
            TreapNode::Empty => None,
        }
    }

    fn left(&self) -> Option<&TreapNode<K, W, V>> {
        match self {
            TreapNode::Node { left, .. } => Some(left.as_ref()),
            TreapNode::Empty => None,
        }
    }

    fn right(&self) -> Option<&TreapNode<K, W, V>> {
        match self {
            TreapNode::Node { right, .. } => Some(right.as_ref()),
            TreapNode::Empty => None,
        }
    }
}

enum ShouldRestore {
    No,
    Left,
    Right,
}
