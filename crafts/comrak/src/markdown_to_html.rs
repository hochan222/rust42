use comrak::{markdown_to_html, ComrakOptions};

pub fn ft_m2h_hello(markdown: &str) -> String {
    let html = markdown_to_html(markdown, &ComrakOptions::default());

    return html;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_rust() {
        let markdown = "# Hello, Rust!";
        let expected_output = "<h1>Hello, Rust!</h1>\n";

        assert_eq!(ft_m2h_hello(markdown), expected_output);
    }
}
