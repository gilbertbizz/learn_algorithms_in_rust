use std::fmt::Debug;
use std::mem;

#[derive(Debug)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>
}

#[derive(Debug)]
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

#[derive(Debug)]
pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

impl<T> Node<T> 
where T: Ord,
{
    fn new(degree: usize, keys: Option<Vec<T>>, children: Option<Vec<Node<T>>>) -> Self {
        let keys = if let Some(key) = keys {
            key
        } else {
            Vec::with_capacity(degree - 1)
        };

        let children = if let Some(c) = children {
            c
        } else {
            Vec::with_capacity(degree)
        };

        Self {
            keys,
            children
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl BTreeProps {
   fn new(degree: usize) -> Self {
        Self {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }

    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }

   fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[self.mid_key_index];
        let right_keys = if let Some((_first, others)) = child.keys.split_off(self.mid_key_index).split_first() {
            others.to_vec()
        } else {
            Vec::with_capacity(self.max_keys)
        };
        let right_children = if !child.is_leaf() {
            Some(child.children.split_off(self.mid_key_index + 1))
        } else {
            None
        };

        let new_child_node: Node<T> = Node::new(self.degree, Some(right_keys), right_children);

        parent.keys.insert(child_index, middle_key);
        parent.children.insert(child_index + 1, new_child_node);
    }

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
        let mut index: isize = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while index >= 0 && node.keys[index as usize] >= key {
            index -= 1;
        }

        let mut u_index: usize = usize::try_from(index + 1).ok().unwrap();
        if node.is_leaf() {
            node.keys.insert(u_index, key);
        } else {
            if self.is_maxed_out(&node.children[u_index]) {
                self.split_child(node, u_index);
                if node.keys[u_index] < key {
                    u_index += 1;
                }
            }
            self.insert_non_full(&mut node.children[u_index], key);
        }
    }

     fn traverse_node<T: Ord + Debug>(node: &Node<T>, depth: usize) {
        if node.is_leaf() {
            println!(" {0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
        } else {
            let _depth = depth + 1;
            for (index, key) in node.keys.iter().enumerate() {
                Self::traverse_node(&node.children[index], _depth);

                println!("{0:{<1$}{2:?}{0:}<1$}", "", depth, key);
            }
            Self::traverse_node(node.children.last().unwrap(), _depth);
        }
    }
}

impl<T> BTree<T>
where 
    T: Ord + Copy + Debug + Default
{
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;

        Self {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }

    pub fn insert(&mut self, key: T) {
        if self.props.is_maxed_out(&self.root) {
            let mut new_root = Node::new(self.props.degree, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            self.props.split_child(&mut self.root, 0);
        }
        self.props.insert_non_full(&mut self.root, key);
    }

    pub fn traverse(&self) {
        BTreeProps::traverse_node(&self.root, 0);
        println!();
    }

    pub fn search(&self, key: T) -> bool {
        let mut current_node = &self.root;
        let mut index: isize;
        
        loop {
            index = isize::try_from(current_node.keys.len()).ok().unwrap() - 1;
            while index >= 0 && current_node.keys[index as usize] > key {
                index -= 1;
            }

            let u_index: usize = usize::try_from(index + 1).ok().unwrap();
            if index >= 0 && current_node.keys[u_index - 1] == key {
                break true;
            } else if current_node.is_leaf() {
                break false;
            } else {
                current_node = &current_node.children[u_index];
            }
        }
    }
}