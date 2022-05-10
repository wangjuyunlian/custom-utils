use crate::util_datetime::MonthDaysBuilder;
use anyhow::{bail, Result};
use std::fmt::Display;
use std::ops::{Add, AddAssign, BitOr, BitOrAssign, Bound, RangeBounds, RangeInclusive, Shl, Sub};
use std::process::Output;
use time::Weekday;
#[derive(Debug, Clone)]
pub struct MonthDays(u32);
#[derive(Debug, Clone)]
pub struct WeekDays(u8);
#[derive(Debug, Clone)]
pub struct Hours(u32);
#[derive(Debug, Clone)]
pub struct Minuters(u64);
#[derive(Debug, Clone)]
pub struct Seconds(u64);
impl Operator for Hours {
    const MIN: Self::ValTy = 0;
    const MAX: Self::ValTy = 23;
    const ONE: Self::ValTy = 1;
    const DEFAULT_MAX: Self::ValTy = u32::MAX >> 8;
    type ValTy = u32;
    fn default() -> Self {
        Self(0)
    }
    fn _val(&self) -> Self::ValTy {
        self.0
    }
    fn _mut_val(&mut self, val: Self::ValTy) {
        self.0 = val
    }
}
impl Operator for Seconds {
    const MIN: Self::ValTy = 0;
    const MAX: Self::ValTy = 59;
    const ONE: Self::ValTy = 1;
    const DEFAULT_MAX: Self::ValTy = u64::MAX >> 4;
    type ValTy = u64;
    fn default() -> Self {
        Self(0)
    }
    fn _val(&self) -> Self::ValTy {
        self.0
    }
    fn _mut_val(&mut self, val: Self::ValTy) {
        self.0 = val
    }
}
impl Operator for Minuters {
    const MIN: Self::ValTy = 0;
    const MAX: Self::ValTy = 59;
    const ONE: Self::ValTy = 1;
    const DEFAULT_MAX: Self::ValTy = u64::MAX >> 4;
    type ValTy = u64;
    fn default() -> Self {
        Self(0)
    }
    fn _val(&self) -> Self::ValTy {
        self.0
    }
    fn _mut_val(&mut self, val: Self::ValTy) {
        self.0 = val
    }
}

impl Operator for MonthDays {
    const MIN: Self::ValTy = 1;
    const MAX: Self::ValTy = 31;
    const ONE: Self::ValTy = 1;
    const DEFAULT_MAX: Self::ValTy = u32::MAX << 1;
    type ValTy = u32;
    fn default() -> Self {
        Self(0)
    }

    fn _val(&self) -> Self::ValTy {
        self.0
    }
    fn _mut_val(&mut self, val: Self::ValTy) {
        self.0 = val
    }
}
impl Operator for WeekDays {
    const MIN: Self::ValTy = 1;
    const MAX: Self::ValTy = 7;
    const ONE: Self::ValTy = 1;
    const DEFAULT_MAX: Self::ValTy = u8::MAX << 1;
    type ValTy = u8;

    fn default() -> Self {
        Self(0)
    }

    fn _val(&self) -> Self::ValTy {
        self.0
    }
    fn _mut_val(&mut self, val: Self::ValTy) {
        self.0 = val
    }
}

pub trait Operator: Sized {
    const MIN: Self::ValTy;
    const MAX: Self::ValTy;
    const ONE: Self::ValTy;
    const DEFAULT_MAX: Self::ValTy;
    type ValTy: BitOr<Output = Self::ValTy>
        + Shl<Output = Self::ValTy>
        + Copy
        + BitOrAssign
        + Add<Output = Self::ValTy>
        + Sub<Output = Self::ValTy>
        + PartialOrd
        + AddAssign
        + Display;

    #[inline]
    fn default() -> Self;
    #[inline]
    fn default_value(val: Self::ValTy) -> Result<Self> {
        let mut ins = Self::default();
        ins.add(val)
    }
    #[inline]
    fn default_range(range: impl RangeBounds<Self::ValTy>) -> Result<Self> {
        let mut ins = Self::default();
        ins.add_range(range)
    }
    #[inline]
    fn default_all() -> Self {
        let mut ins = Self::default();
        ins._mut_val(Self::DEFAULT_MAX);
        ins
    }

