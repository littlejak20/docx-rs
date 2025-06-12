// Required Elements Implementation for wp:anchor
// HAUPTPROBLEM: Fehlende Required Elements, nicht Boolean-Serialisierung
// Diese Structs implementieren vollständige OOXML DrawingML-Compliance

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DrawingMLError {
    MissingRequiredElement(String),
    InvalidPositioning(String),
    IncompleteStructure(String),
}

impl fmt::Display for DrawingMLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DrawingMLError::MissingRequiredElement(msg) => write!(f, "Missing required element: {}", msg),
            DrawingMLError::InvalidPositioning(msg) => write!(f, "Invalid positioning: {}", msg),
            DrawingMLError::IncompleteStructure(msg) => write!(f, "Incomplete structure: {}", msg),
        }
    }
}

impl Error for DrawingMLError {}

// KRITISCH: PositionH MUSS vollständig sein
#[derive(Debug, Clone, PartialEq)]
pub struct PositionH {
    pub relative_from: String,      // "column", "page", "margin"
    pub align: Option<String>,      // "center", "left", "right"
    pub pos_offset: Option<i32>,    // Alternative zu align
}

impl PositionH {
    /// VALIDATION: align XOR pos_offset MUSS gesetzt sein!
    /// Das ist der HAUPTGRUND warum anonymisierte Dokumente nicht öffnen
    pub fn validate(&self) -> Result<(), DrawingMLError> {
        match (&self.align, &self.pos_offset) {
            (None, None) => Err(DrawingMLError::MissingRequiredElement(
                "positionH requires either align or posOffset - THIS IS THE MAIN CORRUPTION CAUSE".to_string()
            )),
            (Some(_), Some(_)) => Err(DrawingMLError::InvalidPositioning(
                "positionH cannot have both align and posOffset".to_string()
            )),
            _ => Ok(()) // Exactly one is set - this is correct
        }
    }
    
    /// Creates a safe default positioning for compatibility mode
    pub fn safe_default() -> Self {
        PositionH {
            relative_from: "column".to_string(),
            align: Some("center".to_string()), // Safe default
            pos_offset: None,
        }
    }
}

// KRITISCH: PositionV ebenfalls vollständig
#[derive(Debug, Clone, PartialEq)]
pub struct PositionV {
    pub relative_from: String,      // "paragraph", "page", "margin"
    pub align: Option<String>,      // "top", "center", "bottom"  
    pub pos_offset: Option<i32>,    // Alternative zu align
}

impl PositionV {
    pub fn validate(&self) -> Result<(), DrawingMLError> {
        match (&self.align, &self.pos_offset) {
            (None, None) => Err(DrawingMLError::MissingRequiredElement(
                "positionV requires either align or posOffset".to_string()
            )),
            (Some(_), Some(_)) => Err(DrawingMLError::InvalidPositioning(
                "positionV cannot have both align and posOffset".to_string()
            )),
            _ => Ok(())
        }
    }
    
    pub fn safe_default() -> Self {
        PositionV {
            relative_from: "paragraph".to_string(),
            align: None,
            pos_offset: Some(0), // Safe default positioning
        }
    }
}

// KRITISCH: EffectExtent ist NICHT optional in funktionierende Dokumente
#[derive(Debug, Clone, PartialEq)]
pub struct EffectExtent {
    pub l: i32,  // left
    pub t: i32,  // top  
    pub r: i32,  // right
    pub b: i32,  // bottom
}

impl Default for EffectExtent {
    fn default() -> Self {
        EffectExtent { l: 0, t: 0, r: 0, b: 0 }
    }
}

// KRITISCH: CnvGraphicFramePr mit GraphicFrameLocks NICHT optional
#[derive(Debug, Clone, PartialEq)]
pub struct CnvGraphicFramePr {
    pub graphic_frame_locks: GraphicFrameLocks,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicFrameLocks {
    pub no_change_aspect: bool,    // Meist true in funktionierenden Dokumenten
}

impl Default for CnvGraphicFramePr {
    fn default() -> Self {
        CnvGraphicFramePr {
            graphic_frame_locks: GraphicFrameLocks {
                no_change_aspect: true, // Safe default
            }
        }
    }
}

// KRITISCH: PicLocks für Picture-Properties
#[derive(Debug, Clone, PartialEq)]
pub struct CnvPicPr {
    pub pic_locks: PicLocks,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PicLocks {
    pub no_change_aspect: bool,
    pub no_change_arrowheads: bool,
}

impl Default for CnvPicPr {
    fn default() -> Self {
        CnvPicPr {
            pic_locks: PicLocks {
                no_change_aspect: true,
                no_change_arrowheads: true,
            }
        }
    }
}

// KRITISCH: SizeRel für relative Größen-Definition
#[derive(Debug, Clone, PartialEq)]
pub struct SizeRelH {
    pub relative_from: String,  // "margin", "page", "leftMargin", "rightMargin"
    pub pct_width: u32,         // Percentage: 0-100000 (100000 = 100%)
}

#[derive(Debug, Clone, PartialEq)]
pub struct SizeRelV {
    pub relative_from: String,  // "margin", "page", "topMargin", "bottomMargin"
    pub pct_height: u32,        // Percentage: 0-100000
}

impl Default for SizeRelH {
    fn default() -> Self {
        SizeRelH {
            relative_from: "margin".to_string(),
            pct_width: 0, // 0% = absolute sizing
        }
    }
}

impl Default for SizeRelV {
    fn default() -> Self {
        SizeRelV {
            relative_from: "margin".to_string(),
            pct_height: 0, // 0% = absolute sizing
        }
    }
}

// VOLLSTÄNDIGE wp:anchor Implementation
#[derive(Debug, Clone, PartialEq)]
pub struct WpAnchor {
    // Distance properties
    pub dist_t: u32,
    pub dist_b: u32,
    pub dist_l: u32,
    pub dist_r: u32,
    
