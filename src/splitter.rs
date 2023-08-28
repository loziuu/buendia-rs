struct PrefixSplit {
    prefix: String,
    val: String,
    val_2: String,
}

fn prefix_split(val: &str, compare_to: &str) -> PrefixSplit {
    let mut prefix = String::new();
    let mut val_chars = val.chars();
    let mut compare_to_chars = compare_to.chars();

    loop {
        if let Some(char) = val_chars.next() {
            if let Some(ct_ch) = compare_to_chars.next() {
                if ct_ch == char {
                    prefix.push(ct_ch);
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }
    PrefixSplit {
        prefix: prefix.clone(),
        val: val[prefix.len()..].to_string(),
        val_2: compare_to[prefix.len()..].to_string(),
    }
}

pub fn prefix_split_only_position(val: &str, compare_to: &str) -> usize {
    let mut prefix = String::with_capacity(std::cmp::min(val.len(), compare_to.len()));
    let mut val_chars = val.chars();
    let mut compare_to_chars = compare_to.chars();

    loop {
        if let Some(char) = val_chars.next() {
            if let Some(ct_ch) = compare_to_chars.next() {
                if ct_ch == char {
                    prefix.push(ct_ch);
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }
    prefix.len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn split_from_begining() {
        let val = "Dzień";
        let val_2 = "Dzieńdobreczek";

        let splitted = super::prefix_split(val, val_2);

        assert_eq!(splitted.prefix, "Dzień");
        assert_eq!(splitted.val, "");
        assert_eq!(splitted.val_2, "dobreczek");
    }

    #[test]
    fn split_edge_cases() {
        let val = "aaaaaaabaaaaaaaaaa";
        let val_2 = "aaabaaaaa";

        let splitted = super::prefix_split(val, val_2);

        assert_eq!(splitted.prefix, "aaa");
        assert_eq!(splitted.val, "aaaabaaaaaaaaaa");
        assert_eq!(splitted.val_2, "baaaaa");
    }

    #[test]
    fn something_something() {
        let val = "construct";
        let val_2 = "consultant";

        let splitted = super::prefix_split(val, val_2);

        assert_eq!(splitted.prefix, "cons");
        assert_eq!(splitted.val, "truct");
        assert_eq!(splitted.val_2, "ultant");
    }

    #[test]
    fn split_equal_strings() {
        let val = "test";
        let val_2 = "test";

        let splitted = super::prefix_split(val, val_2);

        assert_eq!(splitted.prefix, "test");
        assert_eq!(splitted.val, "");
        assert_eq!(splitted.val_2, "");
    }

    #[test]
    fn only_position() {
        let val = "construct";
        let val_2 = "consultant";

        let splitted = super::prefix_split_only_position(val, val_2);

        assert_eq!(&val[..splitted], "cons");
        assert_eq!(&val_2[..splitted], "cons");
        assert_eq!(&val[splitted..], "truct");
        assert_eq!(&val_2[splitted..], "ultant");
    }

    #[test]
    fn split_equals_only_position() {
        let val = "test";
        let val_2 = "test";

        let splitted = super::prefix_split_only_position(val, val_2);

        assert_eq!(splitted, val.len());
    }

    #[test]
    fn split_no_prefix_only_position() {
        let val = "first";
        let val_2 = "second";

        let splitted = super::prefix_split_only_position(val, val_2);

        assert_eq!(splitted, 0);
    }
}
