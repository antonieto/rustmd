pub struct MarkdownElement {
    content: Vec<Box<MarkdownElement>>,
    tag: String,
}

impl MarkdownElement {
    pub fn new(tag: &String) -> MarkdownElement {
        MarkdownElement {
            tag: tag.clone(),
            content: Vec::new(),
        }
    }

    pub fn to_html(&self) -> String {
        // 1. Get children string

        let mut children_content = String::new();

        for child in self.content.iter() {
            let reference = child.as_ref();

            children_content.push_str(&reference.to_html())
        }

        String::from(format!("<{}>{}</{}>", self.tag, children_content, self.tag))
    }
}