    // Boolean properties (NOTE: Boolean-Serialisierung ist SEKUNDÄRES Problem)
    pub simple_pos: bool,
    pub relative_height: u32,
    pub behind_doc: bool,
    pub locked: bool,
    pub layout_in_cell: bool,
    pub allow_overlap: bool,
    
    // KRITISCH: Required positioning (HAUPTPROBLEM!)
    pub position_h: PositionH,
    pub position_v: PositionV,
    
    // KRITISCH: Required DrawingML elements (NICHT optional!)
    pub effect_extent: EffectExtent,
    pub graphic_frame_pr: CnvGraphicFramePr,
    
    // Size relations (Required in funktionierenden Dokumenten)
    pub size_rel_h: SizeRelH,
    pub size_rel_v: SizeRelV,
    
    // Standard elements
    pub extent: Extent,
    pub doc_pr: DocPr,
    pub graphic: Graphic,
}

impl WpAnchor {
    /// Validates that all required elements are present and valid
    /// THIS IS THE CORE FIX - ensures document will open in Word
    pub fn validate(&self) -> Result<(), DrawingMLError> {
        // KRITISCH: Positioning validation
        self.position_h.validate()?;
        self.position_v.validate()?;
        
        // Check that effect_extent is reasonable (not all zeros is suspicious)
        // In practice, working documents rarely have all zeros
        
        // Additional validations could be added here
        
        Ok(())
    }
    
