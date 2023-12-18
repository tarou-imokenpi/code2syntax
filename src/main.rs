mod parse_markdown;
mod parse_python;
use code_to_html::*;
use parse_python::*;

fn main() {
    let before = add_pre_to_line(&read_file(r"kari.py"));
    let mut after = String::new();

    after.push_str("<html>\n");
    after.push_str("<body>\n");
    after.push_str(&add_date());
    after.push_str("<div class='code_block'>\n");

    after.push_str(&python_hightlight(before));

    // add style sheet
    after.push_str("\n");
    after.push_str("</div>\n");
    after.push_str("<style>\n");
    after.push_str(&read_file(r"src\style.css"));
    after.push_str("\n</style>\n");

    // close html
    after.push_str("</body>\n");
    after.push_str("</html>\n");
    save_html(&after, "./tmp/kari.html");
    html_to_pdf(r"full path");
}

fn python_hightlight(before: String) -> String {
    let mut parse_after = before;
    parse_after = python_parse_string(&parse_after);
    parse_after = python_parse_operater(&parse_after);
    parse_after = python_parse_comma(&parse_after);
    parse_after = python_parse_comment(&parse_after);
    parse_after = python_parse_reserved_word(&parse_after);
    parse_after = python_parse_function(&parse_after);
    parse_after
}
