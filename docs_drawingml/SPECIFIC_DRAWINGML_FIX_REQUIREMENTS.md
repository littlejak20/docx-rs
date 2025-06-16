# Specific DrawingML Fix Requirements for docx-rust

## CRITICAL: Previous Fix Attempts Have Failed

**Status:** Two fix attempts have been made, both have **COMPLETELY FAILED** to address the actual DrawingML corruption issue.

### Failed Fix Analysis:
- **ImageFix1:** Only changed Boolean serialization (already correct), no DrawingML fix
- **ImageFix2:** Only added more anonymization stars, no DrawingML fix  
- **Result:** Both files remain equally corrupted with the same DrawingML problem

## The Exact Problem That Must Be Fixed

### Current Corrupted DrawingML:
```xml
<wp:anchor distT="0" distB="0" distL="114300" distR="114300" simplePos="0" 
           relativeHeight="251659264" behindDoc="0" locked="0" 
           layoutInCell="1" allowOverlap="1">
    <wp:simplePos x="0" y="0"/>
    <wp:positionH relativeFrom="column"/>  ← EMPTY/INVALID
    <wp:positionV relativeFrom="paragraph">
        <wp:posOffset>82550</wp:posOffset>
    </wp:positionV>
    <!-- rest of anchor content -->
</wp:anchor>
```

**PROBLEM:** `<wp:positionH relativeFrom="column"/>` is a **self-closing empty element**, which violates OOXML specification.

## Required Fix (Choose ONE):

### Option 1: Add Required Elements to wp:positionH
```xml
<wp:positionH relativeFrom="column">
    <wp:align>left</wp:align>
</wp:positionH>
```

### Option 2: Add posOffset to wp:positionH
```xml
<wp:positionH relativeFrom="column">
    <wp:posOffset>0</wp:posOffset>
</wp:positionH>
```

### Option 3: Convert to Inline Element (Simplest)
```xml
<!-- Replace entire wp:anchor with wp:inline -->
<wp:inline distT="0" distB="0" distL="114300" distR="114300">
    <wp:extent cx="1151890" cy="1447165"/>
    <wp:docPr id="1" name="Grafik 1"/>
    <!-- rest of inline content - no positioning needed -->
</wp:inline>
```

## What docx-rust Must Implement

### 1. Detection Logic:
```rust
// Detect empty wp:positionH elements
if position_h.children().is_empty() && position_h.has_relative_from() {
    // CORRUPTION DETECTED
}
```

### 2. Fix Implementation:
```rust
// Option 1: Add align element
position_h.add_child(WpAlign::new("left"));

// Option 2: Add posOffset element  
position_h.add_child(WpPosOffset::new(0));

// Option 3: Convert to inline
let inline = WpInline::from_anchor(anchor);
drawing.replace_anchor_with_inline(inline);
```

### 3. Validation:
```rust
// Ensure fix is applied
assert!(!position_h.children().is_empty());
// OR
assert!(drawing.has_inline_instead_of_anchor());
```

## Test Requirements

### Before Fix (Current State):
- File size: ~260,584-260,602 bytes
- Contains: `<wp:positionH relativeFrom="column"/>`
- Status: CORRUPTED

### After Fix (Expected State):
- File size: Should increase (due to added elements)
- Contains: `<wp:positionH relativeFrom="column"><wp:align>left</wp:align></wp:positionH>`
- OR: `<wp:inline distT="0"...>` instead of `<wp:anchor>`
- Status: VALID OOXML

## Implementation Priority

1. **HIGHEST:** Fix the empty wp:positionH element
2. **MEDIUM:** Boolean serialization is already correct (don't change)
3. **LOW:** Text anonymization is irrelevant to corruption

## Success Criteria

✅ **Required:** wp:positionH must contain child elements  
✅ **Required:** Document must be valid OOXML  
✅ **Required:** File must open without corruption warnings in Microsoft Word  
❌ **NOT Required:** Changing Boolean serialization (already correct)  
❌ **NOT Required:** Additional anonymization  

## File Location for Testing

**Input File:** `docx_files/customer_documents/anonymisiert 16 O 138-23-anonymisiert.docx`  
**Expected Output:** A file where the DrawingML corruption is actually fixed

## Critical Note

**DO NOT REPEAT THE PREVIOUS MISTAKES:**
- Don't just change anonymization level
- Don't focus on Boolean serialization (already correct) 
- Don't ignore the wp:positionH element
- **ACTUALLY FIX THE DRAWINGML STRUCTURE**

This is the third attempt. The corruption MUST be fixed this time.