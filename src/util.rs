use regex::CaptureMatches;
use crate::commands::Command;

pub fn join_args(iter: CaptureMatches) -> String {
    iter
        .map(|x| x.get(0).unwrap().as_str())
        .collect::<String>()
}

pub struct IsOriginal(pub &'static (dyn Command + Sync), pub bool);
