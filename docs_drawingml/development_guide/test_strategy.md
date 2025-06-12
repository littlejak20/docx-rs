# Test Strategy - docx-rust DrawingML Improvements

## 🎯 **Testing Philosophy**

**Real-World First:** Every fix must be validated against actual problematic DOCX files, not just synthetic examples.

**Microsoft Word Compatibility:** The ultimate test is whether Microsoft Word can open the generated DOCX without errors.

## 📋 **Test Categories**

### **1. Unit Tests - Individual Components**

#### **1.1 Required Elements Validation Tests (CRITICAL)**
```rust
#[cfg(test)]
mod required_elements_tests {
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
    fn test_wp_anchor_validation() {
        // Create anchor with incomplete positioning
        let incomplete_anchor = WpAnchor {
            position_h: PositionH {
                relative_from: "column".to_string(),
                align: None,
                pos_offset: None, // ❌ MISSING REQUIRED ELEMENT
            },
            // ... other fields
        };
        
        // Should fail validation
        assert!(incomplete_anchor.validate().is_err());
        
        // Create anchor with complete positioning  
        let complete_anchor = WpAnchor {
            position_h: PositionH {
                relative_from: "column".to_string(),
                align: Some("center".to_string()), // ✅ REQUIRED ELEMENT PRESENT
                pos_offset: None,
            },
            // ... other fields
        };
        
        // Should pass validation
        assert!(complete_anchor.validate().is_ok());
    }
}
```

#### **1.2 Boolean Serialization Tests (SECONDARY)**
```rust
#[cfg(test)]
mod boolean_serialization_tests {
    use super::*;
    
    #[test]
    fn test_ooxml_boolean_standard() {
        // Core function test
        assert_eq!(serialize_bool_ooxml(true), "1");
        assert_eq!(serialize_bool_ooxml(false), "0");
        
        // Anti-regression - should NEVER return string booleans
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
        
        // Verify all boolean attributes use 0/1 format
        assert!(xml.contains(r#"behindDoc="0""#));
        assert!(xml.contains(r#"locked="1""#));
        assert!(xml.contains(r#"layoutInCell="0""#));
        assert!(xml.contains(r#"allowOverlap="1""#));
        assert!(xml.contains(r#"simplePos="0""#));
        
        // Critical: NO "true"/"false" strings should exist
        assert!(!xml.contains("\"true\""));
        assert!(!xml.contains("\"false\""));
    }
}
```

#### **1.3 Safe Mode Conversion Tests**
```rust
#[cfg(test)]
mod safe_mode_tests {
    use super::*;
    
    #[test]
    fn test_complex_anchor_detection() {
        let complex_anchor = WpAnchor {
            behind_doc: true,        // Complex z-ordering
            position_h: PositionH {
                align: Some("center".to_string()), // Complex positioning
                // ...
            },
            // ...
        };
        
        let drawing = Drawing::from_anchor(complex_anchor);
        assert!(drawing.is_complex_anchor(&drawing.anchor));
    }
    
    #[test]
    fn test_anchor_to_inline_conversion() {
        let complex_anchor = create_complex_test_anchor();
        let drawing = Drawing::from_anchor(complex_anchor);
        
        let inline = drawing.anchor_to_simple_inline(&drawing.anchor).unwrap();
        
        // Essential properties should be preserved
        assert_eq!(inline.extent, drawing.anchor.extent);
        assert_eq!(inline.doc_pr, drawing.anchor.doc_pr);
        assert_eq!(inline.graphic, drawing.anchor.graphic);
        
        // Should have safe inline-specific properties
        assert_eq!(inline.dist_t, 0);
        assert_eq!(inline.dist_b, 0);
    }
    
    #[test]
    fn test_sanitize_for_compatibility() {
        let mut drawing = create_drawing_with_complex_anchor();
        
        let warnings = drawing.sanitize_for_compatibility().unwrap();
        
        // Should have converted to inline
        match drawing.drawing_type {
            DrawingType::Inline(_) => {}, // Success
            _ => panic!("Should have converted complex anchor to inline"),
        }
        
        // Should have generated warning
        assert!(!warnings.is_empty());
        assert!(warnings[0].contains("Converting complex wp:anchor"));
    }
    
    #[test]
    fn test_simple_inline_unchanged() {
        let mut drawing = create_drawing_with_simple_inline();
        
        let warnings = drawing.sanitize_for_compatibility().unwrap();
        
        // Should remain inline
        match drawing.drawing_type {
            DrawingType::Inline(_) => {}, // Should stay inline
            _ => panic!("Simple inline should remain unchanged"),
        }
        
        // Should not generate warnings for already-safe content
        assert!(warnings.is_empty());
    }
}
```

