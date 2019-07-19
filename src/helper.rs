use regex::Match;

pub fn unwrap_optional_match<'t>(regex_match: Option<Match<'t>>) -> Option<&str> {
    if regex_match.is_some() {
        Some(regex_match.unwrap().as_str())
    } else {
        None
    }
}
