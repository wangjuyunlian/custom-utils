pub mod data;

use crate::util_datetime::data::{AsData, DateTime, WeekDay};
use anyhow::{bail, Result};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, BitAnd, BitOr, BitOrAssign, Bound, RangeBounds, Shl, Sub};
use time::macros::date;

#[derive(Clone)]
pub struct MonthDays(u32);
#[derive(Clone)]
pub struct WeekDays(u8);
#[derive(Clone)]
pub struct Hours(u32);
#[derive(Clone)]
pub struct Minuters(u64);
#[derive(Clone)]
pub struct Seconds(u64);
impl Operator for Hours {
    const MIN: Self::ValTy = 0;
    const MAX: Self::ValTy = 23;
    const ONE: Self::ValTy = 1;
    const ZERO: Self::ValTy = 0;
    const DEFAULT_MAX: Self::ValTy = u32::MAX >> 8;
    type ValTy = u32;
    fn _default() -> Self {
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
    const ZERO: Self::ValTy = 0;
    const DEFAULT_MAX: Self::ValTy = u64::MAX >> 4;
    type ValTy = u64;
    fn _default() -> Self {
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
    const ZERO: Self::ValTy = 0;
    const DEFAULT_MAX: Self::ValTy = u64::MAX >> 4;
    type ValTy = u64;
    fn _default() -> Self {
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
    const ZERO: Self::ValTy = 0;
    const DEFAULT_MAX: Self::ValTy = u32::MAX << 1;
    type ValTy = u32;
    fn _default() -> Self {
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
    const ZERO: Self::ValTy = 0;
    const DEFAULT_MAX: Self::ValTy = u8::MAX << 1;
    type ValTy = u8;

    fn _default() -> Self {
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
    const ZERO: Self::ValTy;
    const DEFAULT_MAX: Self::ValTy;
    type ValTy: BitOr<Output = Self::ValTy>
        + Shl<Output = Self::ValTy>
        + Copy
        + BitOrAssign
        + Add<Output = Self::ValTy>
        + Sub<Output = Self::ValTy>
        + PartialOrd
        + AddAssign
        + BitAnd<Output = Self::ValTy>
        + Display;

    fn _default() -> Self;
    #[inline]
    fn default_value(val: impl AsData<Self::ValTy>) -> Self {
        let ins = Self::_default();
        ins.add(val)
    }
    #[inline]
    fn default_range<A: AsData<Self::ValTy>>(range: impl RangeBounds<A>) -> Result<Self> {
        let ins = Self::_default();
        ins.add_range(range)
    }
    #[inline]
    fn default_all() -> Self {
        let mut ins = Self::_default();
        ins._mut_val(Self::DEFAULT_MAX);
        ins
    }

    fn add_array(mut self, vals: &[impl AsData<Self::ValTy>]) -> Self {
        let mut val = self._val();
        for i in vals {
            val |= Self::ONE << i.as_data();
        }
        self._mut_val(val);
        self
    }
    fn add(mut self, index: impl AsData<Self::ValTy>) -> Self {
        let index = index.as_data();
        self._mut_val(self._val() | (Self::ONE << index));
        self
    }
    fn add_range<A: AsData<Self::ValTy>>(mut self, range: impl RangeBounds<A>) -> Result<Self> {
        let mut first = match range.start_bound() {
            Bound::Unbounded => Self::MIN,
            Bound::Included(first) => first.as_data(),
            Bound::Excluded(first) => first.as_data() + Self::ONE,
        };
        let end = match range.end_bound() {
            Bound::Unbounded => Self::MAX,
            Bound::Included(end) => end.as_data(),
            Bound::Excluded(end) => end.as_data() - Self::ONE,
        };
        if first > end {
            bail!("error:{} > {}", first, end);
        }
        let mut val = self._val();
        while first <= end {
            val |= Self::ONE << first;
            first += Self::ONE;
        }
        self._mut_val(val);
        Ok(self)
    }

    fn to_vec(&self) -> Vec<Self::ValTy> {
        let mut res = Vec::new();
        let val = self._val();
        let mut first = Self::MIN;
        while first <= Self::MAX {
            if (val & (Self::ONE << first)) > Self::ZERO {
                res.push(first);
            }
            first += Self::ONE;
        }
        res
    }
    fn contain<D: AsData<Self::ValTy>>(&self, index: D) -> bool {
        let index = index.as_data();
        let val = self._val();
        val & (Self::ONE << index) > Self::ZERO
    }
    fn min_self_next<D: AsData<Self::ValTy>>(
        &self,
        index: D,
    ) -> (Self::ValTy, Option<Self::ValTy>, Option<Self::ValTy>) {
        let self_val = if self.contain(index) {
            Some(index.as_data())
        } else {
            None
        };
        let min = self.min_val();
        let next = self.next(index);
        (min, self_val, next)
    }
    fn next<D: AsData<Self::ValTy>>(&self, index: D) -> Option<Self::ValTy> {
        let mut first = index.as_data() + Self::ONE;
        let val = self._val();
        while first <= Self::MAX {
            if (val & (Self::ONE << first)) > Self::ZERO {
                return Some(first);
            }
            first += Self::ONE;
        }
        None
        // first = Self::MIN;
        // while first <= index.as_data() {
        //     if (val & (Self::ONE << first)) > Self::ZERO {
        //         return Some(first);
        //     }
        //     first += Self::ONE;
        // }
        // unreachable!("it is a bug");
    }
    fn min_val(&self) -> Self::ValTy {
        let mut first = Self::MIN;
        let val = self._val();
        while first <= Self::MAX {
            if (val & (Self::ONE << first)) > Self::ZERO {
                return first;
            }
            first += Self::ONE;
        }
        unreachable!("it is a bug");
    }
    // fn find<D: AsData<Self::ValTy>>(&self, index: D, with_self: bool) -> Self::ValTy {
    //     let index = index.as_data();
    //     let val = self._val();
    //     let index = if with_self { index } else { index + Self::ONE };
    //     let mut first = index;
    //     while first <= Self::MAX {
    //         if (val & (Self::ONE << first)) > Self::ZERO {
    //             return first;
    //         }
    //         first += Self::ONE;
    //     }
    //     first = Self::MIN;
    //     while first < index {
    //         if (val & (Self::ONE << first)) > Self::ZERO {
    //             return first;
    //         }
    //         first += Self::ONE;
    //     }
    //     unreachable!("it is a bug");
    // }
    // #[inline]
    // fn check(index: Self::ValTy) -> Result<()> {
    //     if index >= Self::MIN && index <= Self::MAX {
    //         Ok(())
    //     } else {
    //         bail!("can't out of range {}..={} ", Self::MIN, Self::MAX);
    //     }
    // }
    fn _val(&self) -> Self::ValTy;
    fn _mut_val(&mut self, val: Self::ValTy);
}

#[derive(Debug, Clone)]
pub enum Days {
    MD(MonthDays),
    WD(WeekDays),
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
            days: self.month_days,
            hours: self.hours,
            minuters: self.minuters,
            seconds,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DayHourMinuterSecondConf {
    days: Days,
    hours: Hours,
    minuters: Minuters,
    seconds: Seconds,
}

impl DayHourMinuterSecondConf {
    pub fn next(&self) -> Result<u64> {
        let datetime = DateTime::default()?;
        let (hour_min, hour_self, hour_next) = self.hours.min_self_next(datetime.hour);
        let (minuter_min, minuter_self, minuter_next) =
            self.minuters.min_self_next(datetime.minuter);
        let (second_min, second_self, second_next) = self.seconds.min_self_next(datetime.second);
        match &self.days {
            Days::MD(monthdays) => {
                let (day_min, day_self, day_next) = monthdays.min_self_next(datetime.month_day);
            }
            Days::WD(weekdays) => {
                let (day_min, day_self, day_next) = weekdays.min_self_next(datetime.week_day);
            }
        };

        todo!()
    }
}

impl From<WeekDays> for Days {
    fn from(days: WeekDays) -> Self {
        Self::WD(days)
    }
}
impl From<MonthDays> for Days {
    fn from(days: MonthDays) -> Self {
        Self::MD(days)
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
impl Debug for Seconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == u64::MAX >> 4 {
            write!(f, "all seconds.")
        } else {
            write!(f, "seconds: {:?}.", self.to_vec())
        }
    }
}
impl Debug for Minuters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == u64::MAX >> 4 {
            write!(f, "all minuters.")
        } else {
            write!(f, "minuters: {:?}.", self.to_vec())
        }
    }
}
impl Debug for Hours {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == u32::MAX >> 8 {
            write!(f, "all hours.")
        } else {
            write!(f, "hours: {:?}.", self.to_vec())
        }
    }
}
impl Debug for MonthDays {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == u32::MAX << 1 {
            write!(f, "all month days.")
        } else {
            write!(f, "month days: {:?}.", self.to_vec())
        }
    }
}
impl Debug for WeekDays {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == u8::MAX << 1 {
            write!(f, "all week days.")
        } else {
            write!(f, "week days: {:?}.", self.to_vec())
        }
    }
}
#[cfg(test)]
mod test {
    use super::{DayConfBuilder, Hours, Minuters, MonthDays, Operator, Seconds, WeekDays};
    use crate::util_datetime::data::{Minuter, Second, WeekDay};

