/*! A validator for greek tax identification number (ΑΦΜ)
More information is available on
[ΑΦΜ](https://ti-einai.gr/afm/).
*/

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    true
}

/// Validates the given string for AFM
pub fn validate(afm: &str) -> (bool, &str) {
    if !is_string_numeric(afm.to_string()) {
        return (false, "Provided AFM is not a numeric value");
    }

    if afm.chars().count() != 9 {
        return (false, "Provided number is not 9 digits long");
    }

    let mut m = 1;
    let mut sum = 0;

    for i in (0..=7).rev() {
        m *= 2;
        let digit = afm.chars().nth(i).unwrap() as u32 - '0' as u32;
        sum += digit * m;
    }
    let checkdigit = afm.chars().nth(8).unwrap() as u32 - '0' as u32;
    if (sum % 11 % 10) != checkdigit {
        return (false, "Provided AFM does not have a valid checkdigit");
    }

    (true, "")
}

#[cfg(test)]
mod tests {
    use crate::validate;

    #[test]
    fn it_validates_valid_afm() {
        let (is_valid, _err) = validate("094353733");
        assert_eq!(true, is_valid);
    }

    #[test]
    fn it_fails_when_not_a_number() {
        let (is_valid, _err) = validate("asvs");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn it_fails_when_short_number() {
        let (is_valid, _err) = validate("0943537");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn it_fails_when_long_number() {
        let (is_valid, _err) = validate("094353733124");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn it_fails_with_bad_checkdigit() {
        let (is_valid, _err) = validate("123456789");
        assert_eq!(false, is_valid);
    }

    #[test]
    fn readme() {
        // An invalid AFM
        let (is_valid, err) = validate("123456789");
        assert!(!is_valid);
        println!("{}", err);

        // A valid AFM
        let (is_valid, err) = validate("997788278");
        assert!(is_valid);
        assert_eq!("", err)
    }
}
