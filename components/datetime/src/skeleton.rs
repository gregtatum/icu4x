// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

// TODO - Remove this when ready to land.
#![allow(dead_code)]

use crate::fields::{self, Field, FieldLength, FieldSymbol, LengthError, SymbolError};
use crate::options::components;
use std::{borrow::Cow, convert::TryFrom};

#[derive(Debug, PartialEq)]
struct FieldIndex(usize);

#[derive(Debug, PartialEq)]
pub struct Skeleton {
    fields: Vec<Field>,
    pattern_string: Cow<'static, str>,
    fields_by_type: FieldsByType,
}

impl Skeleton {
    pub fn new(pattern_string: Cow<'static, str>) -> Skeleton {
        Skeleton {
            fields: Vec::new(),
            pattern_string,
            fields_by_type: FieldsByType::new(),
        }
    }

    pub fn add_field(&mut self, symbol: FieldSymbol, length: u8) -> Result<(), SkeletonError> {
        self.fields_by_type
            .set(&symbol, FieldIndex(self.fields.len()));

        self.fields.push(Field {
            symbol,
            length: FieldLength::try_from(length)?,
        });

        Ok(())
    }

    fn get_field(&self, field: &Option<FieldIndex>) -> Option<&Field> {
        field
            .as_ref()
            .map(|FieldIndex(index)| self.fields.get(*index).expect("Expected to find a field."))
    }

    fn get_year(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.year)
    }
    fn get_month(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.month)
    }
    fn get_day(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.day)
    }
    fn get_weekday(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.weekday)
    }
    fn get_day_period(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.day_period)
    }
    fn get_hour(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.hour)
    }
    fn get_minute(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.minute)
    }
    fn get_second(&self) -> Option<&Field> {
        self.get_field(&self.fields_by_type.second)
    }
}

pub type SkeletonStringTuple = (Cow<'static, str>, Cow<'static, str>);

