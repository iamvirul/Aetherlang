#[cfg(test)]
mod tests {
    use crate::core::collections::{AetherList, AetherMap};

    #[test]
    fn test_list_basic_operations() {
        let mut list = AetherList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.len(), 2);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_edge_cases() {
        let mut list = AetherList::new();
        
        // Test pop on empty list
        assert_eq!(list.pop(), None);
        
        // Test clear on empty list
        list.clear();
        assert!(list.is_empty());
        
        // Test pushing and popping large number of items
        for i in 0..1000 {
            list.push(i);
        }
        assert_eq!(list.len(), 1000);
        
        for i in (0..1000).rev() {
            assert_eq!(list.pop(), Some(i));
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_display() {
        let mut list = AetherList::new();
        
        // Test empty list display
        assert_eq!(list.to_string(), "[]");
        
        // Test single item
        list.push(1);
        assert_eq!(list.to_string(), "[1]");
        
        // Test multiple items
        list.push(2);
        list.push(3);
        assert_eq!(list.to_string(), "[1, 2, 3]");
        
        // Test after clear
        list.clear();
        assert_eq!(list.to_string(), "[]");
    }

    #[test]
    fn test_map_basic_operations() {
        let mut map = AetherMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        let key1 = String::from("one");
        let key2 = String::from("two");
        let key3 = String::from("three");

        map.insert(key1.clone(), 1);
        map.insert(key2.clone(), 2);
        map.insert(key3.clone(), 3);

        assert_eq!(map.len(), 3);
        assert!(!map.is_empty());
        assert!(map.contains_key(&key1));
        assert_eq!(map.get(&key2), Some(&2));

        assert_eq!(map.remove(&key3), Some(3));
        assert_eq!(map.len(), 2);

        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_edge_cases() {
        let mut map = AetherMap::new();
        let key = String::from("key");
        
        // Test operations on empty map
        assert_eq!(map.get(&key), None);
        assert_eq!(map.remove(&key), None);
        assert!(!map.contains_key(&key));
        
        // Test clear on empty map
        map.clear();
        assert!(map.is_empty());
        
        // Test inserting empty string key
        let empty_key = String::new();
        map.insert(empty_key.clone(), 1);
        assert_eq!(map.get(&empty_key), Some(&1));
        
        // Test multiple operations on same key
        map.insert(key.clone(), 1);
        map.insert(key.clone(), 2);
        map.insert(key.clone(), 3);
        assert_eq!(map.get(&key), Some(&3));
        
        // Test remove and reinsert
        assert_eq!(map.remove(&key), Some(3));
        map.insert(key.clone(), 4);
        assert_eq!(map.get(&key), Some(&4));
    }

    #[test]
    fn test_map_overwrite() {
        let mut map = AetherMap::new();
        let key = String::from("key");
        
        map.insert(key.clone(), 1);
        assert_eq!(map.get(&key), Some(&1));

        map.insert(key.clone(), 2);
        assert_eq!(map.get(&key), Some(&2));
        
        // Test overwrite with same value
        map.insert(key.clone(), 2);
        assert_eq!(map.get(&key), Some(&2));
        
        // Test remove and check old value is gone
        map.remove(&key);
        assert_eq!(map.get(&key), None);
    }
} 