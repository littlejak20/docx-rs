#![allow(unused_must_use)]

use std::borrow::Cow;

use derive_more::From;
use hard_xml::{XmlRead, XmlWrite};

use crate::{__define_enum, __string_enum};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:drawing")]
pub struct Drawing<'a> {
    /// comment
    #[xml(child = "wp:anchor")]
    pub anchor: Option<Anchor<'a>>,
    #[xml(child = "wp:inline")]
    pub inline: Option<Inline<'a>>,
}

impl<'a> Drawing<'a> {
    /// Converts complex/problematic DrawingML to simple, robust format for maximum Word compatibility
    pub fn sanitize_for_compatibility(&mut self) -> crate::DocxResult<()> {
        // Convert wp:anchor to wp:inline for compatibility
        if let Some(anchor) = self.anchor.take() {
            let inline = self.anchor_to_simple_inline(anchor)?;
            self.inline = Some(inline);
        }
        Ok(())
    }

    /// Converts wp:anchor to wp:inline, preserving essential image data
    fn anchor_to_simple_inline(&self, anchor: Anchor<'a>) -> crate::DocxResult<Inline<'a>> {
        Ok(Inline {
            // Preserve distances (margin from text)
            dist_t: anchor.dist_t,
            dist_b: anchor.dist_b,
            dist_l: anchor.dist_l,
            dist_r: anchor.dist_r,
            
            // Remove problematic positioning attributes for inline
            simple_pos_attr: None,
            relative_height: None,
            behind_doc: None,
            locked: None,
            layout_in_cell: None,
            allow_overlap: None,
            
            // Remove positioning (inline doesn't need complex positioning)
            simple_pos: None,
            position_horizontal: None,
            position_vertical: None,
            
            // Preserve size and content
            extent: anchor.extent,
            doc_property: anchor.doc_property,
            graphic: anchor.graphic,
        })
    }
}

#[derive(Debug, Default, XmlRead, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:anchor")]
pub struct Anchor<'a> {
    #[xml(attr = "distT")]
    pub dist_t: Option<isize>,
    #[xml(attr = "distB")]
    pub dist_b: Option<isize>,
    #[xml(attr = "distL")]
    pub dist_l: Option<isize>,
    #[xml(attr = "distR")]
    pub dist_r: Option<isize>,
    #[xml(attr = "simplePos")]
    pub simple_pos_attr: Option<isize>,
    #[xml(attr = "relativeHeight")]
    pub relative_height: Option<isize>,
    #[xml(attr = "behindDoc")]
    pub behind_doc: Option<bool>,
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    #[xml(attr = "layoutInCell")]
    pub layout_in_cell: Option<bool>,
    #[xml(attr = "allowOverlap")]
    pub allow_overlap: Option<bool>,

    #[xml(child = "wp:simplePos")]
    pub simple_pos: Option<SimplePos>,
    #[xml(child = "wp:positionH")]
    pub position_horizontal: Option<PositionHorizontal>,
    #[xml(child = "wp:positionV")]
    pub position_vertical: Option<PositionVertical>,
    #[xml(child = "wp:extent")]
    pub extent: Option<Extent>,
    #[xml(
        child = "wp:wrapNone",
        child = "wp:wrapSquare",
        child = "wp:wrapTight",
        child = "wp:wrapThrough",
        child = "wp:wrapTopAndBottom"
    )]
    pub wrap: Option<Wrap>,
    #[xml(child = "wp:docPr")]
    pub doc_property: DocPr<'a>,
    #[xml(child = "a:graphic")]
    pub graphic: Option<Graphic<'a>>,
}

