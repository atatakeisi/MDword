//! Remove retained package parts and embedded objects from a document before saving a clean copy.

use std::sync::Arc;

use wordcraft_doc::{Block, Blocks, Document, InlineObject};

/// Drops opaque package parts and embedded object sources while retaining image previews.
pub fn strip_active_content(doc: &mut Document) {
    doc.passthrough.clear();
    doc.settings.attached_template = None;
    doc.settings.link_styles = false;
    sanitize_blocks(&mut doc.body);
    for part in doc.parts.values_mut() {
        sanitize_blocks(&mut part.blocks);
    }
    for entry in &mut doc.glossary {
        sanitize_blocks(&mut entry.blocks);
    }
}

fn sanitize_blocks(blocks: &mut Blocks) {
    let mut pending: Vec<&mut Block> = blocks.iter_mut().map(Arc::make_mut).collect();
    while let Some(block) = pending.pop() {
        match block {
            Block::Para(p) => {
                p.mark.link = None;
                for run in &mut p.runs {
                    run.props.link = None;
                }
                let mut objects: Vec<&mut InlineObject> = p.objects.iter_mut().collect();
                while let Some(object) = objects.pop() {
                    match object {
                        InlineObject::Image { ole, .. } => *ole = None,
                        InlineObject::Graphic { graphic, .. } => Arc::make_mut(graphic).source = None,
                        InlineObject::Group { children, .. } => objects.extend(children.iter_mut().map(|child| &mut child.obj)),
                        InlineObject::Field { instr, locked, dirty, code, form, .. } => {
                            instr.clear();
                            *locked = true;
                            *dirty = false;
                            *code = false;
                            form.clear();
                        }
                        InlineObject::FieldStart { instr, locked, dirty } => {
                            instr.clear();
                            *locked = true;
                            *dirty = false;
                        }
                        InlineObject::Opaque { text, .. } => {
                            *object = InlineObject::Field {
                                instr: String::new(),
                                result: std::mem::take(text),
                                locked: true,
                                dirty: false,
                                code: false,
                                form: String::new(),
                            };
                        }
                        _ => {}
                    }
                }
                p.touch();
            }
            Block::Table(table) => {
                pending.extend(table.rows.iter_mut().flat_map(|row| &mut row.cells).flat_map(|cell| &mut cell.blocks).map(Arc::make_mut));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wordcraft_doc::{CharProps, Paragraph, para::InlineObject};

    #[test]
    fn strips_ole_payload_and_retains_preview_and_text() {
        let mut doc = Document::from_text("Hello");
        doc.settings.attached_template = Some("https://example.invalid/template.dotm".into());
        doc.settings.link_styles = true;
        doc.passthrough.insert(crate::VBA_PROJECT_PART.into(), Arc::new(vec![1, 2]));
        doc.passthrough.insert("word/embeddings/evil.bin".into(), Arc::new(vec![3]));
        let image = InlineObject::Image {
            media: "preview".into(),
            w: 10.0,
            h: 10.0,
            alt: String::new(),
            float: Default::default(),
            crop: [0.0; 4],
            ole: Some(Arc::new(Default::default())),
        };
        let mut p = Paragraph::with_text("content", CharProps::default());
        p.insert_object(7, image, &CharProps::default()).unwrap();
        doc.body.push(Arc::new(Block::Para(p)));
        strip_active_content(&mut doc);
        assert!(doc.passthrough.is_empty());
        assert!(doc.settings.attached_template.is_none());
        assert!(!doc.settings.link_styles);
        let Block::Para(p) = doc.body.last().unwrap().as_ref() else { panic!("paragraph") };
        assert!(p.plain_text().contains("content"));
        assert!(matches!(p.objects.first(), Some(InlineObject::Image { ole: None, media, .. }) if media == "preview"));
        let saved = crate::write(&doc).unwrap();
        let reopened = crate::read(&saved).unwrap();
        assert!(!reopened.passthrough.contains_key(crate::VBA_PROJECT_PART));
        assert!(!reopened.passthrough.contains_key("word/embeddings/evil.bin"));
    }

    #[test]
    fn detects_relationship_to_nonstandard_vba_part() {
        let rels = br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="r1" Type="http://schemas.microsoft.com/office/2006/relationships/vbaProject" Target="custom.bin"/></Relationships>"#;
        let mut parts = std::collections::BTreeMap::new();
        parts.insert("word/_rels/document.xml.rels".into(), rels.to_vec());
        assert!(crate::package::Package::from_parts(parts).has_vba_relationship());
    }
}
