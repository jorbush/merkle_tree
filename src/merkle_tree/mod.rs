use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    layers: Vec<Vec<String>>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<String>) -> Self {
        let mut tree = MerkleTree {
            layers: vec![leaves.iter().map(|leaf| Self::hash(leaf)).collect()],
        };
        tree.build_tree();
        tree
    }

    fn build_tree(&mut self) {
        let mut current_layer = self.layers[0].clone();
        while current_layer.len() > 1 {
            let mut next_layer = Vec::new();
            for chunk in current_layer.chunks(2) {
                if chunk.len() == 2 {
                    next_layer.push(Self::hash(&format!("{}{}", &chunk[0], &chunk[1])));
                } else {
                    next_layer.push(chunk[0].clone());
                }
            }
            self.layers.push(next_layer.clone());
            current_layer = next_layer;
        }
    }

    fn hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn get_root(&self) -> String {
        self.layers.last().unwrap()[0].clone()
    }

    pub fn get_proof(&self, index: usize) -> Vec<(String, bool)> {
        let mut proof = Vec::new();
        let mut current_index = index;

        for layer in &self.layers[..self.layers.len() - 1] {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < layer.len() {
                proof.push((layer[sibling_index].clone(), current_index % 2 == 1));
            }

            current_index /= 2;
        }

        proof
    }

    pub fn verify_proof(root: &str, leaf: &str, proof: &[(String, bool)]) -> bool {
        let mut current_hash = Self::hash(leaf);
        for (sibling, is_left) in proof {
            current_hash = if *is_left {
                Self::hash(&format!("{}{}", &sibling, &current_hash))
            } else {
                Self::hash(&format!("{}{}", &current_hash, &sibling))
            };
        }
        current_hash == root
    }

    pub fn print_tree(&self) {
        if self.layers.is_empty() {
            println!("Empty tree");
            return;
        }
        println!();
        let total_width = 50;
        let root = &self.layers.last().unwrap()[0];
        println!("{:^width$}", root, width = total_width);

        for (i, layer) in self.layers.iter().rev().enumerate() {
            let node_width = total_width / layer.len();
            let padding = " ".repeat(node_width / 4);
            print!("{}", padding);
            for node in layer {
                print!("{:^width$}", &node[..8], width = node_width);
            }
            println!();

            if i < self.layers.len() - 1 {
                print!("{}", padding);
                for _ in layer {
                    print!("{:^width$}", "/\\", width = node_width);
                }
                println!();
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_merkle_tree() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        assert_eq!(tree.layers.len(), 3); // 4 leaves -> 2 nodes -> 1 root
    }

    #[test]
    fn test_get_root() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        let root = tree.get_root();
        assert_eq!(root.len(), 64); // SHA256 hash is 64 characters long in hex
    }

    #[test]
    fn test_get_proof() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        let proof = tree.get_proof(2); // Get proof for "c"
        assert_eq!(proof.len(), 2); // Should have 2 elements in the proof
    }

    #[test]
    fn test_verify_proof_valid() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        let root = tree.get_root();
        let proof = tree.get_proof(2); // Get proof for "c"
        let is_valid = MerkleTree::verify_proof(&root, "c", &proof);
        assert!(is_valid);
    }

    #[test]
    fn test_verify_proof_invalid() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        let root = tree.get_root();
        let proof = tree.get_proof(2); // Get proof for "c"
        let is_valid = MerkleTree::verify_proof(&root, "x", &proof); // "x" is not in the tree
        assert!(!is_valid);
    }

    #[test]
    fn test_tree_with_odd_number_of_leaves() {
        let leaves = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let tree = MerkleTree::new(leaves);
        assert_eq!(tree.layers.len(), 3); // 3 leaves -> 2 nodes -> 1 root
    }

    #[test]
    fn test_hash_function() {
        let hash = MerkleTree::hash("test");
        assert_eq!(hash.len(), 64); // SHA256 hash is 64 characters long in hex
        assert_eq!(
            hash,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn test_print_tree() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        tree.print_tree();
    }

    #[test]
    fn test_single_leaf() {
        let leaves = vec!["a".to_string()];
        let tree = MerkleTree::new(leaves);
        let root = tree.get_root();
        let proof = tree.get_proof(0); // Get proof for "a"
        let is_valid = MerkleTree::verify_proof(&root, "a", &proof);
        assert!(is_valid);
    }

    #[test]
    fn test_large_tree() {
        let leaves = (0..100)
            .map(|i| format!("leaf{}", i))
            .collect::<Vec<String>>();
        let tree = MerkleTree::new(leaves.clone());
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.get_proof(i);
            let is_valid = MerkleTree::verify_proof(&tree.get_root(), leaf, &proof);
            assert!(is_valid, "Failed for leaf {}", leaf);
        }
    }
}
