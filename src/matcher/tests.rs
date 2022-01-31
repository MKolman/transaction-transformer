use super::*;
use crate::ui::string;
#[test]
fn full_match() {
    let mut acc_matcher =
        AccountMatcher::from(HashMap::from([("test".to_owned(), "blah".to_owned())]));
    let ui = string::UI("".into());
    assert_eq!(acc_matcher.find_match("test", &ui), "blah");
}
#[test]
fn good_match() {
    let mut acc_matcher = AccountMatcher::from(HashMap::from([(
        "test_account".to_owned(),
        "blah".to_owned(),
    )]));
    let ui = string::UI("".into());
    assert_eq!(acc_matcher.find_match("test_accounts", &ui), "blah");
}
#[test]
fn no_match() {
    let mut acc_matcher = AccountMatcher::new();
    let ui = string::UI("blah".into());
    assert_eq!(acc_matcher.find_match("test", &ui), "blah");
}