impl TryFrom<&SkeletonStringTuple> for Skeleton {
    type Error = SkeletonError;
    fn try_from(tuple: &SkeletonStringTuple) -> Result<Self, Self::Error> {
        let (skeleton_string, pattern_string) = tuple;
        let mut vec = Vec::new();
        let mut skeleton = Skeleton::new(pattern_string.clone());
        for byte in skeleton_string.bytes() {
            match FieldSymbol::try_from(byte) {
                Ok(symbol) => vec.push(symbol),
                Err(err) => match byte {
                    // The short generic non-location format, e.g Pacific Time, or PT
                    b'v'
                    // Flexible day periods
                    | b'B'
                    // Era
                    | b'G'
                    // Quarter
                    | b'Q'
                    // Zone
                    | b'Z'
                    // Week of Month (numeric)
                    | b'W'
                    // Week of Year (numeric)
                    | b'w'
                    // "-count-*" and "-alt-variant"
                    | b'-' => {
                        return Err(SkeletonError::UnimplementedField(byte as char))
                    }
                    _ => return Err(err.into()),
                },
            };
        }

        let mut length: u8 = 0;
        let mut iter = vec.drain(..).peekable();
        while let Some(symbol) = iter.next() {
            length += 1;
            match iter.peek() {
                Some(next_symbol) => {
                    if *next_symbol != symbol {
                        skeleton.add_field(symbol, length)?;
                        length = 0;
                    }
                }
                None => {
                    skeleton.add_field(symbol, length)?;
                }
            }
        }

        Ok(skeleton)
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldsByType {
    year: Option<FieldIndex>,
    month: Option<FieldIndex>,
    day: Option<FieldIndex>,
    weekday: Option<FieldIndex>,
    day_period: Option<FieldIndex>,
    hour: Option<FieldIndex>,
    minute: Option<FieldIndex>,
    second: Option<FieldIndex>,
}

impl FieldsByType {
    fn new() -> FieldsByType {
        FieldsByType {
            year: None,
            month: None,
            day: None,
            weekday: None,
            day_period: None,
            hour: None,
            minute: None,
            second: None,
        }
    }

    fn set(&mut self, symbol: &FieldSymbol, index: FieldIndex) {
        match symbol {
            FieldSymbol::Year(_) => self.year = Some(index),
            FieldSymbol::Month(_) => self.month = Some(index),
            FieldSymbol::Day(_) => self.day = Some(index),
            FieldSymbol::Weekday(_) => self.weekday = Some(index),
            FieldSymbol::DayPeriod(_) => self.day_period = Some(index),
            FieldSymbol::Hour(_) => self.hour = Some(index),
            FieldSymbol::Minute => self.minute = Some(index),
            FieldSymbol::Second(_) => self.second = Some(index),
        }
    }
}

impl From<FieldSymbol> for char {
    fn from(symbol: FieldSymbol) -> Self {
        match symbol {
            FieldSymbol::Year(year) => match year {
                fields::Year::Calendar => 'y',
                fields::Year::WeekOf => 'Y',
            },
            FieldSymbol::Month(month) => match month {
                fields::Month::Format => 'M',
                fields::Month::StandAlone => 'L',
            },
            FieldSymbol::Day(day) => match day {
                fields::Day::DayOfMonth => 'd',
                fields::Day::DayOfYear => 'D',
                fields::Day::DayOfWeekInMonth => 'F',
                fields::Day::ModifiedJulianDay => 'g',
            },
            FieldSymbol::Weekday(weekday) => match weekday {
                fields::Weekday::Format => 'E',
                fields::Weekday::Local => 'e',
                fields::Weekday::StandAlone => 'c',
            },
            FieldSymbol::DayPeriod(dayperiod) => match dayperiod {
                fields::DayPeriod::AmPm => 'a',
                fields::DayPeriod::NoonMidnight => 'b',
            },
            FieldSymbol::Hour(hour) => match hour {
                fields::Hour::H11 => 'K',
                fields::Hour::H12 => 'h',
                fields::Hour::H23 => 'H',
                fields::Hour::H24 => 'k',
            },
            FieldSymbol::Minute => 'm',
            FieldSymbol::Second(second) => match second {
                fields::Second::Second => 's',
                fields::Second::FractionalSecond => 'S',
                fields::Second::Millisecond => 'A',
            },
        }
    }
}

impl From<&Skeleton> for String {
    fn from(skeleton: &Skeleton) -> Self {
        let mut string = String::new();
        for field in skeleton.fields.iter() {
            let count = field.length as usize;
            for _ in 0..count {
                string.push(char::from(field.symbol));
            }
        }
        string
    }
}

#[derive(Debug)]
pub enum SkeletonError {
    FieldLengthTooLong,
    SymbolUnknown(char),
    UnimplementedField(char),
}

impl From<LengthError> for SkeletonError {
    fn from(_: LengthError) -> Self {
        SkeletonError::FieldLengthTooLong
    }
}

impl From<SymbolError> for SkeletonError {
    fn from(symbol_error: SymbolError) -> Self {
        let SymbolError::Unknown(ch) = symbol_error;
        SkeletonError::SymbolUnknown(ch as char)
    }
}

// The following scalar values are for testing the suitability of a skeleton's field for the
// given input. Per UTS 35, the better the fit of a pattern, the "lower the distance". In this
// implementation each distance type is separated by an order of magnitiude. This magnittude needs
// to be at minimum a multiple of the max length of fields. As of CLDR 38 (2021-01), the max length
// of a skeleton in the "availableFormats" contained a total of 4 fields. The scores use a multiple
// of 10, as a number that will contain the range, and be easy to reason with.

const MAX_FIELDS: u16 = 10;

// Per the skeleton matching algorithm:
// https://unicode.org/reports/tr35/tr35-dates.html#Matching_Skeletons

// > 1. "Input skeleton symbols" are replaced with the best match for a given locale.
// >   - Hour: j → {H, k, h, K} + {a, b, B}
// >           J → {H, k, h, K}
// >           C → j + day period

// The components::Bag does not support step 1

// > 2. For fields with symbols representing the same type (year, month, day, etc):
// >   A. Most symbols have a small distance from each other.
// >     - Months: M ≅ L           (9 ≅ 9)  conjuction, vs standalone
// >       Week:   E ≅ c           (Tue ≅ 2)
// >       Period: a ≅ b ≅ B       (am. ≅ mid. ≅ at night)
// >       Hour:   H ≅ k ≅ h ≅ K   (23, 24, 12, 11)

const NO_DISTANCE: u16 = 0;

// B. Width differences among fields, other than those marking text vs numeric, are given small
// distance from each other.
// - MMM ≅ MMMM  (Sep ≅ September)
//   MM ≅ M      (09 ≅ 9)
const WIDTH_MISMATCH_DISTANCE: u16 = 1;

// C. Numeric and text fields are given a larger distance from each other.
// - MMM ≈ MM    (Sep ≈ 09)
//   MMM
const TEXT_VS_NUMERIC_DISTANCE: u16 = 10;

// D. Symbols representing substantial differences (week of year vs week of month) are given much
// larger a distances from each other.
// - d ≋ D;     (12 ≋ 345) Day of month vs Day of year
const SUBSTANTIAL_DIFFERENCES_DISTANCE: u16 = 100;

// Finally, missing symbols are the biggest distance.
const MISSING_SYMBOL_DISTANCE: u16 = 1000;

pub enum BestSkeleton {
    AllFieldsMatch(Skeleton),
    MissingFields(Skeleton),
    NoMatch,
}

pub fn get_best_skeleton(
    skeletons: impl Iterator<Item = Skeleton>,
    components: &components::Bag,
) -> BestSkeleton {
    let mut closest_distance: u16 = u16::MAX;
    let mut closest_skeleton: Option<Skeleton> = None;
    for skeleton in skeletons {
        debug_assert!(
            skeleton.fields.len() <= MAX_FIELDS as usize,
            "The distance mechanism assumes skeletons are less than MAX_FIELDS in length."
        );
        debug_assert!(
            MAX_FIELDS * MISSING_SYMBOL_DISTANCE < u16::MAX,
            "The distance should fit into a u16."
        );

        let mut distance: u16 = 0;

        if let Some(year) = components.year {
            distance += match skeleton.get_year() {
                Some(skeleton_field) => {
                    match skeleton_field.symbol {
                        FieldSymbol::Year(fields::Year::Calendar) => match skeleton_field.length {
                            FieldLength::TwoDigit => match year {
                                components::Numeric::Numeric => WIDTH_MISMATCH_DISTANCE,
                                components::Numeric::TwoDigit => NO_DISTANCE,
                            },
                            FieldLength::One
                            | FieldLength::Abbreviated
                            | FieldLength::Wide
                            | FieldLength::Narrow
                            | FieldLength::Six => match year {
                                components::Numeric::Numeric => NO_DISTANCE,
                                components::Numeric::TwoDigit => WIDTH_MISMATCH_DISTANCE,
                            },
                        },
                        // As of CLDR 38 (2021-01) there were no Week of Year ("Y") fields in skeletons.
                        // So in theory, this branch will never be hit.
                        FieldSymbol::Year(fields::Year::WeekOf) => SUBSTANTIAL_DIFFERENCES_DISTANCE,
                        _ => panic!("Logic error from symbol matching."),
                    }
                }
                None => {
                    // MISSING_SYMBOL_DISTANCE
                    continue;
                }
            }
        }

        if let Some(month) = components.month {
            distance += match skeleton.get_month() {
                Some(skeleton_field) => match skeleton_field.symbol {
                    FieldSymbol::Month(fields::Month::Format) => {
                        if month.matches_field_length(skeleton_field.length) {
                            NO_DISTANCE
                        } else {
                            WIDTH_MISMATCH_DISTANCE
                        }
                    }
                    // As of CLDR 38 (2021-01) there were no month stand alone fields in skeletons.
                    // So in theory, this branch will never be hit.
                    FieldSymbol::Month(fields::Month::StandAlone) => {
                        SUBSTANTIAL_DIFFERENCES_DISTANCE
                    }
                    _ => panic!("Logic error from symbol matching."),
                },
                None => MISSING_SYMBOL_DISTANCE,
            }
        }

        if distance < closest_distance {
            closest_skeleton = Some(skeleton);
            closest_distance = distance;
        }
    }

    match closest_skeleton {
        Some(skeleton) => {
            if closest_distance < MISSING_SYMBOL_DISTANCE {
                BestSkeleton::AllFieldsMatch(skeleton)
            } else {
                BestSkeleton::MissingFields(skeleton)
            }
        }
        None => BestSkeleton::NoMatch,
    }
}

#[cfg(test)]
mod test {

    use icu_locid_macros::langid;
    use icu_provider::{DataRequest, ResourceOptions, ResourcePath};

    use crate::{
        fields::{Day, Month, Weekday},
        provider::{self, gregory::DatesV1, key::GREGORY_V1},
    };
    use icu_provider::DataProvider;

    use super::*;

    const STRING_SKELETONS: [&str; 95] = [
        "Bh",
        "Bhm",
        "Bhms",
        "d",
        "E",
        "EBhm",
        "EBhms",
        "Ed",
        "Ehm",
        "EHm",
        "Ehms",
        "EHms",
        "Gy",
        "GyMMM",
        "GyMMMd",
        "GyMMMEd",
        "h",
        "H",
        "hm",
        "Hm",
        "hms",
        "Hms",
        "hmsv",
        "Hmsv",
        "hmv",
        "Hmv",
        "M",
        "Md",
        "MEd",
        "MMM",
        "MMMd",
        "MMMEd",
        "MMMMd",
        "MMMMEd",
        "ms",
        "y",
        "yM",
        "yMd",
        "yMEd",
        "yMMM",
        "yMMMd",
        "yMMMEd",
        "yMMMM",
        "yQQQ",
        "yQQQQ",
        "MMdd",
        "yMM",
        "GyMMMM",
        "GyMMMMd",
        "GyMMMMEd",
        "MMMM",
        "MMMMdd",
        "yMMMMd",
        "yMMMMEd",
        "MMd",
        "hmsvvvv",
        "Hmsvvvv",
        "hmvvvv",
        "Hmvvvv",
        "yQ",
        "yMMdd",
        "MMMdd",
        "mmss",
        "HHmmZ",
        "yMMMMEEEEd",
        "yMMMMccccd",
        "EEEEd",
        "GyMMMEEEEd",
        "MEEEEd",
        "MMMEEEEd",
        "yMEEEEd",
        "yMMMEEEEd",
        "HHmmss",
        "Mdd",
        "HHmm",
        "GyM",
        "MMMMEEEEd",
        "MMMMW-count-one",
        "MMMMW-count-other",
        "yw-count-one",
        "yw-count-other",
        "MMMMW-count-zero",
        "MMMMW-count-two",
        "MMMMW-count-few",
        "MMMMW-count-many",
        "yw-count-zero",
        "yw-count-two",
        "yw-count-few",
        "yw-count-many",
        "Md-alt-variant",
        "MEd-alt-variant",
        "MMdd-alt-variant",
        "yM-alt-variant",
        "yMd-alt-variant",
        "yMEd-alt-variant",
    ];

    fn get_skeletons<'a>(
        data: &'a provider::gregory::DatesV1,
    ) -> impl Iterator<Item = Result<Skeleton, SkeletonError>> + 'a {
        data.patterns
            .date_time
            .available_formats
            .iter()
            .map(|tuple| Skeleton::try_from(tuple))
    }

