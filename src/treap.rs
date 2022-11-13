use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Treap<K, W, V>(Rc<RefCell<TreapNode<K, W, V>>>);

#[derive(Debug)]
enum TreapNode<K, W, V> {
    Empty(Weak<RefCell<TreapNode<K, W, V>>>),
    Node {
        parent: Weak<RefCell<TreapNode<K, W, V>>>,
        key: K,
        weight: W,
        value: V,
        left: Rc<RefCell<TreapNode<K, W, V>>>,
        right: Rc<RefCell<TreapNode<K, W, V>>>,
    },
}

impl<K: Ord, W: Ord, V> Treap<K, W, V> {
    pub fn insert(&mut self, ikey: K, iweight: W, ival: V) {
        let mut current = Rc::clone(&self.0);
        let mut curr_parent = Weak::new();
            loop {
                if current.borrow().is_empty() {
                    let mut c = current.borrow_mut();
                    *c = TreapNode::Node {
                        parent: curr_parent,
                        key: ikey,
                        weight: iweight,
                        value: ival,
                        left: Rc::new(RefCell::new(TreapNode::Empty(Rc::downgrade(&current)))),
                        right: Rc::new(RefCell::new(TreapNode::Empty(Rc::downgrade(&current)))),
                    };
                    break;
                }
                let (ord, left, right) = {
                    let c = current.borrow();
                    let TreapNode::Node {
                        key,
                        left,
                        right,
                        ..
                    } = &*c else { unreachable!() };
                    let ord = ikey.cmp(key);
                    let left = Rc::clone(left);
                    let right = Rc::clone(right);
                    (ord, left, right)
                };
                curr_parent = Rc::downgrade(&current);
                match ord {
                    Ordering::Less => current = left,
                    Ordering::Greater => current = right,
                    Ordering::Equal => return,
                }
            }

            loop {
                let parent = current.borrow().parent();
                let parent = if let Some(p) = parent {
                    p
                } else {
                    break;
                };
                let restored = parent.borrow_mut().restore_heap_property();
                let parent_clone = Rc::downgrade(&parent);
                match restored {
                    ShouldRestore::Left => {
                        let TreapNode::Node { right, .. } = &*parent.borrow() else { unreachable!() };
                        let TreapNode::Node { parent, ..} = &mut *right.borrow_mut() else { unreachable!() };
                        let _ = std::mem::replace(parent, parent_clone);
                    }
                    ShouldRestore::Right => {
                        let TreapNode::Node { left, .. } = &*parent.borrow() else { unreachable!() };
                        let TreapNode::Node { parent, ..} = &mut *left.borrow_mut() else { unreachable!() };
                        let _ = std::mem::replace(parent, parent_clone);
                    }
                    ShouldRestore::Nothing => {}
                };
                current = parent;
            }
    }
}

pub struct Find<'a, V> {
    p: *const V,
    _phantom: std::marker::PhantomData<&'a V>,
}

impl<K: Ord, W, V> Treap<K, W, V> {
    pub fn find<'a, 'b>(&'a self, find_key: &'b K) -> Option<Find<'a, V>> {
        let mut current = Rc::clone(&self.0);
            loop {
                let cb = current.borrow();
                let next;
                match &*cb {
                    TreapNode::Empty(_) => return None,
                    TreapNode::Node {
                        key,
                        left,
                        right,
                        value,
                        ..
                    } => {
                        next = if *find_key < *key {
                            Rc::clone(left)
                        } else if *key < *find_key {
                            Rc::clone(right)
                        } else {
                            return Some(Find {
                                p: value,
                                _phantom: std::marker::PhantomData,
                            });
                        }
                    }
                }
                drop(cb);
                current = next;
            }
    }
}

impl<'a, V> std::ops::Deref for Find<'a, V> {
    type Target = V;

    fn deref(&self) -> &'a V {
        unsafe { &*self.p }
    }
}

impl<K, W, V> Treap<K, W, V> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(TreapNode::Empty(Weak::new()))))
    }
}

impl<K, W: Ord, V> TreapNode<K, W, V> {
    pub fn restore_heap_property(&mut self) -> ShouldRestore {
        match self {
            TreapNode::Empty(_) => ShouldRestore::Nothing,
            TreapNode::Node {
                weight,
                left,
                right,
                ..
            } => {
                let sr = {
                    let lb = left.borrow();
                    let rb = right.borrow();
                    let left_weight = lb.weight();
                    let right_weight = rb.weight();
                    if left_weight.map_or(false, |w| w < weight) {
                        ShouldRestore::Left
                    } else if right_weight.map_or(false, |w| w < weight) {
                        ShouldRestore::Right
                    } else {
                        ShouldRestore::Nothing
                    }
                };

                match sr {
                    ShouldRestore::Left => {
                        let TreapNode::Node {
                            parent: zparent,
                            key: zkey,
                            weight: zweight,
                            value: zvalue,
                            left: zleft,
                            right: zright,
                        } = self.take() else { unreachable!() };
                        let TreapNode::Node {
                            parent: _xparent,
                            key: xkey,
                            weight: xweight,
                            value: xvalue,
                            left: xleft,
                            right: xright,
                        } = zleft.borrow_mut().take() else { unreachable!() };
                        let new_right = Rc::new(RefCell::new(TreapNode::Node {
                            key: zkey,
                            parent: Weak::new(),
                            weight: zweight,
                            value: zvalue,
                            left: xright,
                            right: zright,
                        }));
                        let _ = std::mem::replace(
                            self,
                            TreapNode::Node {
                                parent: zparent,
                                key: xkey,
                                weight: xweight,
                                value: xvalue,
                                left: xleft,
                                right: new_right,
                            },
                        );
                    }
                    ShouldRestore::Right => {
                        let TreapNode::Node {
                            parent: zparent,
                            key: zkey,
                            weight: zweight,
                            value: zvalue,
                            left: zleft,
                            right: zright,
                        } = self.take() else { unreachable!() };
                        let TreapNode::Node {
                            parent: _xparent,
                            key: xkey,
                            weight: xweight,
                            value: xvalue,
                            left: xleft,
                            right: xright,
                        } = zright.borrow_mut().take() else { unreachable!() };
                        let new_left = Rc::new(RefCell::new(TreapNode::Node {
                            key: zkey,
                            parent: Weak::new(),
                            weight: zweight,
                            value: zvalue,
                            left: zleft,
                            right: xleft,
                        }));
                        let _ = std::mem::replace(
                            self,
                            TreapNode::Node {
                                parent: zparent,
                                key: xkey,
                                weight: xweight,
                                value: xvalue,
                                left: new_left,
                                right: xright,
                            },
                        );
                    }
                    ShouldRestore::Nothing => {}
                };
                sr
            },
        }
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

    pub fn left(&self) -> Option<Rc<RefCell<Self>>> {
        match self {
            TreapNode::Empty(_) => None,
            TreapNode::Node { left, .. } => Some(Rc::clone(left)),
        }
    }

    pub fn take(&mut self) -> Self {
        let parent = match &*self {
            TreapNode::Empty(p) => Weak::clone(p),
            TreapNode::Node { parent, .. } => Weak::clone(parent),
        };
        std::mem::replace(self, TreapNode::Empty(parent))
    }

    pub fn weight(&self) -> Option<&W> {
        match self {
            TreapNode::Empty(_) => None,
            TreapNode::Node { weight, .. } => Some(weight),
        }
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Self>>> {
        match self {
            TreapNode::Empty(p) => Weak::upgrade(p),
            TreapNode::Node { parent, .. } => Weak::upgrade(parent),
        }
    }
}
