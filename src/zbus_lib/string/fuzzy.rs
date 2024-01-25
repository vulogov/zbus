extern crate log;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use distance::*;
use rhai::{Dynamic, NativeCallContext, EvalAltResult};

pub fn str_match(_context: NativeCallContext, t: String, p: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let matcher = SkimMatcherV2::default();
    match matcher.fuzzy_match(&t, &p) {
        Some(res) => Result::Ok(Dynamic::from(res)),
        None => Result::Ok(Dynamic::from(0 as i64)),
    }
}

pub fn str_match_levenshtein(_context: NativeCallContext, t: String, p: String) -> Result<Dynamic, Box<EvalAltResult>> {
    Result::Ok(Dynamic::from(levenshtein(&t, &p) as i64))
}

pub fn str_match_damerau(_context: NativeCallContext, t: String, p: String) -> Result<Dynamic, Box<EvalAltResult>> {
    Result::Ok(Dynamic::from(damerau_levenshtein(&t, &p) as i64))
}
