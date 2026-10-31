//! Preview the exact Markdown exported from the current document.

use egui::{Color32, FontFamily, FontId, RichText, Stroke, TextFormat, Ui, text::LayoutJob};
use wordcraft_formats::model::{FBlock, Flow, Inline, Kind, ListCounter, Para};

use crate::WordApp;

pub(crate) fn show(app: &mut WordApp, ui: &mut Ui, tab: &mut u8, text: &mut String, flow: &Flow) -> bool {
    let t = crate::theme::Tokens::get(ui.ctx());
    ui.heading(tl!("Markdown preview for sharing"));
    ui.horizontal(|ui| {
        for (value, label) in [(0, tl!("Formatted view")), (1, tl!("Plain Markdown text"))] {
            let active = *tab == value;
            let fill = if active {
                t.accent
            } else if t.dark {
                Color32::from_rgb(0x2e, 0x33, 0x3d)
            } else {
                Color32::from_rgb(0xf0, 0xf2, 0xf5)
            };
            let text_color = if active {
                t.on_accent
            } else if t.dark {
                Color32::from_rgb(0xd8, 0xde, 0xe9)
            } else {
                Color32::from_rgb(0x33, 0x33, 0x33)
            };
            let border = if active {
                if t.dark { Color32::WHITE } else { t.accent }
            } else if t.dark {
                Color32::from_rgb(0x88, 0x92, 0xa6)
            } else {
                Color32::from_rgb(0xaa, 0xaa, 0xaa)
            };
            let button = egui::Button::new(RichText::new(label).color(text_color))
                .selected(active)
                .fill(fill)
                .stroke(Stroke::new(1.5, border))
                .corner_radius(egui::CornerRadius::same(5));
            if ui.add(button).clicked() {
                *tab = value;
            }
        }
    });
    ui.add_space(4.0);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_height(360.0);
        egui::ScrollArea::both().id_salt("markdown-preview-content").max_height(360.0).show(ui, |ui| {
            ui.set_min_width(690.0);
            if *tab == 0 {
                let mut lists = ListCounter::default();
                for (index, block) in flow.blocks.iter().enumerate() {
                    match block {
                        FBlock::Para(p) => paragraph(ui, p, &mut lists),
                        FBlock::Table(table) => {
                            lists.reset();
                            egui::Grid::new(("markdown-preview-table", index)).spacing(egui::vec2(0.0, 0.0)).show(ui, |ui| {
                                for row in &table.rows {
                                    for cell in row {
                                        let label = cell.text();
                                        egui::Frame::new()
                                            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                                            .inner_margin(egui::Margin::symmetric(6, 4))
                                            .show(ui, |ui| {
                                                ui.label(if cell.header { RichText::new(label).strong() } else { RichText::new(label) });
                                            });
                                    }
                                    ui.end_row();
                                }
                            });
                            ui.separator();
                        }
                    }
                }
            } else {
                let mut raw = text.as_str();
                ui.add(egui::TextEdit::multiline(&mut raw).font(egui::TextStyle::Monospace).desired_width(f32::INFINITY).interactive(false));
            }
        });
    });
    ui.add_space(8.0);
    let mut close = false;
    ui.horizontal_wrapped(|ui| {
        let border = if t.dark { Color32::from_rgb(0xc0, 0xd4, 0xff) } else { Color32::from_rgb(0x28, 0x48, 0xa0) };
        let copy =
            crate::widgets::primary_button(ui, tl!("Copy to clipboard")).stroke(Stroke::new(1.5, border)).corner_radius(egui::CornerRadius::same(5));
        if ui.add(copy).clicked() {
            ui.ctx().copy_text(text.clone());
            app.status(tl!("Copied Markdown to the clipboard."));
        }
        if ui.add(secondary_button(tl!("Save as Markdown (.md)"), t.dark)).clicked() {
            app.export_dialog("md");
        }
        if ui.add(secondary_button(tl!("Close"), t.dark)).clicked() || crate::dialogs::cancel_key_pressed(ui) {
            close = true;
        }
    });
    close
}

fn secondary_button(label: &str, dark: bool) -> egui::Button<'_> {
    let fill = if dark { Color32::from_rgb(0x2e, 0x33, 0x3d) } else { Color32::from_rgb(0xf2, 0xf4, 0xf7) };
    let text = if dark { Color32::from_rgb(0xee, 0xee, 0xee) } else { Color32::from_rgb(0x22, 0x22, 0x22) };
    let border = if dark { Color32::from_rgb(0x90, 0x9a, 0xae) } else { Color32::from_rgb(0x99, 0x99, 0x99) };
    egui::Button::new(RichText::new(label).color(text)).fill(fill).stroke(Stroke::new(1.5, border)).corner_radius(egui::CornerRadius::same(5))
}

fn paragraph(ui: &mut Ui, para: &Para, lists: &mut ListCounter) {
    if para.kind == Kind::Rule {
        ui.separator();
        return;
    }
    let heading = matches!(para.kind, Kind::Heading(_) | Kind::Title);
    let size = match para.kind {
        Kind::Title | Kind::Heading(1) => 22.0,
        Kind::Heading(2) => 19.0,
        Kind::Heading(_) => 16.0,
        _ => 14.0,
    };
    let color = ui.visuals().text_color();
    let mut job = LayoutJob::default();
    if let Some(list) = para.list {
        let number = lists.next(list);
        let prefix = if list.ordered { format!("{number}. ") } else { "• ".to_string() };
        job.append(&prefix, f32::from(list.level) * 16.0, TextFormat::simple(FontId::proportional(size), color));
    } else {
        lists.reset();
    }
    for span in &para.inlines {
        let (content, fmt) = match span {
            Inline::Text(s, fmt) => (s.as_str(), Some(fmt)),
            Inline::Ruby { base, fmt, .. } => (base.as_str(), Some(fmt)),
            Inline::Equation { linear, .. } => (linear.as_str(), None),
            Inline::Image(image) => (image.alt.as_str(), None),
            Inline::Figure(alt) => (alt.as_str(), None),
            Inline::Anchor(_) => continue,
        };
        let bold = heading || fmt.is_some_and(|f| f.bold);
        let code = para.kind == Kind::Code || fmt.is_some_and(|f| f.code);
        let family = if code {
            FontFamily::Monospace
        } else if bold {
            FontFamily::Name("semibold".into())
        } else {
            FontFamily::Proportional
        };
        let mut format = TextFormat::simple(FontId::new(size, family), color);
        format.italics = fmt.is_some_and(|f| f.italic) || para.kind == Kind::Quote;
        if fmt.is_some_and(|f| f.strike) {
            format.strikethrough = egui::Stroke::new(1.0, color);
        }
        if fmt.is_some_and(|f| f.underline || f.link.is_some()) {
            format.underline = egui::Stroke::new(1.0, color);
        }
        job.append(content, 0.0, format);
    }
    ui.label(job);
    ui.add_space(if heading { 6.0 } else { 3.0 });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatted_preview_parses_headings_lists_and_tables() {
        let flow = wordcraft_formats::markdown::parse("# Heading\n\n- item\n\n| A | B |\n| --- | --- |\n| one | two |\n");
        assert!(flow.blocks.iter().any(|b| matches!(b, FBlock::Para(p) if p.kind == Kind::Heading(1))));
        assert!(flow.blocks.iter().any(|b| matches!(b, FBlock::Para(p) if p.list.is_some())));
        assert!(flow.blocks.iter().any(|b| matches!(b, FBlock::Table(t) if t.rows.len() == 2)));
    }
}
