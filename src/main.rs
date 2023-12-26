struct BTreeNode<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> {
    node_size: usize,
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<BTreeNode<K, V>>,
}

impl<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> BTreeNode<K, V> {
    fn new(node_size: usize) -> BTreeNode<K, V> {
        BTreeNode {
            node_size,
            keys: Vec::with_capacity(node_size + 1),
            values: Vec::with_capacity(node_size + 1),
            children: Vec::with_capacity(node_size + 1),
        }
    }

    fn generate_find_path(&self, key: &K) -> Vec<usize> {
        let mut stack = Vec::<usize>::new();
        let mut current_node = self;

        loop {
            let i = BTreeNode::<K, V>::find_it(&current_node.keys, key);
            if i < 0 {
                stack.push(-(i + 1) as usize);
                break;
            } else if (i as usize) < current_node.children.len() {
                current_node = &current_node.children[i as usize];
                stack.push(i as usize);
            } else {
                stack.clear();
                break;
            }
        }

        stack.reverse();

        stack
    }

    fn find_it(keys: &Vec<K>, key: &K) -> i32 {
        let mut low = 0;
        let mut high = keys.len() as i32;

        while high != low {
            let mid = (high + low) / 2;

            if key < &keys[mid as usize] {
                high = mid;
            } else if key > &keys[mid as usize] {
                low = mid + 1;
            } else {
                // Return early, exact key found
                return -mid - 1;
            }
        }

        return low;
    }

    fn find(&self, key: &K) -> Option<V> {
        let mut current_node = self;
        let mut path = self.generate_find_path(key);
        let mut key_index = 0;

        while let Some(index) = path.pop() {
            if path.len() == 0 {
                // Last part of path is leaf node. Value is the index of k.
                key_index = index;
                break;
            }

            current_node = &current_node.children[index];
        }

        if current_node.keys[key_index] == *key {
            return Some(current_node.values[key_index].clone());
        }


        return None;
    }

    fn split(&mut self) -> BTreeNode<K, V> {
        let mid = self.keys.len() / 2;

        let mut new_node = BTreeNode::<K, V>::new(self.node_size);
        new_node.keys = self.keys.drain(mid..).collect();
        new_node.values = self.values.drain(mid..).collect();

        new_node
    }

    fn add_recursive(&mut self, key: K, value: V) -> Option<BTreeNode<K, V>> {
        let i = BTreeNode::<K, V>::find_it(&self.keys, &key);
        if self.children.len() < self.node_size + 1 {
            // Add directly to leaf node
            let index = if i < 0 {
                -(i + 1) as usize
            } else {
                i as usize
            };
            self.keys.insert(index, key);
            self.values.insert(index, value);
        } else {
            let index = i as usize;
            let children = &mut self.children;

            assert!(index <= children.len() + 1);

            let split_node = children[index].add_recursive(key.clone(), value);
            if let Some(mut new_node) = split_node {
                let new_key = new_node.keys.remove(0);
                let new_value = new_node.values.remove(0);

                children.insert(index + 1, new_node);
                self.keys.insert(index, new_key);
                self.values.insert(index, new_value);
            }
        }

        if self.keys.len() == self.node_size + 1 {
            return Some(self.split());
        }

        None
    }

    fn display(&self, depth: usize) {
        for (i, key) in self.keys.iter().enumerate() {
            if i < self.children.len() {
                self.children[i].display(depth + 1);
            }

            let k = key.clone();
            let v = self.values[i].clone();
            println!("{}{:?} = {:?} (children: {:?})", " ".repeat(depth * 2), k, v, self.children.len());
        }

        if self.children.len() > self.keys.len() {
            self.children[self.children.len() - 1].display(depth + 1);
        }
    }
}

struct BTree<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> {
    root: BTreeNode<K, V>,
}

impl<K: Ord + Clone + std::fmt::Debug, V: Ord + Clone + std::fmt::Debug> BTree<K, V> {
    fn new(node_size: usize) -> BTree<K, V> {
        BTree {
            root: BTreeNode::new(node_size),
        }
    }

    fn find(&self, k: K) -> Option<V> {
        self.root.find(&k)
    }

    fn add(&mut self, key: K, value: V) {
        let overflow = self.root.add_recursive(key, value);
        if let Some(mut overflow) = overflow {
            let newroot = BTreeNode::<K, V>::new(self.root.node_size);

            let overflow_key = overflow.keys.remove(0);
            let overflow_value = overflow.values.remove(0);

            let oldroot = std::mem::replace(&mut self.root, newroot);

            self.root.children.push(oldroot);

            self.root.keys.push(overflow_key);
            assert!(self.root.keys.len() == 1);

            self.root.values.push(overflow_value);
            assert!(self.root.values.len() == 1);

            self.root.children.push(overflow);
            assert!(self.root.children.len() == 2);
        };
    }
}


fn main() {
    let mut tree = BTree::<u64, String>::new(3);

    let data = vec![
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g"),
        (8, "h"),
        (9, "i"),
        (10, "j"),
        (11, "k"),
        (12, "l"),
        (13, "m"),
        (14, "n"),
        (15, "o"),
        (16, "p"),
        (17, "q"),
        (18, "r"),
        (19, "s"),
        (20, "t"),
        (21, "u"),
        (22, "v"),
        (23, "w"),
        (24, "x"),
        (25, "y"),
        (26, "z"),
    ];

    for (key, value) in data.iter() {
        tree.add(key.clone(), value.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::BTree;

    #[test]
    fn test_btree() {
        let mut tree = BTree::<u64, String>::new(5);

        let data = vec![
            (1, "a"),
            (2, "b"),
            (3, "c"),
            (4, "d"),
            (5, "e"),
            (6, "f"),
            (7, "g"),
            (8, "h"),
            (9, "i"),
            (10, "j"),
            (11, "k"),
            (20, "t"),
            (21, "u"),
            (22, "v"),
            (23, "w"),
            (24, "x"),
            (25, "y"),
            (26, "z"),
            (27, "z"),
            (28, "z"),
            (29, "z"),
            (30, "z"),
            (31, "z"),
            (32, "z"),
        ];

        for (i, (key, value)) in data.iter().enumerate() {
            tree.add(key.clone(), value.to_string());

            for (key, _) in data[..i + 1].iter() {
                assert!(tree.find(*key).is_some());
            }
        }

        tree.root.display(0);


        for (key, value) in data.iter() {
            assert!(tree.find(key.clone()).is_some());
        }
    }
}
