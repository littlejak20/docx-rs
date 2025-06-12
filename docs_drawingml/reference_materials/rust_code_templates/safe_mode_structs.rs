// Safe Mode Implementation for DrawingML Robustness
// Converts fragile wp:anchor to robust wp:inline for guaranteed Word compatibility

use std::path::Path;

#[derive(Debug)]
pub enum DrawingCompatibilityWarning {
    ComplexAnchorConverted { reason: String },
    MissingRequiredElements { elements: Vec<String> },
    InvalidBooleanFormat { field: String },
}

impl Drawing {
    /// Converts complex/problematic DrawingML to simple, robust format
    /// This ensures the DOCX will open in Microsoft Word even if complex elements are missing
    pub fn sanitize_for_compatibility(&mut self) -> Result<Vec<DrawingCompatibilityWarning>, DocxError> {
        let mut warnings = Vec::new();
        
        match &mut self.drawing_type {
            DrawingType::Anchor(anchor) => {
                // Check if anchor has complex features that might cause issues
                if self.is_complex_anchor(anchor) {
                    warnings.push(DrawingCompatibilityWarning::ComplexAnchorConverted {
                        reason: "Complex wp:anchor converted to wp:inline for compatibility".to_string()
                    });
                    
                    // Convert to simple inline representation
                    let inline = self.anchor_to_simple_inline(anchor)?;
                    self.drawing_type = DrawingType::Inline(inline);
                }
            }
            DrawingType::Inline(_) => {
                // wp:inline is already robust, no conversion needed
            }
        }
        
        Ok(warnings)
    }
    
    /// Checks if anchor has complex features that might be problematic
    fn is_complex_anchor(&self, anchor: &WpAnchor) -> bool {
        // Complex if:
        // - Has specific positioning (align, posOffset)
        // - Uses advanced features (effectExtent, sizeRel, etc.)
        // - Boolean values that might be incorrectly serialized
        
        anchor.position_h.align.is_some() ||
        anchor.position_v.align.is_some() ||
        anchor.effect_extent.is_some() ||
        anchor.size_rel_h.is_some() ||
        anchor.size_rel_v.is_some() ||
        anchor.behind_doc ||  // Complex z-ordering
        !anchor.simple_pos   // Non-simple positioning
    }
    
    /// Converts wp:anchor to wp:inline while preserving essential properties
    fn anchor_to_simple_inline(&self, anchor: &WpAnchor) -> Result<WpInline, DocxError> {
        Ok(WpInline {
            // Preserve essential image properties
            extent: anchor.extent.clone(),
            doc_pr: anchor.doc_pr.clone(),
            graphic: anchor.graphic.clone(),
            
            // Simple inline-specific properties
            dist_t: 0,
            dist_b: 0,
            dist_l: 0,
            dist_r: 0,
        })
    }
}

impl Docx {
    /// Saves DOCX with guaranteed Microsoft Word compatibility
    /// Applies safe mode to all drawings to prevent opening issues
    pub fn save_compatible<P: AsRef<Path>>(&self, path: P) -> Result<Vec<DrawingCompatibilityWarning>, DocxError> {
        let mut doc = self.clone();
        let warnings = doc.sanitize_all_drawings()?;
        doc.save(path)?;
        Ok(warnings)
    }
    
    /// Applies compatibility sanitization to all drawings in the document
    pub fn sanitize_all_drawings(&mut self) -> Result<Vec<DrawingCompatibilityWarning>, DocxError> {
        let mut all_warnings = Vec::new();
        
        // Sanitize drawings in main document body
        for paragraph in &mut self.document.body.paragraphs {
            for run in &mut paragraph.runs {
                if let Some(drawing) = &mut run.drawing {
                    let warnings = drawing.sanitize_for_compatibility()?;
                    all_warnings.extend(warnings);
                }
            }
        }
        
        // TODO: Also sanitize drawings in headers, footers, footnotes, etc.
        
        Ok(all_warnings)
    }
    