### **2. Integration Tests - Document Level**

#### **2.1 Document Sanitization Tests**
```rust
#[cfg(test)]
mod document_integration_tests {
    use super::*;
    
    #[test]
    fn test_sanitize_all_drawings() {
        let mut doc = create_test_document_with_mixed_drawings();
        
        let warnings = doc.sanitize_all_drawings().unwrap();
        
        // Should have processed all drawings
        assert!(!warnings.is_empty());
        
        // Verify all complex anchors converted to inline
        for paragraph in &doc.document.body.paragraphs {
            for run in &paragraph.runs {
                if let Some(drawing) = &run.drawing {
                    match &drawing.drawing_type {
                        DrawingType::Inline(_) => {}, // Safe
                        DrawingType::Anchor(anchor) => {
                            // Only simple anchors should remain
                            assert!(!drawing.is_complex_anchor(anchor));
                        }
                    }
                }
            }
        }
    }
    
    #[test]
    fn test_save_compatible_api() {
        let doc = create_test_document_with_complex_drawings();
        let temp_path = "test_output.docx";
        
        let warnings = doc.save_compatible(temp_path).unwrap();
        
        // Should have generated warnings for conversions
        assert!(!warnings.is_empty());
        
        // File should exist and be readable
        assert!(std::path::Path::new(temp_path).exists());
        
        // Clean up
        std::fs::remove_file(temp_path).unwrap();
    }
}
```

### **3. Real-World Document Tests**

#### **3.1 Test with Reference XML Examples**
```rust
#[cfg(test)]
mod real_world_tests {
    use super::*;
    
    #[test]
    fn test_broken_drawingml_fix() {
        // Load the actual problematic XML from reference materials
        let broken_xml = include_str!("../reference_materials/xml_examples/broken_drawingml.xml");
        
        // Parse and process with our fixes
        let mut drawing = Drawing::from_xml(broken_xml).unwrap();
        let warnings = drawing.sanitize_for_compatibility().unwrap();
        
        // Should have detected and fixed problems
        assert!(!warnings.is_empty());
        
        // Generate fixed XML
        let fixed_xml = drawing.to_xml();
        
        // Should match our known-good reference
        let expected_xml = include_str!("../reference_materials/xml_examples/fixed_drawingml.xml");
        assert_xml_equivalent(fixed_xml, expected_xml);
    }
    
    #[test]
    fn test_working_drawingml_unchanged() {
        // Load working XML that should remain unchanged
        let working_xml = include_str!("../reference_materials/xml_examples/working_drawingml.xml");
        
        let mut drawing = Drawing::from_xml(working_xml).unwrap();
        let warnings = drawing.sanitize_for_compatibility().unwrap();
        
        // Should not need changes for already-working content
        assert!(warnings.is_empty());
        
        // Output should be identical to input
        let output_xml = drawing.to_xml();
        assert_xml_equivalent(output_xml, working_xml);
    }
}
```

#### **3.2 Full DOCX File Tests**
```rust
#[cfg(test)]
mod full_docx_tests {
    use super::*;
    
    #[test]
    fn test_problematic_customer_document() {
        // Test with actual customer document that failed
        let problematic_docx = load_test_docx("customer_problematic.docx");
        
        // Apply safe mode
        let warnings = problematic_docx.save_compatible("fixed_customer.docx").unwrap();
        
        // Should have made fixes
        assert!(!warnings.is_empty());
        
        // Result should be valid DOCX
        let fixed_docx = Docx::from_file("fixed_customer.docx").unwrap();
        assert!(fixed_docx.is_valid());
        
        // Clean up
        std::fs::remove_file("fixed_customer.docx").unwrap();
    }
}
```

### **4. Regression Tests**

