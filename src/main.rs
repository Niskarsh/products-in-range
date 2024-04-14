use products_in_range::BinaryTree;
fn main() {
    let prices = vec![1, 2, 4, 454 ,23 ,3 , 7];
    let priceRange = (20, 500);
    // println!("{}", priceRange.0);
    // let prices = vec![1, 2, 454 ,23];
    let mut priceTree = BinaryTree::init(None);
    for price in &prices {
        priceTree.add(price);
    }
    let mut priceIn: Vec<i32> = vec![];
    // Assuming binary tree is not empty
    priceTree.pricesInRange(&priceRange.0, &priceRange.1, &mut priceIn);
    println!(
        "{:?}", priceIn
    )
}
