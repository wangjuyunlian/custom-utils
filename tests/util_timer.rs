use anyhow::Result;
use custom_utils::{Hour, Hours, Operator, Second, Seconds, WeekDay, WeekDays};

#[test]
fn test_week_days() -> Result<()> {
    let wds = WeekDays::default_value(WeekDay::W1).add_range(WeekDay::W3..=WeekDay::W5)?;
    let days = wds.to_vec();
    assert_eq!(vec![1, 3, 4, 5], days);

    let wds = WeekDays::default_value(WeekDay::W1).add_array(&[WeekDay::W3, WeekDay::W5]);
    let days = wds.to_vec();
    assert_eq!(vec![1, 3, 5], days);

    let days = WeekDays::default_all().to_vec();
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], days);
    assert!(WeekDays::default_range(WeekDay::W5..WeekDay::W5).is_err());
    assert!(WeekDays::default_range(WeekDay::W5..WeekDay::W1).is_err());
    let days = WeekDays::default_range(WeekDay::W5..WeekDay::W6)?.to_vec();
    assert_eq!(vec![5], days);

    Ok(())
}

#[test]
fn test_contain() -> Result<()> {
    let all_seconds = Seconds::default_all();
    assert!(all_seconds.contain(Second::S0));
    assert!(all_seconds.contain(Second::S30));
    assert!(all_seconds.contain(Second::S59));
    let some_seconds =
        Seconds::default_range(Second::S1..Second::S30)?.add_range(Second::S31..=Second::S58)?;
    assert!(!some_seconds.contain(Second::S59));
    assert!(!some_seconds.contain(Second::S0));
    assert!(!some_seconds.contain(Second::S30));

    let all_hours = Hours::default_all();
    assert!(all_hours.contain(Hour::H10));
    assert!(all_hours.contain(Hour::H0));
    assert!(all_hours.contain(Hour::H23));
    let some_hours = Hours::default_range(Hour::H1..Hour::H10)?.add_range(Hour::H11..=Hour::H22)?;
    assert!(!some_hours.contain(Hour::H10));
    assert!(!some_hours.contain(Hour::H0));
    assert!(!some_hours.contain(Hour::H23));

    let all_weeks = WeekDays::default_all();
    assert!(all_weeks.contain(WeekDay::W3));
    assert!(all_weeks.contain(WeekDay::W1));
    assert!(all_weeks.contain(WeekDay::W7));
    let some_weeks =
        WeekDays::default_range(WeekDay::W2..WeekDay::W3)?.add_range(WeekDay::W4..=WeekDay::W6)?;
    assert!(!some_weeks.contain(WeekDay::W3));
    assert!(!some_weeks.contain(WeekDay::W1));
    assert!(!some_weeks.contain(WeekDay::W7));
    Ok(())
}