impl<'a> XmlWrite for Anchor<'a> {
    fn to_writer<W: std::io::Write>(&self, writer: &mut hard_xml::XmlWriter<W>) -> hard_xml::XmlResult<()> {
        
        writer.write_element_start("wp:anchor")?;
        
        // Numeric attributes
        if let Some(dist_t) = self.dist_t {
            writer.write_attribute("distT", &dist_t.to_string())?;
        }
        if let Some(dist_b) = self.dist_b {
            writer.write_attribute("distB", &dist_b.to_string())?;
        }
        if let Some(dist_l) = self.dist_l {
            writer.write_attribute("distL", &dist_l.to_string())?;
        }
        if let Some(dist_r) = self.dist_r {
            writer.write_attribute("distR", &dist_r.to_string())?;
        }
        if let Some(simple_pos_attr) = self.simple_pos_attr {
            writer.write_attribute("simplePos", &simple_pos_attr.to_string())?;
        }
        if let Some(relative_height) = self.relative_height {
            writer.write_attribute("relativeHeight", &relative_height.to_string())?;
        }
        
        // OOXML Boolean attributes (0/1 instead of true/false)
        if let Some(behind_doc) = self.behind_doc {
            writer.write_attribute("behindDoc", if behind_doc { "1" } else { "0" })?;
        }
        if let Some(locked) = self.locked {
            writer.write_attribute("locked", if locked { "1" } else { "0" })?;
        }
        if let Some(layout_in_cell) = self.layout_in_cell {
            writer.write_attribute("layoutInCell", if layout_in_cell { "1" } else { "0" })?;
        }
        if let Some(allow_overlap) = self.allow_overlap {
            writer.write_attribute("allowOverlap", if allow_overlap { "1" } else { "0" })?;
        }
        
        writer.write_element_end_open()?;
        
        // Child elements
        if let Some(simple_pos) = &self.simple_pos {
            simple_pos.to_writer(writer)?;
        }
        if let Some(position_horizontal) = &self.position_horizontal {
            position_horizontal.to_writer(writer)?;
        }
        if let Some(position_vertical) = &self.position_vertical {
            position_vertical.to_writer(writer)?;
        }
        if let Some(extent) = &self.extent {
            extent.to_writer(writer)?;
        }
        if let Some(wrap) = &self.wrap {
            wrap.to_writer(writer)?;
        }
        self.doc_property.to_writer(writer)?;
        if let Some(graphic) = &self.graphic {
            graphic.to_writer(writer)?;
        }
        
        writer.write_element_end_close("wp:anchor")?;
        Ok(())
    }
}