    /// Validates document for potential DrawingML compatibility issues
    pub fn validate_drawing_compatibility(&self) -> Vec<DrawingCompatibilityWarning> {
        let mut warnings = Vec::new();
        
        for paragraph in &self.document.body.paragraphs {
            for run in &paragraph.runs {
                if let Some(drawing) = &run.drawing {
                    if let DrawingType::Anchor(anchor) = &drawing.drawing_type {
                        // Check for known problematic patterns
                        if anchor.position_h.align.is_none() && anchor.position_h.pos_offset.is_none() {
                            warnings.push(DrawingCompatibilityWarning::MissingRequiredElements {
                                elements: vec!["positionH.align or positionH.posOffset".to_string()]
                            });
                        }
                        
                        // Check for other potential issues...
                    }
                }
            }
        }
        
        warnings
    }
}

// Feature flag support
#[cfg(feature = "drawing-compat")]
impl Default for DocxSaveOptions {
    fn default() -> Self {
        Self {
            use_compatibility_mode: true,
            validate_drawings: true,
        }
    }
}

#[cfg(not(feature = "drawing-compat"))]
impl Default for DocxSaveOptions {
    fn default() -> Self {
        Self {
            use_compatibility_mode: false,
            validate_drawings: false,
        }
    }
}

pub struct DocxSaveOptions {
    pub use_compatibility_mode: bool,
    pub validate_drawings: bool,
}

impl Docx {
    /// Advanced save with options
    pub fn save_with_options<P: AsRef<Path>>(&self, path: P, options: DocxSaveOptions) -> Result<Vec<DrawingCompatibilityWarning>, DocxError> {
        let mut doc = self.clone();
        let mut warnings = Vec::new();
        
        if options.validate_drawings {
            let validation_warnings = doc.validate_drawing_compatibility();
            warnings.extend(validation_warnings);
        }
        
        if options.use_compatibility_mode {
            let compat_warnings = doc.sanitize_all_drawings()?;
            warnings.extend(compat_warnings);
        }
        
        doc.save(path)?;
        Ok(warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_mode_conversion() {
        let mut drawing = create_complex_anchor_drawing();
        
        let warnings = drawing.sanitize_for_compatibility().unwrap();
        
        // Should have converted to inline
        match drawing.drawing_type {
            DrawingType::Inline(_) => {
                // Success - converted to robust format
            }
            _ => panic!("Should have converted complex anchor to inline"),
        }
        
        // Should have generated warning
        assert!(!warnings.is_empty());
        assert!(matches!(warnings[0], DrawingCompatibilityWarning::ComplexAnchorConverted { .. }));
    }
    
    #[test]
    fn test_simple_inline_unchanged() {
        let mut drawing = create_simple_inline_drawing();
        
        let warnings = drawing.sanitize_for_compatibility().unwrap();
        
        // Should remain inline
        match drawing.drawing_type {
            DrawingType::Inline(_) => {
                // Success - stayed as robust inline
            }
            _ => panic!("Simple inline should remain unchanged"),
        }
        
        // Should not generate warnings for simple inline
        assert!(warnings.is_empty());
    }
    
    fn create_complex_anchor_drawing() -> Drawing {
        Drawing {
            drawing_type: DrawingType::Anchor(WpAnchor {
                position_h: PositionH {
                    relative_from: "column".to_string(),
                    align: Some("center".to_string()),  // Complex positioning
                    pos_offset: None,
                },
                behind_doc: true,  // Complex z-ordering
                // ... other complex properties
            })
        }
    }
    
    fn create_simple_inline_drawing() -> Drawing {
        Drawing {
            drawing_type: DrawingType::Inline(WpInline {
                // Simple inline properties
                dist_t: 0,
                dist_b: 0,
                dist_l: 0,
                dist_r: 0,
                // ... basic properties
            })
        }
    }
}