    #[test]
    fn test() -> anyhow::Result<()> {
        let conf = DayConfBuilder::default_week_days(
            WeekDays::default_value(WeekDay::W1).add_range(WeekDay::W3..WeekDay::W5)?,
        )
        .build_with_hours(Hours::default_all())
        .build_with_minuter_builder(Minuters::default_value(Minuter::M15))
        .build_with_second_builder(Seconds::default_value(Second::S0));
        println!("{:?}", conf);

        let conf = DayConfBuilder::default_week_days(WeekDays::default_all())
            .build_with_hours(Hours::default_all())
            .build_with_minuter_builder(Minuters::default_all())
            .build_with_second_builder(Seconds::default_all());
        println!("{:?}", conf);

        let month_days = MonthDays::default_all();
        println!("{:?}", month_days);
        Ok(())
    }
    #[test]
    fn test_week_days() -> anyhow::Result<()> {
        let wds = WeekDays::default_value(WeekDay::W1).add_range(WeekDay::W3..=WeekDay::W5)?;
        let days = wds.to_vec();
        assert_eq!(vec![1, 3, 4, 5], days);
        // assert_eq!(1, wds.find(WeekDay::W1, true));
        // assert_eq!(3, wds.find(WeekDay::W1, false));
        // assert_eq!(1, wds.find(WeekDay::W6, false));

        let wds = WeekDays::default_value(WeekDay::W1).add_array(&[WeekDay::W3, WeekDay::W5]);
        let days = wds.to_vec();
        assert_eq!(vec![1, 3, 5], days);
        // assert_eq!(1, wds.find(WeekDay::W1, true));
        // assert_eq!(3, wds.find(WeekDay::W1, false));
        // assert_eq!(1, wds.find(WeekDay::W6, false));

        let days = WeekDays::default_all().to_vec();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], days);
        assert!(WeekDays::default_range(WeekDay::W5..WeekDay::W5).is_err());
        assert!(WeekDays::default_range(WeekDay::W5..WeekDay::W1).is_err());
        let days = WeekDays::default_range(WeekDay::W5..WeekDay::W6)?.to_vec();
        assert_eq!(vec![5], days);

        Ok(())
    }
}
