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
        if self.is_equal_to(other) {
            return false;
        }
        let lhs = self
            .split('.')
            .map(|s| {
                s.parse::<u32>()
                    .expect(&format!("Semver format error {}", self))
            })
            .collect::<Vec<_>>();
        let rhs = other
            .split('.')
            .map(|s| {
                s.parse::<u32>()
                    .expect(&format!("Semver format error {}", other))
            })
            .collect::<Vec<_>>();

        if lhs[2] > rhs[2] {
            return true;
        }
        if lhs[1] > rhs[1] {
            return true;
        }
        if lhs[0] > rhs[0] {
            return true;
        }
        return false;
    }

    fn is_smaller_than(&self, other: &Self) -> bool {
        if self.is_equal_to(other) {
            return false;
        }
        let lhs = self
            .split('.')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let rhs = other
            .split('.')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        if lhs[2] < rhs[2] {
            return true;
        }
        if lhs[1] < rhs[1] {
            return true;
        }
        if lhs[0] < rhs[0] {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_cmp() {
        assert!("1.2.3".is_equal_to(&"1.2.3"));
        assert!("1.2.3".is_bigger_than(&"1.2.0"));
        assert!("1.2.3".is_smaller_than(&"1.2.4"));
    }
}
