// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_datetime::date::MockDateTime;
use icu_datetime::{options::style, DateTimeFormat};
use icu_locid_macros::langid;

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
    measure_memory_date_time_format();
}
