use hex;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    leaves: Vec<String>,
    layers: Vec<Vec<String>>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<String>) -> Self {
        let hashed_leaves: Vec<String> = leaves.iter().map(|leaf| Self::hash(leaf)).collect();
        let mut tree = MerkleTree {
            leaves: hashed_leaves.clone(),
            layers: vec![hashed_leaves],
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
                    next_layer.push(Self::hash_pair(&chunk[0], &chunk[1]));
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
        hex::encode(hasher.finalize())
    }

    fn hash_pair(left: &str, right: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(hex::decode(left).unwrap());
        hasher.update(hex::decode(right).unwrap());
        hex::encode(hasher.finalize())
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
        println!("Initial leaf hash: {}", current_hash);

        for (sibling, is_left) in proof {
            println!("Sibling: {}, is_left: {}", sibling, is_left);
            current_hash = if *is_left {
                Self::hash_pair(sibling, &current_hash)
            } else {
                Self::hash_pair(&current_hash, sibling)
            };
            println!("Current hash: {}", current_hash);
        }

        println!("Final hash: {}", current_hash);
        println!("Root: {}", root);
        current_hash == root
    }

    pub fn show_layers(&self) {
        for (i, layer) in self.layers.iter().enumerate() {
            println!("Layer {}: {:?}", i, layer);
        }
    }
}
