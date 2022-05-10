use custom_utils::*;

#[tokio::main]
async fn main() {
    let _logger = logger_default_debug("dev").unwrap();
    debug!("abc");
    info!("abc");
    let handle = daemon();

    let now = time::OffsetDateTime::now_local().unwrap();
    let date = now.clone().date();
    let time = now.time();
    let weekday = date.clone().weekday();
    let (year, month, day) = date.to_calendar_date();
    let next_day = date.next_day();


    time.clone().hour()

    if let Err(_e) = handle.await {}
}
