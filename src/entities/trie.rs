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
    root: TrieNode,
    items: usize
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
            root: TrieNode::new(),
            items: 0
        }
    }


    // The insert method will add a word to the Trie by iterating through its characters and creating child nodes as necessary.
    pub fn insert(&mut self, word: &str) {
        if word.len() < MAX {
            panic!("Word is too short to be inserted in trie")
        }

        let mut root_node = &mut self.root;
        for key in word.chars() {
            root_node = root_node.nodes.entry(key).or_insert_with(TrieNode::new)
        }

        root_node.word = Some(word.to_string());
        root_node.eow = true;
        self.items += 1;
    }

    // The search method will check if a word exists in the Trie by traversing the nodes according to the word's characters or wildcards (unknown characters).
    pub fn search(&self, sequence: &str) -> HashSet<String> {
        let root_node = &self.root;
        let mut items: HashSet<String> = HashSet::new();
        // This function ensures traversing the Trie and passing available words in an array
        self.dfs_pattern_search(root_node, sequence, 0, &mut items);

        items
    }

    // Recursive DFS function to find words that match the current partial pattern.
    fn dfs_pattern_search(&self, mut root_node: &TrieNode, sequence: &str, idx: usize, mut items: &mut HashSet<String>) -> bool {
        if sequence.len() == idx {
            if root_node.eow {
                let word = root_node.word.clone().unwrap();
                // Collect all words that match the pattern
                items.insert(word);
            }
            return root_node.eow;
        }

        // Continue until eow or could not find the pattern occurs
        if sequence.chars().nth(idx).eq(&Some('?')) {
            // Handle the wildcards and traverse every child node of current root accordingly.
            for nextNode in root_node.nodes.values() {
                let result = self.dfs_pattern_search(&nextNode, sequence, idx + 1, items);
            }
        } else if let Some(nextNode) = root_node.nodes.get(&sequence.chars().nth(idx).unwrap()) {
            // If the node is not a wildcards follow the child node if exists
            return self.dfs_pattern_search(nextNode, sequence, idx + 1, items)
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