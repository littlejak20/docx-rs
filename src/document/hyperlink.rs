#![allow(unused_must_use)]

use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, document::bidir::BidirectionalEmbedding, document::Run, document::Text};

/// The root element of a hyperlink within the paragraph
/// 
/// Supports multiple runs to handle complex hyperlinks like table of contents entries.
/// Each run can have different formatting while being part of the same hyperlink.
/// 
/// # Examples
/// 
/// Creating a simple hyperlink:
/// ```rust
/// use docx_rust::document::{Hyperlink, Run};
/// 
/// let hyperlink = Hyperlink::default()
///     .id("link1")
///     .push_run(Run::default().push_text("Click here"));
/// ```
/// 
/// Creating a complex table of contents hyperlink:
/// ```rust
/// use docx_rust::document::{Hyperlink, Run};
/// 
/// let toc_hyperlink = Hyperlink::default()
///     .id("_Toc123456789")
///     .push_run(Run::default().push_text("Chapter 1"))
///     .push_run(Run::default().push_text("    "))  // Tab spacing
///     .push_run(Run::default().push_text("Introduction"))
///     .push_run(Run::default().push_text("..."))   // Dots
///     .push_run(Run::default().push_text("5"));    // Page number
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink<'a> {
    /// Specifies the ID of the relationship in the relationships part for an external link.
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the name of a bookmark within the document.
    #[xml(attr = "w:anchor")]
    pub anchor: Option<Cow<'a, str>>,
    #[xml(child = "w:r")]
    /// Link content - supports multiple runs for complex hyperlinks
    pub content: Vec<Run<'a>>,
    #[xml(child = "w:dir")]
    // Link can contain a bi-directional embedding layer
    pub bidirectional_embedding: Option<BidirectionalEmbedding<'a>>,
}

impl<'a> Hyperlink<'a> {
    __setter!(id: Option<Cow<'a, str>>);
    __setter!(anchor: Option<Cow<'a, str>>);
    // Note: content setter replaced with push_run and add_text methods

    /// Add a Run element to the hyperlink content
    pub fn push_run(mut self, run: Run<'a>) -> Self {
        self.content.push(run);
        self
    }
    
    /// Add text as a new Run to the hyperlink content
    pub fn add_text<T: Into<Text<'a>>>(mut self, text: T) -> Self {
        self.content.push(Run::default().push_text(text));
        self
    }
    
    /// Get the first run (for backward compatibility)
    pub fn first_run(&self) -> Option<&Run<'a>> {
        self.content.first()
    }
    
    /// Get mutable reference to the first run (for backward compatibility)
    pub fn first_run_mut(&mut self) -> Option<&mut Run<'a>> {
        self.content.first_mut()
    }
    
    /// Create hyperlink from a single run (migration helper)
    pub fn from_single_run(run: Run<'a>) -> Self {
        Self {
            content: vec![run],
            ..Default::default()
        }
    }
    
    /// Add text replacement support for all runs in the hyperlink
    pub fn replace_text<'b, I, T, S>(&mut self, dic: T) -> crate::DocxResult<()>
    where
        S: AsRef<str> + 'b,
        T: IntoIterator<Item = I> + Copy,
        I: std::borrow::Borrow<(S, S)>,
    {
        for run in self.content.iter_mut() {
            run.replace_text(dic)?;
        }
        Ok(())
    }
    
    /// Simple text replacement across all runs
    pub fn replace_text_simple<S>(&mut self, old: S, new: S)
    where
        S: AsRef<str>,
    {
        for run in self.content.iter_mut() {
            run.replace_text_simple(&old, &new);
        }
    }

    pub fn text(&self) -> String {
        self.iter_text()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn iter_text(&self) -> Box<dyn Iterator<Item = &Cow<'a, str>> + '_> {
        Box::new(
            self.content.iter().flat_map(|run| run.iter_text()).chain(
                self.bidirectional_embedding
                    .iter()
                    .flat_map(|bidi| bidi.iter_text()),
            ),
        )
    }

    pub fn iter_text_mut(&mut self) -> Box<dyn Iterator<Item = &mut Cow<'a, str>> + '_> {
        Box::new(
            self.content
                .iter_mut()
                .flat_map(|run| run.iter_text_mut())
                .chain(
                    self.bidirectional_embedding
                        .iter_mut()
                        .flat_map(|bidi| bidi.iter_text_mut()),
                ),
        )
    }
}

