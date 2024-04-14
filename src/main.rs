use products_in_range::BinaryTree;
fn main() {
    let prices = vec![1, 2, 4, 454 ,23 ,3 , 7];
    // let prices = vec![1, 2, 454 ,23];
    let mut priceTree = BinaryTree::init(None);
    for price in &prices {
        priceTree.add(price);
    }
    println!(
        "{:?}", priceTree
    )
}
