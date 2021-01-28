// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::fields::{self, Field, FieldLength, FieldSymbol, LengthError, SymbolError};
use serde::{
    de::{self, Deserialize, Deserializer, Unexpected, Visitor},
    ser::{self, Serialize},
};
use std::fmt;
use std::{collections::HashSet, convert::TryFrom};

#[derive(Debug, PartialEq)]
pub struct Skeleton {
    pub fields: HashSet<Field>,
}

impl Skeleton {
    pub fn new() -> Skeleton {
        Skeleton {
            fields: HashSet::new(),
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

impl TryFrom<&str> for Skeleton {
    type Error = SkeletonError;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        let mut set = HashSet::new();
        for byte in input.bytes() {
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
                        set.insert(Field {
                            symbol,
                            length: FieldLength::try_from(length)?,
                        });
                        length = 0;
                    }
                }
                None => {
                    set.insert(Field {
                        symbol,
                        length: FieldLength::try_from(length)?,
                    });
                }
            }
        }

        Ok(Self { fields: set })
    }
}

// https://serde.rs/deserialize-struct.html
impl<'de> Deserialize<'de> for Skeleton {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(SkeletonVisitor {})
    }
}

struct SkeletonVisitor {}

impl<'de> Visitor<'de> for SkeletonVisitor {
    type Value = Skeleton;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a valid string skeleton formed from date field symbols: http://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Skeleton::try_from(string).map_err(|err| {
            let msg = match err {
                SkeletonError::FieldLengthTooLong => {
                    "The skeleton contained a field that was too long."
                }
                SkeletonError::SymbolUnknown(_) => {
                    "The skeleton contained a symbol that was unknown."
                }
                SkeletonError::UnimplementedField(_) => {
                    "The skeleton contain a field that is not implemented yet."
                }
            };
            de::Error::invalid_value(Unexpected::Other(msg), &self)
        })
    }
}

impl Serialize for Skeleton {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let string = String::from(self);

        serializer.serialize_str(&string)
    }
}

#[test]
fn test_skeleton() {
    // These were all of the "available formats" in the CLDR as of 2021-01
    // Generated with:
    // https://gist.github.com/gregtatum/1d76bbdb87132f71a969a10f0c1d2d9c
    let string_skeletons = vec![
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

    for string_skeleton in string_skeletons {
        let skeleton = Skeleton::try_from(string_skeleton);
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
fn test_skeleton_serialization() {
    let parsed: Skeleton = serde_json::from_str("\"MMMMEEEEd\"").unwrap();
    let mut expected = Skeleton::new();

    expected.fields.insert(Field {
        symbol: FieldSymbol::from(fields::Day::DayOfMonth),
        length: FieldLength::One,
    });
    expected.fields.insert(Field {
        symbol: FieldSymbol::from(fields::Weekday::Format),
        length: FieldLength::Wide,
    });
    expected.fields.insert(Field {
        symbol: FieldSymbol::from(fields::Month::Format),
        length: FieldLength::Wide,
    });

    assert_eq!(parsed, expected);

    assert_eq!(serde_json::to_string(&parsed).unwrap(), r#""dMMMMEEEE""#);
}
