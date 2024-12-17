
use zk_graph::builder::Builder;
// Example 3: f(x) = sqrt(x+7)
//
// Assume that x+7 is a perfect square (so x = 2 or 9, etc.).
fn main() {
    let mut builder = Builder::new();
    let x = builder.init();
    let seven = builder.constant(7);
    let x_plus_seven = builder.add(x, seven);

    let sqrt_x_plus_7 = builder.hint(move |values| values.get(&x_plus_seven.id()).map(|val| Builder::square_root(*val)));
    let computed_sq = builder.mul(sqrt_x_plus_7, sqrt_x_plus_7);
    builder.assert_equal(computed_sq, x_plus_seven);

    builder.fill_nodes(&[(x, 2)]);
    println!("Constraints satisfied: {}",builder.check_constraints());
}