use regex::Regex;


pub fn has_anchor_id(content: &str, anchor: &str) -> bool {
    let checks = anchor_id_checks(anchor);
    checks.is_match(content)
}

fn anchor_id_checks(anchor:&str) -> Regex {
    Regex::new(
        &format!(r#" (?i)(id|name) *= *("|')*{}("|'| |>)+"#, anchor)
    ).unwrap()
}


#[cfg(test)]
mod tests{
    use super::anchor_id_checks;

    fn check(anchor:&str, content:&str) -> bool {
        anchor_id_checks(anchor).is_match(content)
    }

    #[test]
    fn matchers () {
        let m = |content| {check("fred", content)};

        // Canonical match/non match
        assert!(m(r#"<a name="fred">"#));
        assert!(m(r#"<a id="fred">"#));
        assert!(!m(r#"<a name="george">"#));

        // Whitespace variants
        assert!(m(r#"<a id ="fred">"#));
        assert!(m(r#"<a id = "fred">"#));
        assert!(m(r#"<a id="fred" >"#));
        assert!(m(r#"<a  id="fred" >"#));

        // Quote variants
        assert!(m(r#"<a id='fred'>"#));
        assert!(m(r#"<a id=fred>"#));

        // Case variants
        assert!(m(r#"<a ID="fred">"#));
        assert!(m(r#"<a iD="fred">"#));
    }
}
