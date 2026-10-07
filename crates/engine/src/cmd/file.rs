//! File menu: new, open, save, export, close, properties.

use serde_json::{Value, json};
use slidecraft_model::{Presentation, defaults};

use super::*;
use crate::{DocState, EngineError, Result, Session};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(noundo "file.new", "New Presentation", ["File"], Some("Cmd+N"), "{theme?: name, size?: [w,h] pt, blank?: bool}", always, new),
        cmd!(noundo "file.open", "Open…", ["File"], Some("Cmd+O"), "{path}", always, open),
        cmd!(noundo "file.openBytes", "Open Data", [], None, "{name, data: base64}", always, open_bytes),
        cmd!(noundo "file.save", "Save", ["File"], Some("Cmd+S"), "{path?, format?: slidecraft|pptx}", has_doc, save),
        cmd!(noundo "file.saveAs", "Save As…", ["File"], Some("Cmd+Shift+S"), "{path, format?}", has_doc, save_as),
        cmd!(noundo "file.saveTemplate", "Save as Template…", ["File"], None, "{path}", has_doc, save_as),
        cmd!(query "file.saveBytes", "Save to Data", [], None, "{format?: slidecraft|pptx} → {data: base64}", has_doc, save_bytes_cmd),
        cmd!(noundo "file.export", "Export…", ["File"], None, "{path, format: png|jpeg|pptx|slidecraft|outline|pdf, slide?: index, scale?: px per pt}", has_doc, export),
        cmd!(query "file.render", "Render Slide", [], None, "{slide?: index, scale?, edit?: bool} → {png: base64, width, height}", has_doc, render),
        cmd!(noundo "file.close", "Close", ["File"], Some("Cmd+W"), "{}", has_doc, close),
        cmd!(
            "file.properties",
            "Properties",
            ["File"],
            None,
            "{title?, subject?, author?, keywords?, comments?, category?, company?} → properties",
            has_doc,
            properties
        ),
        cmd!(noundo "file.revert", "Revert", [], None, "{}", has_doc, revert),
        cmd!(noundo "window.next", "Next Window", ["Window"], Some("Cmd+`"), "{index?}", has_doc, next_window),
    ]
}

fn theme_named(name: &str) -> Option<slidecraft_model::Theme> {
    slidecraft_model::theme::builtin_themes().into_iter().find(|t| t.name.eq_ignore_ascii_case(name))
}

fn new(s: &mut Session, p: &Value) -> Result<Value> {
    let theme = str_param(p, "theme").and_then(theme_named).unwrap_or_default();
    let size = p
        .get("size")
        .and_then(Value::as_array)
        .and_then(|a| Some(slidecraft_geom::Size::new(a.first()?.as_f64()?, a.get(1)?.as_f64()?)))
        .filter(|s| s.width >= 1.0 && s.height >= 1.0 && s.width < 5000.0 && s.height < 5000.0)
        .unwrap_or(defaults::WIDE);
    let doc = defaults::blank_presentation(size, theme, !bool_or(p, "blank", false));
    let title = s.next_untitled();
    let i = s.add_document(DocState::new(doc, None, title));
    Ok(json!({"document": i}))
}

/// Recognise and read presentation bytes (native, PPTX).
pub fn open_presentation(name: &str, bytes: &[u8]) -> Result<Presentation> {
    if slidecraft_format::sniff(bytes) {
        return slidecraft_format::load(bytes).map_err(|e| EngineError::Other(e.to_string()));
    }
    if slidecraft_pptx::sniff(bytes) {
        let mut p = slidecraft_pptx::import(bytes).map_err(|e| EngineError::Other(e.to_string()))?;
        slidecraft_format::repair(&mut p);
        return Ok(p);
    }
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".txt") || lower.ends_with(".md") {
        let text = String::from_utf8_lossy(bytes);
        let mut p = defaults::blank_presentation(defaults::WIDE, Default::default(), false);
        slidecraft_format::outline_to_slides(&mut p, &text);
        if p.slides.is_empty() {
            p = defaults::new_presentation(None);
        }
        return Ok(p);
    }
    Err(EngineError::Other(format!("{name}: not a presentation SlideCraft can read")))
}

pub fn add_opened(s: &mut Session, name: &str, path: Option<String>, p: Presentation) -> usize {
    let title = std::path::Path::new(name).file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| name.to_string());
    s.add_document(DocState::new(p, path, title))
}

