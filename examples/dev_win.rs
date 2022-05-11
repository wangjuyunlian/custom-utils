use custom_utils::r#mod::{Operator, WeekDays};
use custom_utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _logger = logger_default_debug("dev").unwrap();
    debug!("abc");
    info!("abc");
    let handle = daemon();

    // let now = time::OffsetDateTime::now_local().unwrap();
    // let date = now.clone().date();
    // let time = now.time();
    // let weekday = date.clone().weekday();
    // let (year, month, day) = date.to_calendar_date();
    // let next_day = date.next_day();

    let days = WeekDays::default_value(1)?.to_vec();
    println!("{:?}", days);

    if let Err(_e) = handle.await {}

    Ok(())
}
