use std::collections::{HashMap, HashSet};
use crate::MAX;

// Define a structure for Trie nodes. Each node should store its children and a flag indicating if it represents the end of a word.
pub struct TrieNode {
    nodes: HashMap<char, TrieNode>,
    word: Option<String>,
    eow: bool
}

// Define the Trie structure itself, which includes the root node.
pub struct Trie {
    root: TrieNode
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            nodes: HashMap::new(),
            eow: false,
            word: None
        }
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            // The Trie struct will contain a root node which is an instance of TrieNode.
            root: TrieNode::new()
        }
    }


    // The insert method will add a word to the Trie by iterating through its characters and creating child nodes as necessary.
    pub fn insert(&mut self, word: &str) {
        if word.len() <= MAX {
            panic!("Word is too short to be inserted in trie")
        }

        let mut rootNode = &mut self.root;
        for key in word.chars() {
            rootNode = rootNode.nodes.entry(key).or_insert_with(TrieNode::new)
        }

        rootNode.word = Some(word.to_string());
        rootNode.eow = true;
    }

    // The search method will check if a word exists in the Trie by traversing the nodes according to the word's characters or wildcards (unknown characters).
    pub fn search(&self, sequence: &str) -> HashSet<String> {
        let rootNode = &self.root;
        let mut items: HashSet<String> = HashSet::new();
        // This function ensures traversing the Trie and passing available words in an array
        self.dfsPatternSearch(rootNode, sequence, 0, &mut items);

        items
    }

    // Recursive DFS function to find words that match the current partial pattern.
    fn dfsPatternSearch(&self, mut rootNode: &TrieNode, sequence: &str, idx: usize, mut items: &mut HashSet<String>) -> bool {
        if sequence.len() == idx {
            if rootNode.eow {
                let word = rootNode.word.clone().unwrap();
                // Collect all words that match the pattern
                items.insert(word);
            }
            return rootNode.eow;
        }

        // Continue until eow or could not find the pattern occurs
        if sequence.chars().nth(idx).eq(&Some('?')) {
            // Handle the wildcards and traverse every child node of current root accordingly.
            for nextNode in rootNode.nodes.values() {
                let result = self.dfsPatternSearch(&nextNode, sequence, idx + 1, items);
            }
        } else if let Some(nextNode) = rootNode.nodes.get(&sequence.chars().nth(idx).unwrap()) {
            // If the node is not a wildcards follow the child node if exists
            return self.dfsPatternSearch(nextNode, sequence, idx + 1, items)
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::ascii::AsciiExt;
    use super::*;

    #[test]
    fn test_word() {
        let word = "motorcycle";
        let mut trie = Trie::new();

        trie.insert(word);
    }

    #[test]
    #[should_panic(expected = "too short")]
    fn test_small_word() {
        let shortWord = "car";
        let mut trie = Trie::new();

        trie.insert(shortWord);
    }

    #[test]
    fn test_full_search_single_result() {
        let dictionary = vec!["space", "place", "craze", "crate", "state", "plate", "blade", "blato"];

        let mut trie = Trie::new();

        for word in dictionary {
            trie.insert(word);
        }

        let result = trie.search("space");

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_full_search() {
        let dictionary = vec!["space", "place", "craze", "crate", "state", "plate", "blade", "blato"];

        let mut trie = Trie::new();

        for word in dictionary {
            trie.insert(word);
        }

        let result = trie.search("space");

        assert_eq!(result.iter().nth(0).unwrap(), "space");
    }

    #[test]
    fn test_full_search_no_result() {
        let dictionary = vec!["space", "place", "craze", "crate", "state", "plate", "blade", "blato"];

        let mut trie = Trie::new();

        for word in dictionary {
            trie.insert(word);
        }

        let result = trie.search("water");

        match result.iter().nth(0) {
            Some(value) => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn test_wildcard_search_size() {
        let dictionary = vec!["space", "place", "craze", "crate", "state", "plate", "blade", "blato"];

        let mut trie = Trie::new();

        for word in dictionary {
            trie.insert(word);
        }

        let result = trie.search("??a?e");

        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_wildcard_search_items() {
        let dictionary = vec!["space", "place", "craze", "crate", "state", "plate", "blade", "blato"];

        let mut trie = Trie::new();

        for word in dictionary {
            trie.insert(word);
        }

        let mut result = trie.search("??a?e");


        let toCompare = HashSet::from(["state".to_string(), "space".to_string(), "blade".to_string(), "craze".to_string(), "crate".to_string(), "plate".to_string(), "place".to_string()]);
        assert_eq!(result, toCompare)
    }

    #[test]
    fn test_wildcard_search_different_length_items() {
        let dictionary = vec!["star", "stare", "story", "stories", "start", "staring"];

        let mut trie = Trie::new();

        for word in dictionary {
            trie.insert(word);
        }

        let mut result = trie.search("s?a??");

        let toCompare = HashSet::from(["stare".to_string(), "start".to_string()]);
        assert_eq!(result, toCompare);
    }


}