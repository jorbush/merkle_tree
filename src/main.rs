mod merkle_tree;

use merkle_tree::MerkleTree;

fn main() {
    let leaves = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];

    let tree = MerkleTree::new(leaves);
    println!("Merkle Root: {}", tree.get_root());

    MerkleTree::show_layers(&tree);

    let index = 2;
    let proof = tree.get_proof(index);
    println!("Proof for leaf at index {}: {:?}", index, proof);

    let leaf = "c";
    let root = tree.get_root();
    let is_valid = MerkleTree::verify_proof(&root, leaf, &proof);
    println!("Proof verification: {}", is_valid);
}
