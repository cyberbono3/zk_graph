use zk_graph::builder::Builder;

fn main() {
    // Example 2: f(a) = (a+1) / 8
    let mut builder = Builder::new();
    let a = builder.init();
    let one = builder.constant(1);
    let b = builder.add(a, one);

    let c = builder.hint(move |values| values.get(&b.id()).map(|b_val| b_val / 8));

    let eight = builder.constant(8);
    let c_times_8 = builder.mul(c, eight);
    builder.assert_equal(b, c_times_8);

    builder.fill_nodes(&[(a, 7)]);
    println!("Constraints satisfied: {}", builder.check_constraints());
    println!("(7+1)/8 = {:?}", builder.get_value(&c)); // Should print 1
}
