use anyhow::{bail, Result};
use log::debug;
use std::cell::Ref;
use std::collections::Bound;
use std::ops::{Deref, RangeBounds, RangeInclusive};

// use time::OffsetDateTime;
//
// fn test() {
//     let now = OffsetDateTime::now_local().unwrap();
//     let a = now.clone();
//     println!("{}   {}", now, a.date());
// }
/// S: size, total quantity; F: first index;
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct CommonTime<const S: usize, const F: usize>(usize);
#[derive(Debug, Clone)]
#[repr(transparent)]
struct CommonTimes<const S: usize, const F: usize>(Vec<usize>);
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct CommonTimesBuilder<const S: usize, const F: usize>([bool; S]);

// type Second = CommonTime<60, 0>;
// type Minuter = CommonTime<60, 0>;
// type Hour = CommonTime<24, 0>;
// type WeekDay = CommonTime<7, 1>;
// type MonthDay = CommonTime<31, 1>;

type Seconds = CommonTimes<60, 0>;
pub type SecondsBuilder = CommonTimesBuilder<60, 0>;

type Minuters = CommonTimes<60, 0>;
pub type MinutersBuilder = CommonTimesBuilder<60, 0>;

type Hours = CommonTimes<24, 0>;
pub type HoursBuilder = CommonTimesBuilder<24, 0>;

pub type WeekDays = CommonTimes<7, 1>;
pub type WeekDaysBuilder = CommonTimesBuilder<7, 1>;

pub type MonthDays = CommonTimes<31, 1>;
pub type MonthDaysBuilder = CommonTimesBuilder<31, 1>;

pub const EveryMonthDay: MonthDaysBuilder = MonthDaysBuilder::default_all();
pub const EveryWeekDay: WeekDaysBuilder = WeekDaysBuilder::default_all();
pub const EveryHour: HoursBuilder = HoursBuilder::default_all();
pub const EveryMinuter: MinutersBuilder = MinutersBuilder::default_all();
pub const EverySecond: SecondsBuilder = SecondsBuilder::default_all();

impl CommonTimesBuilder<60, 0> {
    pub fn default_zero() -> Self {
        let mut builder = Self::default();
        builder.0[0] = true;
        builder
    }
}
#[derive(Debug, Clone)]
pub enum Days {
    MonthDay(MonthDays),
    WeekDay(WeekDays),
}
// pub enum Every {
//     EveryMonthDay,
//     EveryWeekDay,
//     EveryHour,
//     EveryMinuter,
//     EverySecond,
// }

impl<const S: usize, const F: usize> CommonTime<S, F> {
    const MAX: usize = S + F;
    fn from_u32(&self, i: usize) -> Result<Self> {
        if i >= F && i <= Self::MAX {
            Ok(Self(i))
        } else {
            bail!("")
        }
    }
}

trait Check<const S: usize, const F: usize> {
    const MAX: usize = S + F;
    fn check(&self, i: usize) -> bool {
        if i >= F && i <= Self::MAX {
            true
        } else {
            false
        }
    }
}
impl<const S: usize, const F: usize> Check<S, F> for CommonTimesBuilder<S, F> {}
impl<const S: usize, const F: usize> Check<S, F> for CommonTimes<S, F> {}
impl<const S: usize, const F: usize> Check<S, F> for CommonTime<S, F> {}
impl<const S: usize, const F: usize> From<CommonTimesBuilder<S, F>> for CommonTimes<S, F> {
    fn from(builder: CommonTimesBuilder<S, F>) -> Self {
        let mut data = Vec::with_capacity(S);
        for (index, val) in builder.0.into_iter().enumerate() {
            if val {
                data.push(index + F);
            }
        }
        Self(data)
    }
}
impl<const S: usize, const F: usize> CommonTimesBuilder<S, F> {
    #[inline]
    const fn default() -> Self {
        Self([false; S])
    }
    #[inline]
    pub fn default_value(val: usize) -> Result<Self> {
        let mut ins = Self::default();
        ins.add(val)
    }
    #[inline]
    pub fn default_range(range: impl RangeBounds<usize>) -> Result<Self> {
        let mut ins = Self::default();
        ins.add_range(range)
    }
    #[inline]
    pub const fn default_all() -> Self {
        Self([true; S])
    }
    pub fn add_range(mut self, range: impl RangeBounds<usize>) -> Result<Self> {
        let first = match range.start_bound() {
            Bound::Unbounded => {
                bail!("not support unbounder!");
            }
            Bound::Included(first) => *first,
            Bound::Excluded(first) => *first + 1,
        };
        let end = match range.end_bound() {
            Bound::Unbounded => {
                bail!("not support unbounder!");
            }
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end - 1,
        };
        if !self.check(first) || !self.check(end) {
            // todo 优化
            bail!("can't out of range {}..={} ", F, Self::MAX);
        }
        // println!("{}-{}", first, end);
        let range = RangeInclusive::new(first, end);
        for i in range {
            // println!("{}", i);
            self.0[i - F] = true;
        }
        Ok(self)
    }
    pub fn add(mut self, index: usize) -> Result<Self> {
        if !self.check(index) {
            // todo 优化
            bail!("can't out of range {}..={} ", F, Self::MAX);
        }
        self.0[index - F] = true;
        Ok(self)
    }
}
pub struct WeekDayConfBuilder(WeekDaysBuilder);
impl WeekDayConfBuilder {
    pub fn default_value(val: usize) -> Result<Self> {
        Ok(Self(WeekDaysBuilder::default_value(val)?))
    }
    pub fn default_range(range: impl RangeBounds<usize>) -> Result<Self> {
        Ok(Self(WeekDaysBuilder::default_range(range)?))
    }
    pub fn default_all() -> Self {
        Self(WeekDaysBuilder::default_all())
    }
    pub fn add(mut self, index: usize) -> Result<Self> {
        self.0 = self.0.add(index)?;
        Ok(self)
    }
    pub fn add_range(mut self, range: impl RangeBounds<usize>) -> Result<Self> {
        self.0 = self.0.add_range(range)?;
        Ok(self)
    }
    pub fn build(self) -> MonthDayHourConfBuilder {
        MonthDayHourConfBuilder {
            month_days: self.0.into(),
            hours_builder: HoursBuilder::default(),
        }
    }
    pub fn build_with_hour_builder(self, hours_builder: HoursBuilder) -> MonthDayHourConfBuilder {
        MonthDayHourConfBuilder {
            month_days: self.0.into(),
            hours_builder,
        }
    }
}

