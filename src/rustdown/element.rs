// Note: this is not a good implementation.
// Should probably be a strategy pattern instead (a Trait and different implementations)

// A generic html element
// It may be a complex element with more elements or just an element containing a string
pub enum Element {
    Complex(ElementWithChildren),
    Text(TextElement),
}

impl Element {
    pub fn render(&self) -> String {
        match self {
            Element::Text(el) => format!("<{}>{}</{}>", el.tag, el.content, el.tag),
            // This one's kinda hard bc  we need to call render recursively
            Element::Complex(el) => {
                let mut buff = String::new();
                for ele in &el.content {
                    buff.push_str(ele.render().as_str());
                }
                format!("<{}>{}</{}>", el.tag, buff, el.tag)
            }
        }
    }

    pub fn add_child(&mut self, el: Element) {
        match self {
            Self::Complex(parent) => parent.add_child(el),
            Self::Text(_) => {}
        }
    }
}

pub struct TextElement {
    content: String,
    tag: String,
}

impl TextElement {
    pub fn new(tag: String, content: String) -> TextElement {
        TextElement { content, tag }
    }
}

pub struct ElementWithChildren {
    content: Vec<Element>,
    tag: String,
}

impl ElementWithChildren {
    pub fn new(tag: &str) -> ElementWithChildren {
        ElementWithChildren {
            tag: String::from(tag),
            content: Vec::new(),
        }
    }

    // Private function to add children
    fn add_child(&mut self, el: Element) {
        self.content.push(el);
    }
}
