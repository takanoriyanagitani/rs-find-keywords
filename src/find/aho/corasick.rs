use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::BufRead;

use aho_corasick::AhoCorasick;
use aho_corasick::MatchKind;

pub const MATCH_KIND_DEFAULT: &str = "standard";

pub fn find_words_write_line<W>(
    line: &[u8],
    patterns: &AhoCorasick,
    writer: &mut W,
    invert_match: bool,
) -> Result<(), io::Error>
where
    W: Write,
{
    let found: bool = patterns.is_match(line);
    if found ^ invert_match {
        writer.write_all(line)?;
        writeln!(writer)?;
    }
    Ok(())
}

pub fn lines2founds2writer<I, W>(
    lines: I,
    patterns: &AhoCorasick,
    writer: &mut W,
    invert_match: bool,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    W: Write,
{
    for rline in lines {
        let line: Vec<u8> = rline?;
        find_words_write_line(&line, patterns, writer, invert_match)?;
    }
    writer.flush()
}

/// Writes matched lines to the stdout.
pub fn stdin2founds2stdout<P, S>(
    patterns: P,
    match_kind: &str,
    invert_match: bool,
) -> Result<(), io::Error>
where
    P: Iterator<Item = S>,
    S: AsRef<[u8]>,
{
    let typ: MatchKind = str2match_kind(match_kind);
    let pat: AhoCorasick = AhoCorasick::builder()
        .match_kind(typ)
        .build(patterns)
        .map_err(io::Error::other)?;

    let i = io::stdin();
    let il = i.lock();
    let lines = il.split(b'\n');

    let o = io::stdout();
    let mut ol = o.lock();

    let mut bw = BufWriter::new(&mut ol);
    lines2founds2writer(lines, &pat, &mut bw, invert_match)?;
    drop(bw);

    ol.flush()
}

pub fn find_words_write_ids<W>(
    line: &[u8],
    patterns: &AhoCorasick,
    writer: &mut W,
) -> Result<(), io::Error>
where
    W: Write,
{
    let founds = patterns.try_find_iter(line).map_err(io::Error::other)?;
    let ids = founds.map(|p| p.pattern());
    for id in ids {
        let u: u32 = id.as_u32();
        writeln!(writer, "{u:08x}")?;
    }
    Ok(())
}

pub fn lines2ids2writer<I, W>(
    lines: I,
    patterns: &AhoCorasick,
    writer: &mut W,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    W: Write,
{
    for rline in lines {
        let line: Vec<u8> = rline?;
        find_words_write_ids(&line, patterns, writer)?;
    }
    writer.flush()
}

pub fn str2match_kind(s: &str) -> MatchKind {
    match s {
        "1st" => MatchKind::LeftmostFirst,
        "first" => MatchKind::LeftmostFirst,
        "mostfirst" => MatchKind::LeftmostFirst,
        "most-first" => MatchKind::LeftmostFirst,
        "leftmostfirst" => MatchKind::LeftmostFirst,
        "left-most-first" => MatchKind::LeftmostFirst,
        "long" => MatchKind::LeftmostLongest,
        "longest" => MatchKind::LeftmostLongest,
        "mostlongest" => MatchKind::LeftmostLongest,
        "most-longest" => MatchKind::LeftmostLongest,
        "left-most-longest" => MatchKind::LeftmostLongest,
        "leftmostlongest" => MatchKind::LeftmostLongest,
        "standard" => MatchKind::Standard,
        "std" => MatchKind::Standard,
        _ => MatchKind::Standard,
    }
}

/// Writes matched ids to the stdout.
pub fn stdin2ids2stdout<P, S>(patterns: P, match_kind: &str) -> Result<(), io::Error>
where
    P: Iterator<Item = S>,
    S: AsRef<[u8]>,
{
    let typ: MatchKind = str2match_kind(match_kind);
    let pat: AhoCorasick = AhoCorasick::builder()
        .match_kind(typ)
        .build(patterns)
        .map_err(io::Error::other)?;

    let i = io::stdin();
    let il = i.lock();
    let lines = il.split(b'\n');

    let o = io::stdout();
    let mut ol = o.lock();

    let mut bw = BufWriter::new(&mut ol);
    lines2ids2writer(lines, &pat, &mut bw)?;
    drop(bw);

    ol.flush()
}
