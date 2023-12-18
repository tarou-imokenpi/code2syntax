mod parse_markdown;
mod parse_python;
use code_to_html::*;

fn main() {
    let before = add_pre_to_line(&read_file(r"full"));
    let mut after = String::new();

    after.push_str("<html>\n");
    after.push_str("<body>\n");
    after.push_str(&add_date());

    after.push_str("<div class='code_block'>\n");
    after.push_str(&python_hightlight(before));

    after.push_str("\n");
    after.push_str("</div>\n");

    // add css
    after.push_str("<style>\n");
    after.push_str(&read_file(r"full"));
    after.push_str("\n</style>\n");

    // close html
    after.push_str("</body>\n");
    after.push_str("</html>\n");
    save_html(&after, r"full");

    html_to_pdf(r"full");
}