#[cfg(not(target_arch = "wasm32"))]
fn open(s: &mut Session, p: &Value) -> Result<Value> {
    let path = str_param(p, "path").ok_or_else(|| bad("file.open", "missing `path`"))?;
    let bytes = std::fs::read(path).map_err(|e| EngineError::Other(format!("{path}: {e}")))?;
    let doc = open_presentation(path, &bytes)?;
    let native = slidecraft_format::sniff(&bytes) || slidecraft_pptx::sniff(&bytes);
    let i = add_opened(s, path, native.then(|| path.to_string()), doc);
    Ok(json!({"document": i, "slides": s.doc()?.doc.slides.len()}))
}
#[cfg(target_arch = "wasm32")]
fn open(_s: &mut Session, _p: &Value) -> Result<Value> {
    Err(EngineError::Other("use file.openBytes in the browser".into()))
}

fn open_bytes(s: &mut Session, p: &Value) -> Result<Value> {
    let name = str_param(p, "name").unwrap_or("Presentation");
    let data = str_param(p, "data").and_then(base64_decode).ok_or_else(|| bad("file.openBytes", "missing or invalid base64 `data`"))?;
    let doc = open_presentation(name, &data)?;
    let i = add_opened(s, name, None, doc);
    Ok(json!({"document": i}))
}

/// Encode the presentation in `format` (`slidecraft`, `pptx`, `outline`).
pub fn save_bytes(doc: &Presentation, format: &str) -> Result<Vec<u8>> {
    match format {
        "pptx" | "potx" | "ppsx" => slidecraft_pptx::export(doc).map_err(|e| EngineError::Other(e.to_string())),
        "outline" | "txt" => Ok(slidecraft_format::slides_to_outline(doc).into_bytes()),
        _ => slidecraft_format::save(doc).map_err(|e| EngineError::Other(e.to_string())),
    }
}

pub fn format_for_path(path: &str) -> &'static str {
    let l = path.to_ascii_lowercase();
    if l.ends_with(".pptx") || l.ends_with(".potx") || l.ends_with(".ppsx") {
        "pptx"
    } else if l.ends_with(".png") {
        "png"
    } else if l.ends_with(".jpg") || l.ends_with(".jpeg") {
        "jpeg"
    } else if l.ends_with(".pdf") {
        "pdf"
    } else if l.ends_with(".txt") || l.ends_with(".md") {
        "outline"
    } else {
        "slidecraft"
    }
}

