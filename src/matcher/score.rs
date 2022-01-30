type Cache = Vec<Vec<Option<usize>>>;

fn str_dist(source: &str, target: &str) -> usize {
    str_dist_cached(
        source,
        target,
        &mut vec![vec![None; target.len() + 1]; source.len() + 1],
    )
}
fn str_dist_cached(source: &str, target: &str, cache: &mut Cache) -> usize {
    if let Some(result) = cache[source.len()][target.len()] {
        return result;
    }
    let result = match (source.chars().next(), target.chars().next()) {
        (Some(a), Some(b)) if a == b => {
            str_dist_cached(&source[a.len_utf8()..], &target[b.len_utf8()..], cache)
        }
        (Some(a), Some(b)) => {
            1 + [
                str_dist_cached(&source[a.len_utf8()..], target, cache),
                str_dist_cached(source, &target[b.len_utf8()..], cache),
                str_dist_cached(&source[a.len_utf8()..], &target[b.len_utf8()..], cache),
            ]
            .iter()
            .min()
            .expect("We know that the array has 3 elements.")
        }

        _ => source.chars().count() + target.chars().count(), // At least one these is zero.
    };
    cache[source.len()][target.len()] = Some(result);
    result
}

pub fn score(source: &str, target: &str) -> f64 {
    let dist = str_dist(source, target) as f64;
    let len = *[source.chars().count(), target.chars().count(), 1]
        .iter()
        .max()
        .unwrap() as f64;
    1.0 - dist / len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(str_dist("", ""), 0)
    }
    #[test]
    fn test_equal() {
        let test_str = "This is a test string 123";
        assert_eq!(str_dist(test_str, test_str), 0);
    }

    #[test]
    fn test_rm() {
        let test_str = "This is a test string 123";
        assert_eq!(str_dist("", test_str), test_str.len());
        assert_eq!(str_dist(test_str, ""), test_str.len());
    }

    #[test]
    fn test_change() {
        assert_eq!(str_dist("effect", "affect"), 1);
    }
    #[test]
    fn test_multiple() {
        assert_eq!(str_dist("efecters", "affecter"), 3);
    }
    #[test]
    fn test_unicode() {
        assert_eq!(str_dist("💩💩💩💩", "poop💩"), 4);
        assert_eq!(str_dist("💩", ""), 1);
        assert_eq!(str_dist("", "💩"), 1);
    }
    #[test]
    fn test_long() {
        assert_eq!(
            str_dist(
                "These two strings are really long and very dfferent. This means that",
                "this function might take a really long time, which I would like to avoid.",
            ),
            52,
        );
    }
}
