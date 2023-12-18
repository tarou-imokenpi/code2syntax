use regex::Regex;

/// parse markdown comment
pub fn markdown_parse_comment(text: &str) -> String {
    let pattern = Regex::new(r".*<!--\s?([\s\S]*?)\s?-->").unwrap();
    let after = pattern.replace_all(&text, "<span>$0</span>");
    after.to_string()
}

#[cfg(test)]
mod md_tests {
    use super::*;
    #[test]
    fn parse_comment() {
        let before = "<!-- markdown example -->";
        let after = markdown_parse_comment(before);
        assert_eq!(after.as_str(), "<span><!-- markdown example --></span>");
    }
    #[test]
    fn parse_comment2() {
        let before = "<!-- markdown example -->\n";
        let after = markdown_parse_comment(before);
        assert_eq!(after.as_str(), "<span><!-- markdown example --></span>\n");
    }
    #[test]
    fn parse_comment3() {
        let before = "<!-- markdown example -->\n<!-- markdown example2 -->";
        let after = markdown_parse_comment(before);
        assert_eq!(
            after.as_str(),
            "<span><!-- markdown example --></span>\n<span><!-- markdown example2 --></span>"
        );
    }
    #[test]
    fn parse_comment4() {
        let before = "<!-- markdown example -->\n<!-- markdown example2 -->\n";
        let after = markdown_parse_comment(before);
        assert_eq!(
            after.as_str(),
            "<span><!-- markdown example --></span>\n<span><!-- markdown example2 --></span>\n"
        );
    }
    #[test]
    fn parse_comment5() {
        let before = "<!--markdown example-->\n";
        let after = markdown_parse_comment(before);
        assert_eq!(after.as_str(), "<span><!--markdown example--></span>\n");
    }
}
