use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, PartialEq)]
pub struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_order() {
        assert!(
            Interval {
                lower: 10,
                upper: 20
            } < Interval {
                lower: 20,
                upper: 40
            }
        );
        assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
        assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });
    }

    #[test]
    fn test_overlapping() {
        let left = Interval {
            lower: 10,
            upper: 30,
        };
        let right = Interval {
            lower: 20,
            upper: 40,
        };
        assert!(!(left < right));
        assert!(!(left >= right));
    }
}
