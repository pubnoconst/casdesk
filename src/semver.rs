pub trait SemverCmp {
    fn is_bigger_than(&self, other: &Self) -> bool;
    fn is_smaller_than(&self, other: &Self) -> bool;
    fn is_equal_to(&self, other: &Self) -> bool;
}

impl SemverCmp for &str {
    fn is_equal_to(&self, other: &Self) -> bool {
        self == other
    }

    fn is_bigger_than(&self, other: &Self) -> bool {
        let (lhs, rhs) = (parse_semver(self), parse_semver(other));
        lhs > rhs
    }

    fn is_smaller_than(&self, other: &Self) -> bool {
        let (lhs, rhs) = (parse_semver(self), parse_semver(other));
        lhs < rhs
    }
}

fn parse_semver(semver: &str) -> (u32, u32, u32) {
    let parts: Vec<u32> = semver
        .split('.')
        .map(|s| s.parse::<u32>().expect(&format!("Invalid semver format: {}", semver)))
        .collect();

    if parts.len() != 3 {
        panic!("Semver must have exactly three parts: {}", semver);
    }

    (parts[0], parts[1], parts[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_cmp() {
        assert!("1.2.3".is_equal_to(&"1.2.3"));
        assert!("1.2.3".is_bigger_than(&"1.2.0"));
        assert!("1.2.3".is_smaller_than(&"1.2.4"));
        assert!("2.0.0".is_bigger_than(&"1.11.9"));
        assert!("0.1.0".is_smaller_than(&"0.1.1"));
    }
}
