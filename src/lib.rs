use std::cmp::Ordering;

struct Node {
    leaf: bool,
    prefix: Vec<u8>,
    children: Vec<Node>,
}

impl Node {
    fn contains(&self, s: &[u8]) -> bool {
        match self.prefix.len().cmp(&s.len()) {
            Ordering::Less => {
                if s.starts_with(&self.prefix) {
                    let tail = &s[self.prefix.len()..];
                    let b = self.children.binary_search_by(|child| child.prefix[0].cmp(&tail[0]));
                    match b {
                        Ok(i) => self.children[i].contains(tail),
                        Err(_) => false,
                    }
                } else {
                    false
                }
            },
            Ordering::Equal => self.leaf && self.prefix == s,
            Ordering::Greater => false,
        }
    }

    fn split(&mut self, mismatch_index: usize, s: &[u8]) {
        let new_prefix;
        let mut node_a;
        let node_b;
        {
            let (common, current) = self.prefix.split_at(mismatch_index);
            node_a = Node {
                leaf: self.leaf,
                prefix: current.into(),
                children: vec![],
            };
            node_b = Node {
                leaf: true,
                prefix: s[mismatch_index..].into(),
                children: vec![],
            };
            new_prefix = common.into();
        }
        // TODO(bouk): wtf
        std::mem::swap(&mut self.children, &mut node_a.children);

        if node_a.prefix[0] < node_b.prefix[0] {
            self.children.push(node_a);
            self.children.push(node_b);
        } else {
            self.children.push(node_b);
            self.children.push(node_a);
        }

        self.leaf = false;
        self.prefix = new_prefix;
    }

    fn add_child(&mut self, s: &[u8]) -> bool {
        match self.prefix.len().cmp(&s.len()) {
            Ordering::Less => {
                let tail = &s[self.prefix.len()..];

                let b = self.children.binary_search_by(|child| child.prefix[0].cmp(&tail[0]));
                match b {
                    Ok(i) => self.children[i].insert(tail),
                    Err(i) => {
                        self.children.insert(i, Node {
                            leaf: true,
                            prefix: tail.into(),
                            children: vec![],
                        });

                        false
                    }
                }
            }
            Ordering::Greater => {
                let mut node;
                let new_prefix;
                {
                    let (common, current) = self.prefix.split_at(s.len());
                    node = Node {
                        leaf: self.leaf,
                        prefix: current.into(),
                        children: vec![],
                    };
                    new_prefix = common.into();
                }

                // TODO(bouk): wtf
                std::mem::swap(&mut self.children, &mut node.children);

                self.children.push(node);
                self.leaf = true;
                self.prefix = new_prefix;

                false
            }
            Ordering::Equal => {
                let was_leaf = self.leaf;
                self.leaf = true;

                was_leaf
            }
        }
    }

    fn insert(&mut self, s: &[u8]) -> bool {
        let p = self.prefix.iter().zip(s.iter()).position(|t| t.0 != t.1);

        match p {
            Some(mismatch_index) => {
                if mismatch_index == 0 {
                    self.add_child(s)
                } else {
                    self.split(mismatch_index, s);
                    
                    false
                }
            }
            None => {
                self.add_child(s)
            }
        }
    }

    fn len(&self) -> usize {
        let mut total = self.children.iter().map(|child| child.len()).sum();
        if self.leaf {
            total += 1
        }
        total
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Node {
                leaf: false,
                prefix: vec![],
                children: vec![],
            }
        }
    }

    /// Insert a string into the Trie
    ///
    /// Returns true if the string was already in the Trie
    pub fn insert(&mut self, s: &[u8]) -> bool {
        self.root.insert(s)
    }

    /// Returns whether a string is in the Trie
    pub fn contains(&self, s: &[u8]) -> bool {
        self.root.contains(s)
    }

    /// Returns the number of unique strings in the Trie
    pub fn len(&self) -> usize {
        self.root.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_check() {
        let mut trie = Trie::new();
        assert_eq!(0, trie.len());
        assert!(!trie.insert(b"Hey there"));
        assert_eq!(1, trie.len());

        assert!(trie.contains(b"Hey there"));
        assert!(!trie.contains(b"Hey there not"));

        assert!(trie.insert(b"Hey there"));
        assert_eq!(1, trie.len());
    }

    #[test]
    fn insert_with_split() {
        let mut trie = Trie::new();
        assert!(!trie.insert(b"What"));
        assert_eq!(1, trie.len());

        assert!(!trie.insert(b"Wow"));
        assert_eq!(2, trie.len());

        assert!(trie.insert(b"What"));
        assert!(trie.insert(b"Wow"));
        assert!(trie.contains(b"What"));
        assert!(trie.contains(b"Wow"));

        assert!(!trie.contains(b"W"));
        assert!(!trie.insert(b"W"));
        assert_eq!(3, trie.len());

        assert!(trie.contains(b"W"));
        assert!(trie.contains(b"What"));
        assert!(trie.contains(b"Wow"));
    }
}
