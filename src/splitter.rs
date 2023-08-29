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
