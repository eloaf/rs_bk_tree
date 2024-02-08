use strsim::levenshtein;
use std::io::BufRead;



struct BKNode {
    word: String,
    children: Vec<(usize, Box<BKNode>)>,
}

impl BKNode {
    fn new(word: String) -> Self {
        BKNode {
            word,
            children: Vec::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let distance = levenshtein(&self.word, &word);
        if distance > 0 {
            match self.children.iter_mut().find(|(d, _)| *d == distance) {
                Some((_, node)) => node.insert(word),
                None => self.children.push((distance, Box::new(BKNode::new(word)))),
            }
        }
    }

    fn search(&self, word: &str, tolerance: usize, results: &mut Vec<String>) {
        let distance = levenshtein(&self.word, word);
        if distance <= tolerance {
            // results.push(self.word.to_string());
            results.push(self.word.clone());
        }

        for &(d, ref child) in &self.children {
            if d >= distance.saturating_sub(tolerance) && d <= distance + tolerance {
                child.search(word, tolerance, results);
            }
        }
    }
}

struct BKTree {
    root: Option<Box<BKNode>>,
}

impl BKTree {
    fn new() -> Self {
        BKTree { root: None }
    }

    fn insert(&mut self, word: String) {
        match self.root {
            Some(ref mut node) => node.insert(word),
            None => self.root = Some(Box::new(BKNode::new(word))),
        }
    }

    fn search(&self, word: &str, tolerance: usize) -> Vec<String> {
        let mut results = Vec::new();
        if let Some(ref node) = self.root {
            node.search(word, tolerance, &mut results);
        }
        results
    }
}

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

    #[test]
    fn test_nearest_neighbours() {
        let mut tree = BKTree::new();

        // Open file with one string per line
        let file = std::fs::File::open("words_10000.txt").unwrap();
        let reader = std::io::BufReader::new(file);
        // Load the strings into the tree
    
        // NOTE: example
        // fn sum_of_squares(input: &[i32]) -> i32 {
        //     input.par_iter() // <-- just change that!
        //          .map(|&i| i * i)
        //          .sum()
        // }

        // reader.lines().par_iter().map(|&i| tree.insert(i.unwrap().to_string()));

        for line in reader.lines() {
            // print the line
            // println!("{}", line.unwrap());
            match line {
                Ok(line) => {
                    // Print the line
                    // println!("{}", line);
                    tree.insert(line.to_string());
                }
                Err(e) => {
                    // Handle the error (e.g., print it)
                    eprintln!("Error reading line: {}", e);
                }
            }
        }

        // HVQLVESGGDSVQAGGSLRLSCTLSGYPYSTAVMGWFRQNSGNEREGVAAFYNDGGDPHYADSVKGRFTISKDNANNTLYLQMNSLKSEDTGMYYCAADNWRCGGSWSEVRVPYWGQGTQVTVS
        // VQLVESGGGLVQPGGSLRLSCGASGVYFRGYHMSWFRQAPGKEREFVESIINSGRNTTYADFVRGRFAVSRDNNKNTVYLEMNNLKPEDTAIYYCARPSGDYWGQGTQVTVSS
        // VQLVESGGGSVEAGGSLSLSCTASGYILRTKCMGWFRDGPGMEREGVAAVYVGGGTYYTDSVKGRFTISQDNAKNTLYLQMNSLKPEDTAMYYCAALKGSWSCGHWAKYYYWAQGTQVTVS
        // VQLVESGGGLVQAGGSLRLSCAASGRTFSSYRMGWFRQAPGKERDFVAAISWSGGSTYYADSVKGRFTISRDNAKNTVYLQMNSLKPEDTADYFCAASWFNSVTYYRERSYHYWGQGTQVTVTS
        // VQLVESGGGLVQPGGSLRFSCVASGFTFSSYAMSWVRQAPGKGLEWVSSINRGGGSTYYADSVKGRFTVSRDDTVNTLYLQLNSLKTEDTAMYYCASGYLVAGNPDWGQGTQVTVS
        let search_results = tree.search("HVQLVESGGDSVQAGGSLRLSCTLSGYPYSTAVMGWFRQNSGNEREGVAAFYNDGGDPHYADSVKGRFTISKDNANNTLYLQMNSLKSEDTGMYYCAADNWRCGGSWSEVRVPYWGQGTQVTVS", 30);
        println!("{:?}", search_results);
    }
}