    fn get_valid_skeletons<'a>(
        data: &'a provider::gregory::DatesV1,
    ) -> impl Iterator<Item = Skeleton> + 'a {
        get_skeletons(data)
            .filter(|skeleton| skeleton.is_ok())
            .map(|skeleton| skeleton.unwrap())
    }

    fn get_skeleton_with_fake_pattern(skeleton_string: &'static str) -> Skeleton {
        let fake_pattern = Cow::from("FAKE");
        let tuple: SkeletonStringTuple = (Cow::Borrowed(skeleton_string), fake_pattern);
        Skeleton::try_from(&tuple).expect("Unable to generate a Skeleton")
    }

    #[test]
    fn test_skeleton() {
        // These were all of the "available formats" in the CLDR as of 2021-01
        // Generated with:
        // https://gist.github.com/gregtatum/1d76bbdb87132f71a969a10f0c1d2d9c

        let fake_pattern_string = Cow::from("FAKE");
        for string_skeleton in &STRING_SKELETONS {
            let tuple: SkeletonStringTuple =
                (Cow::Borrowed(string_skeleton), fake_pattern_string.clone());
            let skeleton = Skeleton::try_from(&tuple);
            match skeleton {
                Ok(skeleton) => {
                    eprintln!("{:?} {:#?}", string_skeleton, skeleton);
                }
                Err(SkeletonError::UnimplementedField(ch)) => {
                    eprintln!("{:?} Unimplemented Field{:?}", string_skeleton, ch);
                }
                Err(err) => {
                    panic!("Unknown character, {:?}", err);
                }
            };
        }
    }

    #[test]
    fn test_skeleton_deserialization() {
        // TODO - Write a real assertion here. Serialization was removed.
        assert_eq!(
            get_skeleton_with_fake_pattern("MMMMEEEEd"),
            Skeleton {
                fields: vec![
                    Field {
                        symbol: Month::Format.into(),
                        length: FieldLength::Wide
                    },
                    Field {
                        symbol: Weekday::Format.into(),
                        length: FieldLength::Wide
                    },
                    Field {
                        symbol: Day::DayOfMonth.into(),
                        length: FieldLength::One
                    }
                ],
                pattern_string: Cow::Borrowed("FAKE"),
                fields_by_type: FieldsByType {
                    year: None,
                    month: Some(FieldIndex(0)),
                    day: Some(FieldIndex(2)),
                    weekday: Some(FieldIndex(1)),
                    day_period: None,
                    hour: None,
                    minute: None,
                    second: None
                }
            }
        );
    }

    fn get_data_provider() -> Cow<'static, DatesV1> {
        let provider = icu_testdata::get_provider();
        let langid = langid!("en");
        provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap()
    }

    #[test]
    fn test_skeleton_matching() {
        let components = components::Bag::default();
        let data_provider = get_data_provider();
        let skeletons = get_valid_skeletons(&data_provider);

        match get_best_skeleton(skeletons, &components) {
            BestSkeleton::AllFieldsMatch(skeleton) => {
                assert_eq!(skeleton.pattern_string, "MMMM y")
            }
            _ => panic!(),
        };
    }
}