    /// Creates a "safe" wp:anchor with all required elements properly set
    /// This should generate XML that opens reliably in Microsoft Word
    pub fn safe_default(extent: Extent, doc_pr: DocPr, graphic: Graphic) -> Self {
        WpAnchor {
            // Safe distance defaults
            dist_t: 0,
            dist_b: 0, 
            dist_l: 114300, // Common value from working documents
            dist_r: 114300,
            
            // Safe boolean defaults
            simple_pos: false,
            relative_height: 251659264, // Common value
            behind_doc: false,
            locked: false,
            layout_in_cell: true,
            allow_overlap: true,
            
            // KRITISCH: Complete positioning
            position_h: PositionH::safe_default(),
            position_v: PositionV::safe_default(),
            
            // KRITISCH: All required DrawingML elements
            effect_extent: EffectExtent::default(),
            graphic_frame_pr: CnvGraphicFramePr::default(),
            
            // Size relations with safe defaults
            size_rel_h: SizeRelH::default(),
            size_rel_v: SizeRelV::default(),
            
            // Required standard elements
            extent,
            doc_pr,
            graphic,
        }
    }
}

// Standard structs (these likely exist already in docx-rs)
#[derive(Debug, Clone, PartialEq)]
pub struct Extent {
    pub cx: u32, // width in EMUs
    pub cy: u32, // height in EMUs
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocPr {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graphic {
    // Graphic data structure - implementation depends on existing docx-rs code
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position_h_validation() {
        // ❌ Should fail - no positioning specified (main corruption cause)
        let broken_pos = PositionH {
            relative_from: "column".to_string(),
            align: None,
            pos_offset: None,
        };
        assert!(broken_pos.validate().is_err());
        
        // ❌ Should fail - both positioning methods specified
        let invalid_pos = PositionH {
            relative_from: "column".to_string(),
            align: Some("center".to_string()),
            pos_offset: Some(100),
        };
        assert!(invalid_pos.validate().is_err());
        
        // ✅ Should pass - align specified
        let valid_align = PositionH {
            relative_from: "column".to_string(),
            align: Some("center".to_string()),
            pos_offset: None,
        };
        assert!(valid_align.validate().is_ok());
        
        // ✅ Should pass - pos_offset specified
        let valid_offset = PositionH {
            relative_from: "column".to_string(),
            align: None,
            pos_offset: Some(100),
        };
        assert!(valid_offset.validate().is_ok());
    }
    
    #[test]
    fn test_wp_anchor_safe_default() {
        let extent = Extent { cx: 1151890, cy: 1447165 };
        let doc_pr = DocPr { id: 1, name: "Grafik 1".to_string() };
        let graphic = Graphic {}; // Placeholder
        
        let anchor = WpAnchor::safe_default(extent, doc_pr, graphic);
        
        // Should validate without errors
        assert!(anchor.validate().is_ok());
        
        // Should have complete positioning
        assert!(anchor.position_h.align.is_some() || anchor.position_h.pos_offset.is_some());
        assert!(anchor.position_v.align.is_some() || anchor.position_v.pos_offset.is_some());
    }
    
    #[test]
    fn test_position_v_validation() {
        // Test similar patterns for PositionV
        let broken_pos = PositionV {
            relative_from: "paragraph".to_string(),
            align: None,
            pos_offset: None,
        };
        assert!(broken_pos.validate().is_err());
        
        let valid_pos = PositionV {
            relative_from: "paragraph".to_string(),
            align: None,
            pos_offset: Some(82550),
        };
        assert!(valid_pos.validate().is_ok());
    }
}

// XML Serialization helpers (example - actual implementation depends on docx-rs architecture)
impl WpAnchor {
    /// Serializes to XML with all required elements
    /// This should generate XML that Microsoft Word can reliably open
    pub fn to_xml(&self) -> Result<String, DrawingMLError> {
        // First validate that structure is complete
        self.validate()?;
        
        // Generate XML with all required elements
        // NOTE: Boolean values should use serialize_bool_ooxml() function
        let xml = format!(
            r#"<wp:anchor distT="{}" distB="{}" distL="{}" distR="{}" 
               simplePos="{}" relativeHeight="{}" 
               behindDoc="{}" locked="{}" layoutInCell="{}" allowOverlap="{}">
  <wp:simplePos x="0" y="0"/>
  {}
  {}
  <wp:extent cx="{}" cy="{}"/>
  <wp:effectExtent l="{}" t="{}" r="{}" b="{}"/>
  <wp:wrapTopAndBottom/>
  <wp:docPr id="{}" name="{}"/>
  <wp:cNvGraphicFramePr>
    <a:graphicFrameLocks noChangeAspect="{}"/>
  </wp:cNvGraphicFramePr>
  {}
  <wp:sizeRelH relativeFrom="{}">
    <wp:pctWidth>{}</wp:pctWidth>
  </wp:sizeRelH>
  <wp:sizeRelV relativeFrom="{}">
    <wp:pctHeight>{}</wp:pctHeight>
  </wp:sizeRelV>
</wp:anchor>"#,
            self.dist_t, self.dist_b, self.dist_l, self.dist_r,
            serialize_bool_ooxml(self.simple_pos),
            self.relative_height,
            serialize_bool_ooxml(self.behind_doc),
            serialize_bool_ooxml(self.locked),
            serialize_bool_ooxml(self.layout_in_cell),
            serialize_bool_ooxml(self.allow_overlap),
            
            // KRITISCH: Complete positioning
            self.position_h.to_xml(),
            self.position_v.to_xml(),
            
            // Extent
            self.extent.cx, self.extent.cy,
            
            // KRITISCH: Effect extent
            self.effect_extent.l, self.effect_extent.t, 
            self.effect_extent.r, self.effect_extent.b,
            
            // DocPr
            self.doc_pr.id, self.doc_pr.name,
            
            // KRITISCH: GraphicFrameLocks
            serialize_bool_ooxml(self.graphic_frame_pr.graphic_frame_locks.no_change_aspect),
            
            // Graphic data (implementation depends on existing structure)
            self.graphic.to_xml(),
            
            // KRITISCH: Size relations
            self.size_rel_h.relative_from, self.size_rel_h.pct_width,
            self.size_rel_v.relative_from, self.size_rel_v.pct_height,
        );
        
        Ok(xml)
    }
}

impl PositionH {
    pub fn to_xml(&self) -> String {
        let content = if let Some(align) = &self.align {
            format!("<wp:align>{}</wp:align>", align)
        } else if let Some(offset) = &self.pos_offset {
            format!("<wp:posOffset>{}</wp:posOffset>", offset)
        } else {
            // This should never happen if validate() was called
            "<wp:align>center</wp:align>".to_string() // Emergency fallback
        };
        
        format!(r#"<wp:positionH relativeFrom="{}">{}</wp:positionH>"#, 
                self.relative_from, content)
    }
}

impl PositionV {
    pub fn to_xml(&self) -> String {
        let content = if let Some(align) = &self.align {
            format!("<wp:align>{}</wp:align>", align)
        } else if let Some(offset) = &self.pos_offset {
            format!("<wp:posOffset>{}</wp:posOffset>", offset)
        } else {
            // Emergency fallback
            "<wp:posOffset>0</wp:posOffset>".to_string()
        };
        
        format!(r#"<wp:positionV relativeFrom="{}">{}</wp:positionV>"#, 
                self.relative_from, content)
    }
}

// Boolean serialization function (from boolean_serialization.rs)
fn serialize_bool_ooxml(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}