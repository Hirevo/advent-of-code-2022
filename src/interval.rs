#![allow(unused)]

use std::fmt::{Display, self};

use num_traits::PrimInt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Interval<T> {
    pub lo: T,
    pub hi: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Difference<T> {
    Two(Interval<T>, Interval<T>),
    One(Interval<T>),
    None,
}

impl<T> Interval<T> {
    pub fn new(lo: T, hi: T) -> Self {
        Self { lo, hi }
    }
}

impl<T: Clone> Interval<T> {
    pub fn singleton(value: T) -> Self {
        Self {
            lo: value.clone(),
            hi: value,
        }
    }
}

impl<T: PrimInt> Interval<T> {
    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    pub fn length(&self) -> T {
        self.hi - self.lo
    }

    pub fn contains(&self, value: T) -> bool {
        self.lo <= value && value <= self.hi
    }

    pub fn overlaps_with(&self, other: &Self) -> bool {
        self.hi.min(other.hi) >= self.lo.max(other.lo)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let intersection = Self::new(self.lo.max(other.lo), self.hi.min(other.hi));
        (!intersection.is_empty()).then_some(intersection)
    }

    pub fn union(&self, other: &Self) -> Self {
        Self::new(self.lo.min(other.lo), self.hi.max(other.hi))
    }

    pub fn difference(&self, other: &Self) -> Difference<T> {
        if self.is_empty() || other.is_empty() {
            return Difference::None;
        }

        let left = Self::new(self.lo, self.hi.min(other.lo - T::one()));
        let right = Self::new(self.lo.max(other.hi + T::one()), self.hi);

        match (left.is_empty(), right.is_empty()) {
            (true, true) => Difference::None,
            (true, false) => Difference::One(right),
            (false, true) => Difference::One(left),
            (false, false) if left == right => Difference::One(left),
            (false, false) => Difference::Two(left, right),
        }
    }

    pub fn split_at(&self, value: T) -> Difference<T> {
        self.difference(&Self::singleton(value))
    }
}

impl<T: Display> Display for Interval<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.lo, self.hi)
    }
}
