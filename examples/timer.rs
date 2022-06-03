use custom_utils::logger::logger_debug_default;
use custom_utils::timer::*;
use std::time::Duration;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _log = logger_debug_default();
    let conf = DayHourMinuterSecondConf::default_week_days(WeekDays::default_value(W6))
        .build_with_hours(Hours::default_all())
        .build_with_minuter(Minuters::default_array(&[M0, M10, M20, M30, M40, M50]))
        .build_with_second(Seconds::default_array(&[S0, S30]));

    // let next_seconds = conf.next()?;

    let handle = tokio::spawn(async move {
        loop {
            let off_seconds = conf.next().unwrap();
            println!("next seconds: {}", off_seconds);
            tokio::time::sleep(Duration::from_secs(off_seconds)).await;
            println!("{:?}", OffsetDateTime::now_local().unwrap());
        }
    });
    handle.await.unwrap();
    Ok(())
}
