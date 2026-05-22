use std::fmt::Write;
use std::path::Path;

use rsomics_common::{Result, RsomicsError};

pub fn align(reference: &Path, query: &Path, preset: &str) -> Result<String> {
    let mm_preset = match preset {
        "sr" => minimap2::Preset::Sr,
        "map-hifi" => minimap2::Preset::MapHifi,
        "map-pb" => minimap2::Preset::MapPb,
        _ => minimap2::Preset::MapOnt,
    };

    let aligner = minimap2::Aligner::builder()
        .preset(mm_preset)
        .with_index(reference.to_str().unwrap_or(""), None)
        .map_err(|e| RsomicsError::InvalidInput(format!("building index: {e}")))?;

    let seqs = std::fs::read_to_string(query)
        .map_err(|e| RsomicsError::InvalidInput(format!("{}: {e}", query.display())))?;

    let mut output = String::new();
    for record in seqs.split('>').skip(1) {
        let mut lines = record.lines();
        let name = lines.next().unwrap_or("unknown");
        let seq: String = lines.collect();

        let mappings = aligner
            .map(
                seq.as_bytes(),
                false,
                false,
                None,
                None,
                Some(name.as_bytes()),
            )
            .map_err(|e| RsomicsError::InvalidInput(format!("mapping: {e}")))?;

        for m in mappings {
            // PAF: qname qlen qstart qend strand tname tlen tstart tend nmatch alnlen mapq
            let strand = if matches!(m.strand, minimap2::Strand::Forward) {
                '+'
            } else {
                '-'
            };
            let qname = m.query_name.as_ref().map_or("*", |s| s.as_str());
            let tname = m.target_name.as_ref().map_or("*", |s| s.as_str());
            let qlen = m.query_len.map_or(0, std::num::NonZeroI32::get);
            writeln!(
                output,
                "{qname}\t{qlen}\t{}\t{}\t{strand}\t{tname}\t{}\t{}\t{}\t{}\t{}\t{}",
                m.query_start,
                m.query_end,
                m.target_len,
                m.target_start,
                m.target_end,
                m.match_len,
                m.block_len,
                m.mapq
            )
            .ok();
        }
    }

    Ok(output)
}