    fn add_array(mut self, vals: &[Self::ValTy]) -> Result<Self> {
        for i in vals {
            Self::check(*i)?;
        }
        let mut val = self._val();
        for i in vals {
            val |= (Self::ONE << *i);
        }
        self._mut_val(val);
        Ok(self)
    }
    fn add(mut self, index: Self::ValTy) -> Result<Self> {
        Self::check(index)?;
        self._mut_val(self._val() | (Self::ONE << index));
        Ok(self)
    }
    fn add_range(mut self, range: impl RangeBounds<Self::ValTy>) -> Result<Self> {
        let mut first = match range.start_bound() {
            Bound::Unbounded => {
                bail!("not support unbounder!");
            }
            Bound::Included(first) => *first,
            Bound::Excluded(first) => (*first) + Self::ONE,
        };
        let end = match range.end_bound() {
            Bound::Unbounded => {
                bail!("not support unbounder!");
            }
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end - Self::ONE,
        };
        Self::check(first)?;
        Self::check(end)?;
        // let range = RangeInclusive::new(first, end);
        let mut val = self._val();
        while first > end {
            val |= (Self::ONE << first);
            first += Self::ONE;
        }
        self._mut_val(val);
        Ok(self)
    }
    #[inline]
    fn check(index: Self::ValTy) -> Result<()> {
        if index >= Self::MIN && index <= Self::MAX {
            Ok(())
        } else {
            bail!("can't out of range {}..={} ", Self::MIN, Self::MAX);
        }
    }
    fn _val(&self) -> Self::ValTy;
    fn _mut_val(&mut self, val: Self::ValTy);
}

#[derive(Debug, Clone)]
pub enum Days {
    MonthDay(MonthDays),
    WeekDay(WeekDays),
}
pub struct DayConfBuilder(Days);
impl DayConfBuilder {
    pub fn default_month_days(days: MonthDays) -> Self {
        Self(days.into())
    }
    pub fn default_week_days(days: WeekDays) -> Self {
        Self(days.into())
    }
    pub fn build_with_hours(self, hours: Hours) -> DayHourConfBuilder {
        DayHourConfBuilder {
            month_days: self.0.into(),
            hours,
        }
    }
}
pub struct DayHourConfBuilder {
    month_days: Days,
    hours: Hours,
}
impl DayHourConfBuilder {
    pub fn build_with_minuter_builder(self, minuters: Minuters) -> DayHourMinuterConfBuilder {
        DayHourMinuterConfBuilder {
            month_days: self.month_days,
            hours: self.hours,
            minuters,
        }
    }
}
pub struct DayHourMinuterConfBuilder {
    month_days: Days,
    hours: Hours,
    minuters: Minuters,
}
impl DayHourMinuterConfBuilder {
    pub fn build_with_second_builder(self, seconds: Seconds) -> DayHourMinuterSecondConf {
        DayHourMinuterSecondConf {
            month_days: self.month_days,
            hours: self.hours,
            minuters: self.minuters,
            seconds,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DayHourMinuterSecondConf {
    month_days: Days,
    hours: Hours,
    minuters: Minuters,
    seconds: Seconds,
}

#[inline]
fn weekday_to_usize(day: Weekday) -> u8 {
    match day {
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
        Weekday::Sunday => 7,
    }
}

impl From<WeekDays> for Days {
    fn from(days: WeekDays) -> Self {
        Self::WeekDay(days)
    }
}
impl From<MonthDays> for Days {
    fn from(days: MonthDays) -> Self {
        Self::MonthDay(days)
    }
}
impl From<MonthDays> for DayConfBuilder {
    fn from(builder: MonthDays) -> Self {
        Self(builder.into())
    }
}
impl From<WeekDays> for DayConfBuilder {
    fn from(builder: WeekDays) -> Self {
        Self(builder.into())
    }
}
#[cfg(test)]
mod test {
    use crate::util_datetime2::{DayConfBuilder, Hours, Minuters, Operator, Seconds, WeekDays};

    #[test]
    fn test() -> anyhow::Result<()> {
        let conf = DayConfBuilder::default_week_days(WeekDays::default_value(1)?.add_range(3..5)?)
            .build_with_hours(Hours::default_all())
            .build_with_minuter_builder(Minuters::default_value(15)?)
            .build_with_second_builder(Seconds::default_value(0)?);
        println!("{:?}", conf);
        Ok(())
    }
}