#### **4.1 Existing Functionality Preservation**
```rust
#[cfg(test)]
mod regression_tests {
    use super::*;
    
    #[test]
    fn test_existing_docx_creation_unchanged() {
        // Test that existing API still works
        let doc = Docx::new();
        let paragraph = Paragraph::new().add_run(Run::new().add_text("Test"));
        let doc = doc.add_paragraph(paragraph);
        
        // Should save without issues
        doc.save("regression_test.docx").unwrap();
        
        // Should be readable
        let loaded = Docx::from_file("regression_test.docx").unwrap();
        assert_eq!(loaded.document.body.paragraphs.len(), 1);
        
        std::fs::remove_file("regression_test.docx").unwrap();
    }
    
    #[test]
    fn test_simple_images_unchanged() {
        // Images that already work should continue working
        let doc = create_document_with_simple_inline_image();
        
        doc.save("simple_image_test.docx").unwrap();
        
        // Should load without warnings
        let loaded = Docx::from_file("simple_image_test.docx").unwrap();
        let warnings = loaded.validate_drawing_compatibility();
        assert!(warnings.is_empty());
        
        std::fs::remove_file("simple_image_test.docx").unwrap();
    }
}
```

## 🧪 **Manual Testing Protocol**

### **Step 1: Generate Test DOCX**
```bash
# Run test that creates DOCX with fixed DrawingML
cargo test test_save_compatible_api -- --nocapture

# This should generate test file: test_output.docx
```

### **Step 2: Microsoft Word Compatibility Test**
1. **Open in Microsoft Word**
   - File should open without "corrupted document" error
   - Images should display correctly
   - No layout corruption

2. **Test Document Functions**
   - Can edit text around images
   - Can save document from Word
   - Can print/export to PDF

3. **Cross-Version Testing** (if available)
   - Word 2016, 2019, 2021, Office 365
   - Should work consistently across versions

### **Step 3: XML Validation**
```bash
# Extract and examine generated XML
unzip -o test_output.docx -d test_extracted/
cat test_extracted/word/document.xml | grep -A 10 -B 5 "wp:anchor\|wp:inline"

# Verify boolean format
grep -E 'behindDoc|locked|layoutInCell' test_extracted/word/document.xml
# Should show: behindDoc="0", NOT behindDoc="false"
```

## 📊 **Performance Testing**

### **Benchmark Critical Paths**
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_sanitization_performance() {
        let doc = create_large_document_with_many_images(100); // 100 images
        
        let start = Instant::now();
        let warnings = doc.sanitize_all_drawings().unwrap();
        let duration = start.elapsed();
        
        // Should complete in reasonable time
        assert!(duration.as_millis() < 1000); // Less than 1 second
        
        println!("Sanitized {} images in {:?}", warnings.len(), duration);
    }
    
    #[test]
    fn test_boolean_serialization_performance() {
        let start = Instant::now();
        
        for _ in 0..10000 {
            let _ = serialize_bool_ooxml(true);
            let _ = serialize_bool_ooxml(false);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
    }
}
```

## ✅ **Test Coverage Requirements**

### **Minimum Coverage Targets:**
- **Boolean Serialization:** 100% (critical path)
- **Safe Mode Conversion:** 95%
- **Document Sanitization:** 90%
- **XML Generation:** 85%

### **Critical Test Cases That Must Pass:**
1. ✅ Boolean attributes use `0/1` format (not `true/false`)
2. ✅ Complex `wp:anchor` converts to `wp:inline` in safe mode
3. ✅ Simple `wp:inline` remains unchanged
4. ✅ Real problematic document becomes openable in Word
5. ✅ Existing functionality unaffected (regression prevention)
6. ✅ Performance acceptable for typical document sizes

## 🚀 **Continuous Integration Setup**

```yaml
# .github/workflows/drawingml-tests.yml
name: DrawingML Compatibility Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run DrawingML tests
      run: |
        cargo test boolean_serialization
        cargo test safe_mode
        cargo test drawingml_integration
        cargo test regression
    
    - name: Validate XML output
      run: |
        cargo test test_save_compatible_api
        # Check generated XML format
        unzip -o test_output.docx -d test_extracted/
        grep -q 'behindDoc="[01]"' test_extracted/word/document.xml
        ! grep -q 'behindDoc="true\|false"' test_extracted/word/document.xml
```

This comprehensive test strategy ensures that all DrawingML fixes are thoroughly validated against real-world scenarios while preventing regressions.