pub type MonthDayConfBuilder = MonthDaysBuilder;
impl MonthDayConfBuilder {
    fn build(self) -> MonthDayHourConfBuilder {
        MonthDayHourConfBuilder {
            month_days: self.into(),
            hours_builder: HoursBuilder::default(),
        }
    }
    pub fn build_with_hour_builder(self, hours_builder: HoursBuilder) -> MonthDayHourConfBuilder {
        MonthDayHourConfBuilder {
            month_days: self.into(),
            hours_builder,
        }
    }
}
pub struct MonthDayHourConfBuilder {
    month_days: Days,
    hours_builder: HoursBuilder,
}
impl MonthDayHourConfBuilder {
    fn build(self) -> MonthDayHourMinuterConfBuilder {
        MonthDayHourMinuterConfBuilder {
            month_days: self.month_days,
            hours: self.hours_builder.into(),
            minuters_builder: MinutersBuilder::default(),
        }
    }
    pub fn build_with_minuter_builder(
        self,
        minuters_builder: MinutersBuilder,
    ) -> MonthDayHourMinuterConfBuilder {
        MonthDayHourMinuterConfBuilder {
            month_days: self.month_days,
            hours: self.hours_builder.into(),
            minuters_builder: minuters_builder,
        }
    }
}
pub struct MonthDayHourMinuterConfBuilder {
    month_days: Days,
    hours: Hours,
    minuters_builder: MinutersBuilder,
}
impl MonthDayHourMinuterConfBuilder {
    fn build(self) -> MonthDayHourMinuterSecondConfBuilder {
        MonthDayHourMinuterSecondConfBuilder {
            month_days: self.month_days,
            hours: self.hours,
            minuters: self.minuters_builder.into(),
            seconds_builder: SecondsBuilder::default(),
        }
    }
    pub fn build_with_second_builder(
        self,
        seconds_builder: SecondsBuilder,
    ) -> MonthDayHourMinuterSecondConfBuilder {
        MonthDayHourMinuterSecondConfBuilder {
            month_days: self.month_days,
            hours: self.hours,
            minuters: self.minuters_builder.into(),
            seconds_builder,
        }
    }
}

pub struct MonthDayHourMinuterSecondConfBuilder {
    month_days: Days,
    hours: Hours,
    minuters: Minuters,
    seconds_builder: SecondsBuilder,
}
impl MonthDayHourMinuterSecondConfBuilder {
    pub fn build(self) -> MonthDayHourMinuterSecondConf {
        MonthDayHourMinuterSecondConf {
            month_days: self.month_days,
            hours: self.hours,
            minuters: self.minuters,
            seconds: self.seconds_builder.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonthDayHourMinuterSecondConf {
    month_days: Days,
    hours: Hours,
    minuters: Minuters,
    seconds: Seconds,
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
impl From<MonthDaysBuilder> for Days {
    fn from(builder: MonthDaysBuilder) -> Self {
        Self::MonthDay(builder.into())
    }
}
impl From<WeekDaysBuilder> for Days {
    fn from(builder: WeekDaysBuilder) -> Self {
        Self::WeekDay(builder.into())
    }
}
#[cfg(test)]
mod test {
    use crate::util_datetime::{
        HoursBuilder, MinutersBuilder, MonthDayHourConfBuilder, SecondsBuilder, WeekDayConfBuilder,
    };

    #[test]
    fn test() -> anyhow::Result<()> {
        let conf = WeekDayConfBuilder::default_value(1)?
            .add_range(3..5)?
            .build_with_hour_builder(HoursBuilder::default_value(9)?)
            .build_with_minuter_builder(MinutersBuilder::default_value(15)?)
            .build_with_second_builder(SecondsBuilder::default_zero())
            .build();
        println!("{:?}", conf);
        Ok(())
    }
}
