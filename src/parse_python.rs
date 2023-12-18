use regex::Regex;

pub fn python_parse_comment(text: &str) -> String {
    let pattern = Regex::new(r"#.*").unwrap();
    let after = pattern.replace_all(&text, "<span class='comment'>$0</span>");
    after.to_string()
}

pub fn python_parse_reserved_word(text: &str) -> String {
    let pattern = Regex::new(r"\b(False|None|True|and|as|assert|async|await|break|continue|def|del|elif|else|except|False|finally|for|from|global|if|import|in|is|lambda|nonlocal|None|not|or|pass|raise|return|True|try|while|with|yield)\b").unwrap();
    let after = pattern.replace_all(&text, "<span class='reserved_word'>$0</span>");
    Regex::new(r"\b(class\s+)(\w+)<span class='comma'>:</span></pre>")
        .unwrap()
        .replace_all(
            &after,
            "<span class='reserved_word'>$1</span><span class='function'>$2</span><span class='comma'>:</span></pre>",
        )
        .to_string();

    todo!("bool or Noneのスタイルを分ける")
}

pub fn python_parse_operater(text: &str) -> String {
    let pattern = Regex::new(r"[^<>\n](\+|-|\*|/)").unwrap();
    let after = pattern.replace_all(&text, "<span class='reserved_word'>$1</span>");
    after.to_string()
}
pub fn python_parse_comma(text: &str) -> String {
    let pattern = Regex::new(r":").unwrap();
    let after = pattern.replace_all(&text, "<span class='comma'>$0</span>");
    after.to_string()
}

pub fn python_parse_string(text: &str) -> String {
    let pattern = Regex::new(r#"(?m)"[^"].*""#).unwrap();
    let after = pattern.replace_all(&text, "<span class='string'>$0</span>");
    Regex::new(r#"(?m)(?s)<pre>\s*""".*?"""</pre>"#)
        .unwrap()
        .replace_all(&after, "<span class='string'>$0</span>")
        .to_string()
    // after.to_string()
}

pub fn python_parse_function(text: &str) -> String {
    let pattern = Regex::new(r"(\w+)\(").unwrap();
    let after = pattern.replace_all(&text, "<span class='function'>$1</span>(");
    after.to_string()
}

// pub fn python_get_class_info(text: &str) {
//     let pattern = Regex::new(
//         r#"(?m)(?s)class\s(?P<className>.*?):.*?"{3}(?P<summary>.*?)\n(?P<discription>.*?)"{3}"#,
//     )
//     .unwrap();
//     for caps in pattern.captures_iter(&text) {
//         println!("className >>>{:?}", &caps[className]);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_parse_comment() {
        let text = "This is a comment # aaaa";
        let expected = "This is a comment <span class='comment'># aaaa</span>";
        assert_eq!(python_parse_comment(text), expected);
    }

    #[test]
    fn test_python_parse_reserved_word() {
        let text = "if num:";
        let expected = "<span class='reserved_word'>if</span> num:";
        assert_eq!(python_parse_reserved_word(text), expected);
    }

    #[test]
    fn test_python_parse_reserved_word2() {
        let text = "while True:";
        let expected =
            "<span class='reserved_word'>while</span> <span class='bool_or_None'>True</span:";
        assert_eq!(python_parse_reserved_word(text), expected);
    }

    #[test]
    fn test_python_parse_operater() {
        let text = "1 + 2 - 3 * 4 / 5";
        let expected = "1<span class='reserved_word'>+</span> 2<span class='reserved_word'>-</span> 3<span class='reserved_word'>*</span> 4<span class='reserved_word'>/</span> 5";
        assert_eq!(python_parse_operater(text), expected);
    }

    #[test]
    fn test_python_parse_comma() {
        let text = "if num:";
        let expected = "if num<span class='comma'>:</span>";
        assert_eq!(python_parse_comma(text), expected);
    }

    #[test]
    fn test_python_parse_string() {
        let text = r#"let string = "This is a string""#;
        let expected = r#"let string = <span class='string'>"This is a string"</span>"#;
        assert_eq!(python_parse_string(text), expected);
    }

    #[test]
    fn test_python_parse_function() {
        let text = "def add(a: i32, b: i32) -> i32 {";
        let expected = "def <span class='function'>add</span>(a: i32, b: i32) -> i32 {";
        assert_eq!(python_parse_function(text), expected);
    }
}
