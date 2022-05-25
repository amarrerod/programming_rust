use std::io::{self, Write};

pub struct HtmlDocument {
    pub name: String,
}

impl HtmlDocument {
    pub fn new(name: String) -> HtmlDocument {
        HtmlDocument { name }
    }
}

pub trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        println!("Document know as {}", html.name);
        let content = format!("The document is named as <p>{}</p>", html.name);

        self.write(content.as_bytes())?;
        self.flush()?;
        Ok(())
    }
}
