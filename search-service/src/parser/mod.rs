use anyhow::{bail, Result};

/// Parse a document into text chunks (512-token sliding window, 50-token overlap).
/// `filename` is used to detect format; `content` is the raw file bytes as a string.
pub fn parse(filename: &str, content: &str) -> Result<Vec<String>> {
    let ext = filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();

    let text = match ext.as_str() {
        "txt" => content.to_string(),
        "md" => strip_markdown(content),
        "pdf" => extract_pdf(content)?,
        _ => bail!("unsupported file type: .{ext}"),
    };

    Ok(chunk_text(&text, 512, 50))
}

/// Strip Markdown formatting to plain text using pulldown-cmark.
fn strip_markdown(md: &str) -> String {
    use pulldown_cmark::{Event, Parser as MdParser, Tag};

    let mut out = String::with_capacity(md.len());
    for event in MdParser::new(md) {
        match event {
            Event::Text(t) | Event::Code(t) => out.push_str(&t),
            Event::End(Tag::Paragraph) | Event::End(Tag::Heading { .. }) => {
                out.push('\n');
            }
            _ => {}
        }
    }
    out
}

/// Decode base64-encoded bytes and extract text from a PDF using lopdf.
fn extract_pdf(encoded: &str) -> Result<String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use lopdf::Document;

    let bytes = if encoded.starts_with('%') {
        // Raw PDF bytes passed directly (dev / testing shortcut)
        encoded.as_bytes().to_vec()
    } else {
        STANDARD.decode(encoded.trim())?
    };

    let doc = Document::load_mem(&bytes)?;
    let mut text = String::new();
    for page_id in doc.page_iter() {
        if let Ok(page_text) = doc.extract_text(&[page_id.0]) {
            text.push_str(&page_text);
            text.push('\n');
        }
    }
    Ok(text)
}

/// Split `text` into overlapping chunks using a sliding-window approach.
/// `window` = max tokens per chunk; `overlap` = shared tokens between adjacent chunks.
pub fn chunk_text(text: &str, window: usize, overlap: usize) -> Vec<String> {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.is_empty() {
        return vec![];
    }

    let step = window.saturating_sub(overlap).max(1);
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < tokens.len() {
        let end = (start + window).min(tokens.len());
        chunks.push(tokens[start..end].join(" "));
        if end == tokens.len() {
            break;
        }
        start += step;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_text_basic() {
        let text = (0..600).map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
        let chunks = chunk_text(&text, 512, 50);
        assert!(chunks.len() >= 2, "expected overlap to produce multiple chunks");
        for chunk in &chunks {
            let word_count = chunk.split_whitespace().count();
            assert!(word_count <= 512);
        }
    }

    #[test]
    fn test_strip_markdown() {
        let md = "# Title\n\nSome **bold** text.";
        let plain = strip_markdown(md);
        assert!(plain.contains("Title"));
        assert!(plain.contains("bold"));
        assert!(!plain.contains('#'));
    }
}
