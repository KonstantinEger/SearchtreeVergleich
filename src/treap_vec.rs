use std::cmp::Ordering;

#[derive(Debug)]
pub struct Treap<K, W, V>(Vec<TreapNode<K, W, V>>);

#[derive(Debug)]
enum TreapNode<K, W, V> {
    Empty(Option<usize>),
    Node {
        parent: Option<usize>,
        key: K,
        weight: W,
        value: V,
        left: usize,
        right: usize,
    },
}

impl<K: Ord, W: Ord, V> Treap<K, W, V> {
    pub fn insert(&mut self, ikey: K, iweight: W, ival: V) {
        let mut current = 0;
        let mut curr_parent = None;
        loop {
            if self.0[current].is_empty() {
                self.0.push(TreapNode::Empty(Some(current)));
                self.0.push(TreapNode::Empty(Some(current)));
                let left = self.0.len() - 1;
                let right = self.0.len() - 2;
                self.0[current] = TreapNode::Node {
                    parent: curr_parent,
                    key: ikey,
                    weight: iweight,
                    value: ival,
                    left,
                    right,
                };
                break;
            }
            let (ord, left, right) = {
                let c = &self.0[current];
                let TreapNode::Node {
                    key,
                    left,
                    right,
                    ..
                } = &*c else { unreachable!() };
                let ord = ikey.cmp(key);
                (ord, *left, *right)
            };
            curr_parent = Some(current);
            match ord {
                Ordering::Less => current = left,
                Ordering::Greater => current = right,
                Ordering::Equal => return,
            }
        }

        loop {
            let parent = self.0[current].parent_idx();
            let parent_idx = if let Some(p) = parent {
                p
            } else {
                break;
            };
            self.restore_heap_property(parent_idx);
            current = parent_idx;
        }
    }

    fn restore_heap_property(&mut self, parent_idx: usize) -> ShouldRestore {
        if self.0[parent_idx].is_empty() {
            ShouldRestore::Nothing
        } else {
            let sr = {
                let weight = self.0[parent_idx].weight().unwrap();
                let (left, right) = self.0[parent_idx].children_idx().unwrap();
                let left_weight = self.0[left].weight();
                let right_weight = self.0[right].weight();
                if left_weight.map_or(false, |w| w < weight) {
                    ShouldRestore::Left
                } else if right_weight.map_or(false, |w| w < weight) {
                    ShouldRestore::Right
                } else {
                    ShouldRestore::Nothing
                }
            };

            match sr {
                ShouldRestore::Nothing => {},
                ShouldRestore::Left => {
                    let TreapNode::Node {
                        parent: zparent,
                        key: zkey,
                        weight: zweight,
                        value: zvalue,
                        left: zleft,
                        right: zright,
                    } = self.0[parent_idx].take() else { unreachable!() };
                    let TreapNode::Node {
                        parent: _xparent,
                        key: xkey,
                        weight: xweight,
                        value: xvalue,
                        left: xleft,
                        right: xright,
                    } = self.0[zleft].take() else { unreachable!() };
                    let zidx = zleft;
                    self.0[parent_idx] = TreapNode::Node {
                        parent: zparent,
                        key: xkey,
                        weight: xweight,
                        value: xvalue,
                        left: xleft,
                        right: zidx,
                    };
                    self.0[zidx] = TreapNode::Node {
                        parent: Some(parent_idx),
                        key: zkey,
                        weight: zweight,
                        value: zvalue,
                        left: xright,
                        right: zright,
                    };
                    *self.0[xleft].parent_mut() = Some(parent_idx);
                    *self.0[xright].parent_mut() = Some(zidx);
                    *self.0[zright].parent_mut() = Some(zidx);
                },
                ShouldRestore::Right => {
                    let TreapNode::Node {
                        parent: zparent,
                        key: zkey,
                        weight: zweight,
                        value: zvalue,
                        left: zleft,
                        right: zright,
                    } = self.0[parent_idx].take() else { unreachable!() };
                    let TreapNode::Node {
                        parent: _xparent,
                        key: xkey,
                        weight: xweight,
                        value: xvalue,
                        left: xleft,
                        right: xright,
                    } = self.0[zright].take() else { unreachable!() };
                    let zidx = zright;
                    self.0[zidx] = TreapNode::Node {
                        key: zkey,
                        parent: Some(parent_idx),
                        weight: zweight,
                        value: zvalue,
                        left: zleft,
                        right: xleft,
                    };
                    self.0[parent_idx] = TreapNode::Node {
                        parent: zparent,
                        key: xkey,
                        weight: xweight,
                        value: xvalue,
                        left: zidx,
                        right: xright,
                    };
                    *self.0[zleft].parent_mut() = Some(zidx);
                    *self.0[xleft].parent_mut() = Some(zidx);
                    *self.0[xright].parent_mut() = Some(parent_idx);
                }
            }
            sr
        }
    }
}

impl<K: Ord, W, V> Treap<K, W, V> {
    pub fn find<'a, 'b>(&'a self, find_key: &'b K) -> Option<&'a V> {
        let mut current = 0;
        loop {
            let next;
            match &self.0[current] {
                TreapNode::Empty(_) => return None,
                TreapNode::Node {
                    key,
                    left,
                    right,
                    value,
                    ..
                } => {
                    next = if *find_key < *key {
                        *left
                    } else if *key < *find_key {
                        *right
                    } else {
                        return Some(value);
                    }
                }
            }
            current = next;
        }
    }
}

impl<K, W, V> Treap<K, W, V> {
    pub fn new() -> Self {
        Self(vec![TreapNode::Empty(None)])
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut v = Vec::with_capacity(cap);
        v.push(TreapNode::Empty(None));
        Self(v)
    }
}

enum ShouldRestore {
    Left,
    Right,
    Nothing,
}

impl<K, W, V> TreapNode<K, W, V> {
    pub fn is_empty(&self) -> bool {
        match self {
            TreapNode::Empty(_) => true,
            _ => false,
        }
    }

    pub fn parent_idx(&self) -> Option<usize> {
        match self {
            Self::Empty(p) => *p,
            Self::Node { parent, .. } => *parent,
        }
    }

    pub fn parent_mut(&mut self) -> &mut Option<usize> {
        match self {
            Self::Empty(p) => p,
            Self::Node { parent, .. } => parent,
        }
    }

    pub fn children_idx(&self) -> Option<(usize, usize)> {
        match self {
            Self::Empty(_) => None,
            Self::Node {
                left,
                right,
                ..
            } => Some((*left, *right))
        }
    }

    pub fn weight(&self) -> Option<&W> {
        match self {
            TreapNode::Empty(_) => None,
            TreapNode::Node { weight, .. } => Some(weight),
        }
    }

    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Self::Empty(None))
    }
}
