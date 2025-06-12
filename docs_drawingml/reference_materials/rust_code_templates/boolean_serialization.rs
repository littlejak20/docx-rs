// Boolean Serialization Fix for OOXML DrawingML
// Problem: docx-rs uses "true"/"false" but OOXML requires "0"/"1"

/// Serializes boolean values according to OOXML standard
/// OOXML expects "1" for true and "0" for false, NOT "true"/"false"
fn serialize_bool_ooxml(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

/// Alternative implementation with explicit matching
fn serialize_bool_ooxml_explicit(value: bool) -> &'static str {
    match value {
        true => "1",
        false => "0",
    }
}

// Example usage in DrawingML anchor serialization:
impl XmlSerialize for WpAnchor {
    fn to_xml(&self) -> String {
        format!(
            r#"<wp:anchor distT="{}" distB="{}" distL="{}" distR="{}" 
               simplePos="{}" relativeHeight="{}" 
               behindDoc="{}" locked="{}" layoutInCell="{}" allowOverlap="{}">"#,
            self.dist_t, 
            self.dist_b, 
            self.dist_l, 
            self.dist_r,
            serialize_bool_ooxml(self.simple_pos),
            self.relative_height,
            serialize_bool_ooxml(self.behind_doc),      // ✅ "0" or "1"
            serialize_bool_ooxml(self.locked),          // ✅ "0" or "1"  
            serialize_bool_ooxml(self.layout_in_cell),  // ✅ "0" or "1"
            serialize_bool_ooxml(self.allow_overlap)    // ✅ "0" or "1"
        )
    }
}

// Test cases to ensure correct serialization
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_serialization_ooxml_standard() {
        assert_eq!(serialize_bool_ooxml(true), "1");
        assert_eq!(serialize_bool_ooxml(false), "0");
        
        // Should NEVER return "true" or "false"
        assert_ne!(serialize_bool_ooxml(true), "true");
        assert_ne!(serialize_bool_ooxml(false), "false");
    }
    
    #[test]
    fn test_anchor_boolean_attributes() {
        let anchor = WpAnchor {
            behind_doc: false,
            locked: true,
            layout_in_cell: false,
            allow_overlap: true,
            simple_pos: false,
            // ... other fields
        };
        
        let xml = anchor.to_xml();
        
        // Ensure all boolean attributes use 0/1 format
        assert!(xml.contains(r#"behindDoc="0""#));
        assert!(xml.contains(r#"locked="1""#));
        assert!(xml.contains(r#"layoutInCell="0""#));
        assert!(xml.contains(r#"allowOverlap="1""#));
        assert!(xml.contains(r#"simplePos="0""#));
        
        // Ensure NO "true"/"false" strings exist
        assert!(!xml.contains("true"));
        assert!(!xml.contains("false"));
    }
}

// Additional boolean attributes that may need fixing:
// - <a:graphicFrameLocks noChangeAspect="1"/> 
// - <a:picLocks noChangeAspect="1" noChangeArrowheads="1"/>
// - Various drawing properties that use boolean values

/// Helper macro for consistent boolean serialization across DrawingML
macro_rules! serialize_bool_attr {
    ($name:expr, $value:expr) => {
        format!(r#"{}="{}""#, $name, serialize_bool_ooxml($value))
    };
}

// Usage example:
// let attr = serialize_bool_attr!("locked", self.locked);
// Result: locked="0" or locked="1"