__xml_test_suites!(
    Hyperlink,
    Hyperlink::default(),
    r#"<w:hyperlink/>"#,
    Hyperlink::default().id("id"),
    r#"<w:hyperlink r:id="id"/>"#,
    Hyperlink::default().anchor("anchor"),
    r#"<w:hyperlink w:anchor="anchor"/>"#,
    Hyperlink::default().push_run(Run::default()),
    r#"<w:hyperlink><w:r/></w:hyperlink>"#,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Run;

    #[test]
    fn test_multiple_runs() {
        let mut hyperlink = Hyperlink::default()
            .push_run(Run::default().push_text("First "))
            .push_run(Run::default().push_text("Second"));
        
        assert_eq!(hyperlink.content.len(), 2);
        assert_eq!(hyperlink.text(), "First Second");
    }

    #[test]
    fn test_add_text_method() {
        let hyperlink = Hyperlink::default()
            .add_text("Hello ")
            .add_text("World");
        
        assert_eq!(hyperlink.content.len(), 2);
        assert_eq!(hyperlink.text(), "Hello World");
    }

    #[test]
    fn test_text_replacement_across_runs() {
        let mut hyperlink = Hyperlink::default()
            .add_text("Old text in first run")
            .add_text("Old text in second run");
        
        hyperlink.replace_text_simple("Old text", "New text");
        
        assert_eq!(hyperlink.text(), "New text in first runNew text in second run");
    }

    #[test]
    fn test_text_replacement_multiple_replacements() {
        let mut hyperlink = Hyperlink::default()
            .add_text("Hello World")
            .add_text("Goodbye Moon");
        
        let replacements = [("Hello", "Hi"), ("Goodbye", "Bye")];
        hyperlink.replace_text(&replacements).unwrap();
        
        assert_eq!(hyperlink.text(), "Hi WorldBye Moon");
    }

    #[test]
    fn test_backward_compatibility_first_run() {
        let hyperlink = Hyperlink::default()
            .add_text("First")
            .add_text("Second");
        
        assert_eq!(hyperlink.first_run().unwrap().text(), "First");
    }

    #[test]
    fn test_from_single_run() {
        let run = Run::default().push_text("Single run");
        let hyperlink = Hyperlink::from_single_run(run);
        
        assert_eq!(hyperlink.content.len(), 1);
        assert_eq!(hyperlink.text(), "Single run");
    }

    #[test]
    fn test_iter_text_multiple_runs() {
        let hyperlink = Hyperlink::default()
            .add_text("First")
            .add_text(" Second");
        
        let texts: Vec<String> = hyperlink.iter_text()
            .map(|cow| cow.to_string())
            .collect();
        
        assert_eq!(texts, vec!["First", " Second"]);
    }

    #[test]
    fn test_complex_table_of_contents_scenario() {
        // Simulate a table of contents hyperlink with multiple formatting runs
        let mut hyperlink = Hyperlink::default()
            .id("_Toc123456789")
            .push_run(Run::default().push_text("Chapter 1"))
            .push_run(Run::default().push_text("    "))  // Tab spacing
            .push_run(Run::default().push_text("Introduction"));
        
        assert_eq!(hyperlink.content.len(), 3);
        assert_eq!(hyperlink.text(), "Chapter 1    Introduction");
        assert_eq!(hyperlink.id.as_ref().unwrap(), "_Toc123456789");
        
        // Test text replacement works across all runs
        hyperlink.replace_text_simple("Chapter 1", "Section 1");
        assert_eq!(hyperlink.text(), "Section 1    Introduction");
    }
}
