use ft_comrak::ast_node;

fn main() {
    let markdown = "# Hello, Rust!\n\nThis is a 42.";
    println!("{}", ast_node::ft_custom_node(markdown));
}
