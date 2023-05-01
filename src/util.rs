use regex::{Match, Regex};
use std::ops::{
    Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use std::str::FromStr;

use lazy_static::lazy_static;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnyRange<Idx> {
    Index(Idx),
    Range(Range<Idx>),
    RangeFrom(RangeFrom<Idx>),
    RangeFull(RangeFull),
    RangeInclusive(RangeInclusive<Idx>),
    RangeTo(RangeTo<Idx>),
    RangeToInclusive(RangeToInclusive<Idx>),
}

impl<Idx> AnyRange<Idx>
where
    Idx: PartialOrd<Idx>,
{
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: PartialOrd<Idx> + ?Sized,
    {
        match self {
            AnyRange::Index(i) => i == item,
            AnyRange::Range(r) => r.contains(item),
            AnyRange::RangeFrom(r) => r.contains(item),
            AnyRange::RangeFull(r) => r.contains(item),
            AnyRange::RangeInclusive(r) => r.contains(item),
            AnyRange::RangeTo(r) => r.contains(item),
            AnyRange::RangeToInclusive(r) => r.contains(item),
        }
    }
}

#[derive(Debug)]
pub struct ParseAnyRangeError;

impl FromStr for AnyRange<usize> {
    type Err = ParseAnyRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: do more precise errors

        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?<start>\d+)?(?:(?<range>\.\.)(?<inclusive>=)?(?<end>\d+)?)?")
                    .unwrap();
        }

        let captures = RE.captures(s).unwrap();
        let start: Option<usize> = captures
            .name("start")
            .map(|m| m.as_str().parse::<u16>().unwrap().into());
        let range = captures.name("range");
        let inclusive = captures.name("inclusive");
        let end = captures
            .name("end")
            .map(|m| m.as_str().parse::<u16>().unwrap().into());

        match (start, range, inclusive, end) {
            // 1
            (Some(start), None, None, None) => Ok(AnyRange::Index(start)),

            // 1..
            (Some(start), Some(_), None, None) => Ok(AnyRange::RangeFrom(RangeFrom { start })),

            // ..
            (None, Some(_), None, None) => Ok(AnyRange::RangeFull(RangeFull)),

            // 1..2
            (Some(start), Some(_), None, Some(end)) => Ok(AnyRange::Range(Range { start, end })),

            // ..2
            (None, Some(_), None, Some(end)) => Ok(AnyRange::RangeTo(RangeTo { end })),

            // 1..=2
            (Some(start), Some(_), Some(_), Some(end)) => {
                Ok(AnyRange::RangeInclusive(RangeInclusive::new(start, end)))
            }

            // ..=2
            (None, Some(_), Some(_), Some(end)) => {
                Ok(AnyRange::RangeToInclusive(RangeToInclusive { end }))
            }

            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::AnyRange;
    use std::ops::*;

    #[test]
    fn from_str_works() {
        let start = 1;
        let end = 2;
        assert_eq!("1".parse::<AnyRange<usize>>().unwrap(), AnyRange::Index(1));
        assert_eq!(
            "1..".parse::<AnyRange<usize>>().unwrap(),
            AnyRange::RangeFrom(RangeFrom { start })
        );
        assert_eq!(
            "1..2".parse::<AnyRange<usize>>().unwrap(),
            AnyRange::Range(Range { start, end })
        );
        assert_eq!(
            "..2".parse::<AnyRange<usize>>().unwrap(),
            AnyRange::RangeTo(RangeTo { end })
        );
        assert_eq!(
            "..=2".parse::<AnyRange<usize>>().unwrap(),
            AnyRange::RangeToInclusive(RangeToInclusive { end })
        );
        assert_eq!(
            "..".parse::<AnyRange<usize>>().unwrap(),
            AnyRange::RangeFull(RangeFull)
        );
        assert_eq!(
            "1..=2".parse::<AnyRange<usize>>().unwrap(),
            AnyRange::RangeInclusive(RangeInclusive::new(start, end))
        )
    }
}
