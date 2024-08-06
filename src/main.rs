mod merkle_tree;

use merkle_tree::MerkleTree;

fn main() {
    let leaves = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];

    let mut tree = MerkleTree::new(leaves);
    println!("Merkle Root: {}", tree.get_root());

    MerkleTree::print_tree(&tree);

    let index = 2;
    let proof = tree.get_proof(index);
    println!("Proof for leaf at index {}: {:?}", index, proof);

    let leaf = "c";
    let root = tree.get_root();
    let is_valid = MerkleTree::verify_proof(&root, leaf, &proof);
    println!("Proof verification: {}", is_valid);

    let new_leaf = "e";
    tree.add_leaf(new_leaf.to_string());
    let new_root = tree.get_root();
    let new_proof = tree.get_proof(4);
    let new_is_valid = MerkleTree::verify_proof(&new_root, new_leaf, &new_proof);
    println!("Proof verification for new leaf: {}", new_is_valid);
}
