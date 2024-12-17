use zk_graph::builder::Builder;

fn main() {
    // Example 1: f(x) = x^2 + x + 5
    let mut builder = Builder::new();
    let x = builder.init();
    let x_squared = builder.mul(x, x);
    let five = builder.constant(5);
    let x_squared_plus_5 = builder.add(x_squared, five);
    let result = builder.add(x_squared_plus_5, x);

    builder.fill_nodes(&[(x, 3)]);
    println!("f(3) = {:?}", builder.get_value(&result)); // Should print 17 (3^2 + 3 + 5)
}
