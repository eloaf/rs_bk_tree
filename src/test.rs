#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = BKTree::new();
        tree.insert("book".to_string());
        tree.insert("books".to_string());
        tree.insert("cake".to_string());
        tree.insert("boo".to_string());

        let search_results = tree.search("bo", 1);
        assert!(search_results.contains(&"boo".to_string()));

        let search_results = tree.search("book", 1);
        assert!(search_results.contains(&"book".to_string()));
        assert!(search_results.contains(&"books".to_string()));
        assert!(!search_results.contains(&"cake".to_string()));
    }

    #[test]
    fn test_empty_tree() {
        let tree = BKTree::new();
        let search_results = tree.search("anything", 1);
        assert!(search_results.is_empty());
    }

    #[test]
    fn test_tolerance() {
        let mut tree = BKTree::new();
        tree.insert("example".to_string());
        tree.insert("samples".to_string());

        let search_results = tree.search("example", 2);
        assert!(search_results.contains(&"example".to_string()));
        assert!(!search_results.contains(&"samples".to_string()));
    }
}
