use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct AetherList<T> {
    items: Vec<T>,
}

impl<T> AetherList<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[derive(Clone, Debug)]
pub struct AetherMap<K, V> {
    items: HashMap<K, V>,
}

impl<K, V> AetherMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.items.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.items.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.items.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T: fmt::Display> fmt::Display for AetherList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
} 