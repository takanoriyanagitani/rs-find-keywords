use std::process::ExitCode;

use std::io;

use rs_find_keywords::find::aho::corasick::MATCH_KIND_DEFAULT;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} missing: {e}")))
}

fn match_kind() -> String {
    env_val_by_key("ENV_MATCH_KIND")
        .ok()
        .unwrap_or_else(|| MATCH_KIND_DEFAULT.into())
}

fn keywords_from_args() -> impl Iterator<Item = String> {
    std::env::args().skip(1)
}

fn stdin2lines2ids2stdout() -> Result<(), io::Error> {
    rs_find_keywords::find::aho::corasick::stdin2ids2stdout(
        keywords_from_args(),
        match_kind().as_str(),
    )
}

fn invert_match() -> bool {
    env_val_by_key("ENV_INVERT_MATCH")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or_default()
}

fn stdin2lines2founds2stdout() -> Result<(), io::Error> {
    rs_find_keywords::find::aho::corasick::stdin2founds2stdout(
        keywords_from_args(),
        match_kind().as_str(),
        invert_match(),
    )
}

fn write_id() -> bool {
    env_val_by_key("ENV_WRITE_ID")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or_default()
}

fn sub() -> Result<(), io::Error> {
    match write_id() {
        true => stdin2lines2ids2stdout(),
        false => stdin2lines2founds2stdout(),
    }
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
