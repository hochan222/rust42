use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};

pub fn ft_ast_node_print(markdown: &str) -> String {
    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &ComrakOptions::default());

    build_ast_output(root, 0)
}

fn build_ast_output<'a>(node: &'a AstNode<'a>, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    let mut result = match &node.data.borrow().value {
        NodeValue::Text(text) => format!(
            "{}Text: {}\n",
            indent,
            String::from_utf8_lossy(text.as_bytes())
        ),
        NodeValue::Heading(heading) => format!("{}Heading: Level {}\n", indent, heading.level),
        NodeValue::Paragraph => format!("{}Paragraph\n", indent),
        NodeValue::Document => format!("{}Document\n", indent),
        _ => format!("{}Other: {:?}\n", indent, node.data.borrow().value),
    };

    for child in node.children() {
        result.push_str(&build_ast_output(child, depth + 1));
    }

    result
}

pub fn ft_custom_node(markdown: &str) -> String {
    // let arena = Arena::new();
    // let root = parse_document(&arena, markdown, &ComrakOptions::default());

    // 모든 헤딩 노드를 대체
    transform_headings_to_custom_text();

    String::new()
}

fn transform_headings_to_custom_text<'a>() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ft_test_ast_node_print() {
        let markdown = "# Hello, Rust!\n\nThis is a 42.";
        let expected_output = "Document\n  Heading: Level 1\n    Text: Hello, Rust!\n  Paragraph\n    Text: This is a 42.\n";

        assert_eq!(ft_ast_node_print(markdown), expected_output);
    }

    #[test]
    fn ft_test_custom_node() {
        let markdown = "# Hello, Rust!\n\nThis is a 42.";
        let expected_output = "### Hello, Rust!\n\nThis is a 42.";

        assert_eq!(ft_custom_node(markdown), expected_output);
    }
}