fn mark_saved(s: &mut Session, path: Option<String>) -> Result<()> {
    let st = s.doc_mut()?;
    st.saved_doc = st.doc.clone();
    if path.is_some() {
        st.path = path;
    }
    st.revision += 1;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn write_file(path: &str, bytes: &[u8]) -> Result<()> {
    // Write to a temporary sibling then rename, so a failed save never truncates the old file.
    let tmp = format!("{path}.saving");
    std::fs::write(&tmp, bytes).map_err(|e| EngineError::Other(format!("{path}: {e}")))?;
    std::fs::rename(&tmp, path).map_err(|e| EngineError::Other(format!("{path}: {e}")))
}
#[cfg(target_arch = "wasm32")]
fn write_file(_path: &str, _bytes: &[u8]) -> Result<()> {
    Err(EngineError::Other("saving to a path is not available in the browser".into()))
}

fn save(s: &mut Session, p: &Value) -> Result<Value> {
    let path = str_param(p, "path").map(String::from).or_else(|| s.active().and_then(|d| d.path.clone()));
    let Some(path) = path else {
        s.ui_requests.push(crate::UiRequest::PickFile { purpose: "saveAs".into() });
        return Ok(json!({"needsPath": true}));
    };
    let format = str_param(p, "format").unwrap_or(format_for_path(&path));
    let bytes = save_bytes(&s.doc()?.doc, format)?;
    write_file(&path, &bytes)?;
    mark_saved(s, Some(path.clone()))?;
    Ok(json!({"path": path, "bytes": bytes.len()}))
}

fn save_as(s: &mut Session, p: &Value) -> Result<Value> {
    if str_param(p, "path").is_none() {
        s.ui_requests.push(crate::UiRequest::PickFile { purpose: "saveAs".into() });
        return Ok(json!({"needsPath": true}));
    }
    save(s, p)
}

fn save_bytes_cmd(s: &mut Session, p: &Value) -> Result<Value> {
    let format = str_param(p, "format").unwrap_or("slidecraft");
    let bytes = save_bytes(&s.doc()?.doc, format)?;
    Ok(json!({"data": base64_encode(&bytes), "bytes": bytes.len()}))
}

/// Render slide `index` (current when absent) to PNG bytes.
pub fn render_png(doc: &Presentation, index: usize, scale: f64, edit: bool) -> (Vec<u8>, u32, u32) {
    let img =
        slidecraft_render::render_slide(doc, index, &slidecraft_render::RenderOpts { scale: scale.clamp(0.01, 16.0), edit, ..Default::default() });
    (img.to_png(), img.width, img.height)
}

fn render(s: &mut Session, p: &Value) -> Result<Value> {
    let st = s.doc()?;
    let i = usize_param(p, "slide").unwrap_or(st.selection.slide);
    if i >= st.doc.slides.len() {
        return Err(bad("file.render", format!("no slide {i}")));
    }
    let (png, w, h) = render_png(&st.doc, i, f64_or(p, "scale", 1.0), bool_or(p, "edit", false));
    Ok(json!({"png": base64_encode(&png), "width": w, "height": h}))
}

fn export(s: &mut Session, p: &Value) -> Result<Value> {
    let path = str_param(p, "path").ok_or_else(|| bad("file.export", "missing `path`"))?.to_string();
    let format = str_param(p, "format").unwrap_or(format_for_path(&path)).to_string();
    let st = s.doc()?;
    let scale = f64_or(p, "scale", 2.0);
    match format.as_str() {
        "png" | "jpeg" | "jpg" => {
            let slides: Vec<usize> = match usize_param(p, "slide") {
                Some(i) => vec![i],
                None if bool_or(p, "all", false) => (0..st.doc.slides.len()).collect(),
                None => vec![st.selection.slide],
            };
            let mut written = vec![];
            for (k, i) in slides.iter().enumerate() {
                if *i >= st.doc.slides.len() {
                    return Err(bad("file.export", format!("no slide {i}")));
                }
                let img = slidecraft_render::render_slide(
                    &st.doc,
                    *i,
                    &slidecraft_render::RenderOpts { scale: scale.clamp(0.01, 16.0), ..Default::default() },
                );
                let bytes = if format == "png" { img.to_png() } else { img.to_jpeg(92) };
                let out = if slides.len() == 1 {
                    path.clone()
                } else {
                    let pth = std::path::Path::new(&path);
                    let stem = pth.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_else(|| "Slide".into());
                    let ext = pth.extension().map(|x| x.to_string_lossy().to_string()).unwrap_or_else(|| format.clone());
                    pth.with_file_name(format!("{stem}{}.{ext}", k + 1)).to_string_lossy().to_string()
                };
                write_file(&out, &bytes)?;
                written.push(out);
            }
            Ok(json!({"files": written}))
        }
        "pdf" => Err(EngineError::Other("PDF export is not available yet".into())),
        other => {
            let bytes = save_bytes(&st.doc, other)?;
            write_file(&path, &bytes)?;
            Ok(json!({"path": path, "bytes": bytes.len()}))
        }
    }
}

fn close(s: &mut Session, _p: &Value) -> Result<Value> {
    if let Some(i) = s.active_index() {
        s.close_document(i);
    }
    ok()
}

fn properties(s: &mut Session, p: &Value) -> Result<Value> {
    let fields = ["title", "subject", "author", "keywords", "comments", "category", "company"];
    if fields.iter().any(|f| p.get(*f).is_some()) {
        s.edit(|doc, _| {
            let pr = &mut doc.props;
            for f in fields {
                if let Some(v) = str_param(p, f) {
                    let slot = match f {
                        "title" => &mut pr.title,
                        "subject" => &mut pr.subject,
                        "author" => &mut pr.author,
                        "keywords" => &mut pr.keywords,
                        "comments" => &mut pr.comments,
                        "category" => &mut pr.category,
                        _ => &mut pr.company,
                    };
                    *slot = v.to_string();
                }
            }
            Ok(())
        })?;
    }
    Ok(serde_json::to_value(&s.doc()?.doc.props).unwrap_or_default())
}

fn revert(s: &mut Session, _p: &Value) -> Result<Value> {
    let st = s.doc_mut()?;
    st.doc = st.saved_doc.clone();
    st.history = crate::History { limit: st.history.limit, ..Default::default() };
    st.revision += 1;
    ok()
}

fn next_window(s: &mut Session, p: &Value) -> Result<Value> {
    let n = s.documents().len();
    if n == 0 {
        return ok();
    }
    let i = usize_param(p, "index").unwrap_or_else(|| (s.active_index().unwrap_or(0) + 1) % n);
    s.set_active(i.min(n - 1));
    ok()
}
