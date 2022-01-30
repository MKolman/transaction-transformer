use super::*;
use crate::ui::string::StringUI;
#[test]
fn full_match() {
    let mut acc_matcher =
        AccountMatcher::from(HashMap::from([("test".to_owned(), "blah".to_owned())]));
    let ui = StringUI("".into());
    assert_eq!(acc_matcher.find_match("test", &ui), "blah");
}
#[test]
fn good_match() {
    let mut acc_matcher = AccountMatcher::from(HashMap::from([(
        "test_account".to_owned(),
        "blah".to_owned(),
    )]));
    let ui = StringUI("".into());
    assert_eq!(acc_matcher.find_match("test_accounts", &ui), "blah");
}
#[test]
fn no_match() {
    let mut acc_matcher = AccountMatcher::new();
    let ui = StringUI("blah".into());
    assert_eq!(acc_matcher.find_match("test", &ui), "blah");
}
