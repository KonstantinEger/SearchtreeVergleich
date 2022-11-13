#[derive(Debug)]
pub struct BST<K, V>(BSTNode<K, V>);

impl<K, V> BST<K, V> {
    pub fn new() -> Self {
        Self(BSTNode::Empty)
    }
}

impl<K: Ord, V> BST<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert(key, value);
    }

    pub fn find<'a, 'b>(&'a self, key: &'b K) -> Option<&'a V> {
        self.0.find(key)
    }
}

#[derive(Debug)]
enum BSTNode<K, V> {
    Empty,
    Node(K, V, Box<BSTNode<K, V>>, Box<BSTNode<K, V>>),
}

impl<K: Ord, V> BSTNode<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        let mut current = self;
        loop {
            if current.is_empty() {
                *current = BSTNode::Node(
                    key,
                    value,
                    Box::new(BSTNode::Empty),
                    Box::new(BSTNode::Empty),
                );
                break;
            } else {
                let BSTNode::Node(k, _, left, right) = current else { unreachable!() };
                if key < *k {
                    current = left.as_mut();
                } else if *k < key {
                    current = right.as_mut();
                } else {
                    break;
                }
            }
        }
    }

    pub fn find<'a, 'b>(&'a self, key: &'b K) -> Option<&'a V> {
        let mut current = self;
        loop {
            if current.is_empty() {
                return None;
            } else {
                let BSTNode::Node(k, v, left, right) = current else { unreachable!() };
                if *k == *key {
                    return Some(v);
                } else if *k < *key {
                    current = right.as_ref();
                } else {
                    current = left.as_ref();
                }
            }
        }
    }
}

impl<K, V> BSTNode<K, V> {
    fn is_empty(&self) -> bool {
        match self {
            BSTNode::Empty => true,
            _ => false,
        }
    }
}
