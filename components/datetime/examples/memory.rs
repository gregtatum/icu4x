use dhat::{Dhat, DhatAlloc};
use icu_datetime::date::MockDateTime;
use icu_datetime::{options::style, DateTimeFormat};
use icu_locid_macros::langid;

#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn measure_memory_date_time_format() {
    let provider = icu_testdata::get_provider();
    let lid = langid!("en");

    let date = "2020-10-14T13:21:50"
        .parse::<MockDateTime>()
        .expect("Failed to parse a date time.");

    let options = style::Bag {
        date: Some(style::Date::Medium),
        time: Some(style::Time::Short),
        ..Default::default()
    }
    .into();

    let formatter = DateTimeFormat::try_new(lid, &provider, &options)
        .expect("Failed to create a DateTimeFormat");

    let formatted_date = formatter.format(&date);

    assert_eq!(formatted_date.to_string(), "Oct 14, 2020, 1:21 PM");
}

fn main() {
    let _dhat = Dhat::start_heap_profiling();
    measure_memory_date_time_format();
}