#[derive(Debug, From, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Wrap {
    #[xml(tag = "wp:wrapNone")]
    None(WrapNone),
    #[xml(tag = "wp:wrapSquare")]
    Square(WrapSquare),
    #[xml(tag = "wp:wrapTight")]
    Tight(WrapTight),
    #[xml(tag = "wp:wrapThrough")]
    Through(WrapThrough),
    #[xml(tag = "wp:wrapTopAndBottom")]
    TopAndBottom(WrapTopAndBottom),
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:wrapNone")]
pub struct WrapNone {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:wrapSquare")]
pub struct WrapSquare {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:wrapTight")]
pub struct WrapTight {
    #[xml(attr = "wrapText")]
    pub wrap_text: WrapTextType,
    #[xml(child = "wp:wrapPolygon")]
    pub wrap_polygon: WrapPolygon,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:wrapPolygon")]
pub struct WrapPolygon {
    #[xml(attr = "edited")]
    pub edited: Option<bool>,
    #[xml(child = "wp:start")]
    pub start: WrapPolygonStart,
    #[xml(child = "wp:lineTo")]
    pub lineto: Vec<WrapPolygonLineTo>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:start")]
pub struct WrapPolygonStart {
    #[xml(attr = "x")]
    pub x: Option<isize>,
    #[xml(attr = "y")]
    pub y: Option<isize>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:lineTo")]
pub struct WrapPolygonLineTo {
    #[xml(attr = "x")]
    pub x: Option<isize>,
    #[xml(attr = "y")]
    pub y: Option<isize>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub enum WrapTextType {
    #[default]
    Both,
}

__string_enum! {
    WrapTextType {
    Both = "bothSides",
}}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:wrapThrough")]
pub struct WrapThrough {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:wrapTopAndBottom")]
pub struct WrapTopAndBottom {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:positionH")]
pub struct PositionHorizontal {
    #[xml(attr = "relativeFrom")]
    pub relative_from: Option<RelativeFromH>,
    #[xml(flatten_text = "wp:posOffset")]
    pub pos_offset: Option<isize>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:positionV")]
pub struct PositionVertical {
    #[xml(attr = "relativeFrom")]
    pub relative_from: Option<RelativeFromV>,
    #[xml(flatten_text = "wp:posOffset")]
    pub pos_offset: Option<isize>,
}

__define_enum! {
    RelativeFromH {
        Margin= "margin",	//Page Margin
        Page = "page",//	Page Edge
        Column= "column",//	Column
        Character= "character",//	Character
        LeftMargin= "leftMargin",//	Left Margin
        RightMargin= "rightMargin",//	Right Margin
        InsideMargin= "insideMargin",//	Inside Margin
        OUtsideMargin= "outsideMargin",//	Outside Margin
    }
}

__define_enum! {
    RelativeFromV {
        Margin= "margin",	//Page Margin
        Page = "page",//	Page Edge
        Paragraph= "paragraph",//	Paragraph
        Line= "line",//	Line
        TopMargin= "topMargin",//	Left Margin
        BottomMargin= "bottomMargin",//	Right Margin
        InsideMargin= "insideMargin",//	Inside Margin
        OUtsideMargin= "outsideMargin",//	Outside Margin
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:inline")]
pub struct Inline<'a> {
    #[xml(attr = "distT")]
    pub dist_t: Option<isize>,
    #[xml(attr = "distB")]
    pub dist_b: Option<isize>,
    #[xml(attr = "distL")]
    pub dist_l: Option<isize>,
    #[xml(attr = "distR")]
    pub dist_r: Option<isize>,
    #[xml(attr = "simplePossimplePos")]
    pub simple_pos_attr: Option<isize>,
    #[xml(attr = "relativeHeight")]
    pub relative_height: Option<isize>,
    #[xml(attr = "behindDoc")]
    pub behind_doc: Option<bool>,
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    #[xml(attr = "layoutInCell")]
    pub layout_in_cell: Option<bool>,
    #[xml(attr = "allowOverlap")]
    pub allow_overlap: Option<bool>,

    #[xml(child = "wp:simplePos")]
    pub simple_pos: Option<SimplePos>,
    #[xml(child = "wp:positionH")]
    pub position_horizontal: Option<PositionHorizontal>,
    #[xml(child = "wp:positionV")]
    pub position_vertical: Option<PositionVertical>,
    #[xml(child = "wp:extent")]
    pub extent: Option<Extent>,
    #[xml(child = "wp:docPr")]
    pub doc_property: DocPr<'a>,
    #[xml(child = "a:graphic")]
    pub graphic: Option<Graphic<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:docPr")]
pub struct DocPr<'a> {
    #[xml(attr = "id")]
    pub id: Option<isize>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "descr")]
    pub descr: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:graphic")]
pub struct Graphic<'a> {
    #[xml(default, attr = "xmlns:a")]
    pub a: Cow<'a, str>,
    #[xml(default, child = "a:graphicData")]
    pub data: GraphicData<'a>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:graphicData")]
pub struct GraphicData<'a> {
    #[xml(default, attr = "uri")]
    pub uri: Cow<'a, str>,
    // graphic data can have any element in any namespace as a child
    #[xml(child = "pic:pic")]
    pub children: Vec<Picture<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:pic")]
pub struct Picture<'a> {
    #[xml(default, attr = "xmlns:pic")]
    pub a: Cow<'a, str>,
    #[xml(child = "pic:nvPicPr")]
    pub nv_pic_pr: NvPicPr<'a>,
    #[xml(child = "pic:blipFill")]
    pub fill: BlipFill<'a>,
    #[xml(child = "pic:spPr")]
    pub sp_pr: SpPr<'a>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:spPr")]
pub struct SpPr<'a> {
    #[xml(child = "a:xfrm")]
    pub xfrm: Option<Xfrm>,
    #[xml(child = "a:prstGeom")]
    pub prst_geom: Option<PrstGeom<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:prstGeom")]
pub struct PrstGeom<'a> {
    #[xml(attr = "prst")]
    pub prst: Option<Cow<'a, str>>,
    #[xml(child = "a:avLst")]
    pub av_lst: Option<AvList>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:avLst")]
pub struct AvList {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:xfrm")]
pub struct Xfrm {
    #[xml(child = "a:off")]
    pub offset: Option<Offset>,
    #[xml(child = "a:ext")]
    pub ext: Option<Ext>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:ext")]
pub struct Ext {
    #[xml(attr = "cx")]
    pub cx: Option<isize>,
    #[xml(attr = "cy")]
    pub cy: Option<isize>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:off")]
pub struct Offset {
    #[xml(attr = "x")]
    pub x: Option<isize>,
    #[xml(attr = "y")]
    pub y: Option<isize>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:simplePos")]
pub struct SimplePos {
    #[xml(attr = "x")]
    pub x: Option<isize>,
    #[xml(attr = "y")]
    pub y: Option<isize>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:nvPicPr")]
pub struct NvPicPr<'a> {
    #[xml(child = "pic:cNvPr")]
    pub c_nv_pr: Option<CNvPr<'a>>,
    #[xml(child = "pic:cNvPicPr")]
    pub c_nv_pic_pr: Option<CNvPicPr>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:cNvPr")]
pub struct CNvPr<'a> {
    #[xml(attr = "id")]
    pub id: Option<isize>,
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
    #[xml(attr = "descr")]
    pub descr: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:cNvPicPr")]
pub struct CNvPicPr {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:blipFill")]
pub struct BlipFill<'a> {
    #[xml(default, child = "a:blip")]
    pub blip: Blip<'a>,
    #[xml(child = "a:stretch")]
    pub stretch: Option<Stretch>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:blip")]
pub struct Blip<'a> {
    #[xml(default, attr = "r:embed")]
    pub embed: Cow<'a, str>,
    #[xml(default, attr = "cstate")]
    pub cstate: Option<Cow<'a, str>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:stretch")]
pub struct Stretch {
    #[xml(child = "a:fillRect")]
    pub fill_rect: Option<FillRect>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:fillRect")]
pub struct FillRect {}

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:extent")]
pub struct Extent {
    #[xml(default, attr = "cx")]
    pub cx: u64,

    #[xml(default, attr = "cy")]
    pub cy: u64,
}
