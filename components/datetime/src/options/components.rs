// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Components is a model of encoding information on how to format date and time by specifying a list of components
//! the user wants to be visible in the formatted string and how each field should be displayed.
//!
//! This model closely corresponds to `ECMA402` API and allows for high level of customization compared to `Style` model.
//!
//! Additionally, the bag contains an optional set of `Preferences` which represent user preferred adjustments
//! that can be applied onto the pattern right before formatting.
//!
//! # Pattern Selection
//!
//! It is important to understand that the components bag is a human-friendly way to describe a skeleton, not a pattern.
//! That means that the components and their styles provided by the user will be matched against available patterns for
//! a given locale and the closest available pattern will be used for formatting.
//!
//! That means, that it is possible that if the user asks for a combination of fields or lengths that `CLDR` has no
//! data associated with, the selected pattern may be different than the selection in the `Components` bag.
//! Such scenarios should be rare.
//!
//! # Examples
//!
//! ```
//! use icu_datetime::DateTimeFormatOptions;
//! use icu_datetime::options::components;
//!
//! let bag = components::Bag {
//!     year: Some(components::Numeric::Numeric),
//!     month: Some(components::Month::Long),
//!     day: Some(components::Numeric::Numeric),
//!
//!     hour: Some(components::Numeric::TwoDigit),
//!     minute: Some(components::Numeric::TwoDigit),
//!
//!     preferences: None,
//!
//!     ..Default::default()
//! };
//!
//! // The options can be created manually.
//! let options = DateTimeFormatOptions::Components(bag);
//! ```
//!
//! Or the options can be inferred through the `.into()` trait.
//!
//! ```
//! # use icu_datetime::DateTimeFormatOptions;
//! # use icu_datetime::options::components;
//! let options: DateTimeFormatOptions = components::Bag::default().into();
//! ```
//!
//! *Note*: The exact result returned from [`DateTimeFormat`](crate::DateTimeFormat) is a subject to change over
//! time. Formatted result should be treated as opaque and displayed to the user as-is,
//! and it is strongly recommended to never write tests that expect a particular formatted output.
use super::preferences;
#[cfg(all(not(feature = "serialize_none"), feature = "serde"))]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Deserialize)
)]
pub struct Bag {
    pub era: Option<Text>,
    pub year: Option<Numeric>,
    pub month: Option<Month>,
    pub day: Option<Numeric>,
    pub weekday: Option<Text>,

    pub hour: Option<Numeric>,
    pub minute: Option<Numeric>,
    pub second: Option<Numeric>,

    pub time_zone_name: Option<TimeZoneName>,

    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(skip_serializing, skip_deserializing)
    )]
    pub preferences: Option<preferences::Bag>,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            era: None,
            year: Some(Numeric::Numeric),
            month: Some(Month::Long),
            day: Some(Numeric::Numeric),
            weekday: None,

            hour: Some(Numeric::Numeric),
            minute: Some(Numeric::Numeric),
            second: Some(Numeric::Numeric),

            time_zone_name: None,

            preferences: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum Numeric {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "numeric")
    )]
    Numeric,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "two-digit")
    )]
    TwoDigit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum Text {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "long")
    )]
    Long,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "short")
    )]
    Short,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "narrow")
    )]
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum Month {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "numeric")
    )]
    Numeric,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "two-digit")
    )]
    TwoDigit,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "long")
    )]
    Long,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "short")
    )]
    Short,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "narrow")
    )]
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TimeZoneName {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "long")
    )]
    Long,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "short")
    )]
    Short,
}

#[cfg(test)]
mod test {
    // TODO - Remove me
    #![allow(unused_imports)]
    use crate::options::preferences::HourCycle;

    use super::*;
    use serde::{de, ser, Serialize};

    // As of 2021-01-26:
    // The total list of locales, and the count of skeleton usage.
    // Script:
    // ```js
    // const testFolder = './tests/';
    // const fs = require('fs');
    // const path = require('path');
    //
    // const localesPath = "./cldr-json/cldr-dates-full/main";
    // const skeletons = {};
    // let localeCount = 0;
    //
    // for (const locale of fs.readdirSync(localesPath)) {
    //   localeCount++;
    //   const calendarPath = path.join(localesPath, locale, "ca-gregorian.json");
    //   const calendar = JSON.parse(fs.readFileSync(calendarPath, 'utf8'));
    //   const { availableFormats } = calendar.main[locale].dates.calendars.gregorian.dateTimeFormats
    //   for (const skeleton of Object.keys(availableFormats)) {
    //     skeletons[skeleton] = (skeletons[skeleton] || 0) + 1;
    //   }
    // }
    //
    // console.log({localeCount, skeletons});
    // ```
    // {
    //     localeCount: 565,
    //     skeletons: {
    //       Bh: 565,
    //       Bhm: 565,
    //       Bhms: 565,
    //       d: 565,
    //       E: 565,
    //       EBhm: 565,
    //       EBhms: 565,
    //       Ed: 565,
    //       Ehm: 565,
    //       EHm: 565,
    //       Ehms: 565,
    //       EHms: 565,
    //       Gy: 565,
    //       GyMMM: 565,
    //       GyMMMd: 565,
    //       GyMMMEd: 565,
    //       h: 565,
    //       H: 565,
    //       hm: 565,
    //       Hm: 565,
    //       hms: 565,
    //       Hms: 565,
    //       hmsv: 565,
    //       Hmsv: 565,
    //       hmv: 565,
    //       Hmv: 565,
    //       M: 565,
    //       Md: 565,
    //       MEd: 565,
    //       MMM: 565,
    //       MMMd: 565,
    //       MMMEd: 565,
    //       MMMMd: 565,
    //       MMMMEd: 223,
    //       'MMMMW-count-one': 393,
    //       'MMMMW-count-other': 565,
    //       ms: 565,
    //       y: 565,
    //       yM: 565,
    //       yMd: 565,
    //       yMEd: 565,
    //       yMMM: 565,
    //       yMMMd: 565,
    //       yMMMEd: 565,
    //       yMMMM: 565,
    //       yQQQ: 565,
    //       yQQQQ: 565,
    //       'yw-count-one': 392,
    //       'yw-count-other': 565,
    //       MMdd: 271,
    //       'MMMMW-count-zero': 30,
    //       'MMMMW-count-two': 37,
    //       'MMMMW-count-few': 63,
    //       'MMMMW-count-many': 46,
    //       yMM: 172,
    //       'yw-count-zero': 30,
    //       'yw-count-two': 36,
    //       'yw-count-few': 63,
    //       'yw-count-many': 46,
    //       GyMMMM: 36,
    //       GyMMMMd: 40,
    //       GyMMMMEd: 39,
    //       MMMM: 2,
    //       MMMMdd: 8,
    //       yMMMMd: 56,
    //       yMMMMEd: 53,
    //       MMd: 66,
    //       hmsvvvv: 33,
    //       Hmsvvvv: 33,
    //       hmvvvv: 5,
    //       Hmvvvv: 5,
    //       yQ: 1,
    //       yMMdd: 20,
    //       'Md-alt-variant': 1,
    //       'MEd-alt-variant': 1,
    //       'MMdd-alt-variant': 1,
    //       'yM-alt-variant': 1,
    //       'yMd-alt-variant': 1,
    //       'yMEd-alt-variant': 1,
    //       MMMdd: 32,
    //       mmss: 21,
    //       HHmmZ: 2,
    //       yMMMMEEEEd: 3,
    //       yMMMMccccd: 1,
    //       EEEEd: 3,
    //       GyMMMEEEEd: 4,
    //       MEEEEd: 3,
    //       MMMEEEEd: 4,
    //       yMEEEEd: 3,
    //       yMMMEEEEd: 15,
    //       HHmmss: 4,
    //       Mdd: 1,
    //       HHmm: 2,
    //       GyM: 1,
    //       MMMMEEEEd: 1
    //     }
    //   }

    /// Translate a components bag into it's skeleton.
    /// http://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
    impl Serialize for Bag {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            let mut string = String::new();

            if let Some(era) = self.era {
                string.push_str(match era {
                    Text::Long => "G", // Also GG, and GGG
                    Text::Short => "GGGG",
                    Text::Narrow => "GGGGG",
                });
            }
            if let Some(year) = self.year {
                string.push_str(match year {
                    Numeric::Numeric => "y",
                    Numeric::TwoDigit => "yy",
                    // Available formats ignores
                });
            }
            if let Some(month) = self.month {
                string.push_str(match month {
                    Month::Numeric => "M",
                    Month::TwoDigit => "MM",
                    Month::Short => "MMM",
                    Month::Long => "MMMM",
                    Month::Narrow => "MMMMM",
                });
            }
            if let Some(day) = self.day {
                string.push_str(match day {
                    Numeric::Numeric => "d",
                    Numeric::TwoDigit => "dd",
                });
            }
            if let Some(weekday) = self.weekday {
                string.push_str(match weekday {
                    Text::Long => "EEEE",
                    Text::Short => "EEEEEE",
                    Text::Narrow => "EEEEE",
                });
            }
            if let Some(hour) = self.hour {
                let hour_cycle: Option<HourCycle> = match self.preferences {
                    Some(ref preferences) => preferences.hour_cycle,
                    None => None,
                };

                string.push_str(match hour_cycle {
                    // TODO - What happens if None is provided?
                    Some(HourCycle::H24) | None => match hour {
                        Numeric::Numeric => "k",
                        Numeric::TwoDigit => "kk",
                    },
                    Some(HourCycle::H23) => match hour {
                        Numeric::Numeric => "H",
                        Numeric::TwoDigit => "HH",
                    },
                    Some(HourCycle::H12) => match hour {
                        Numeric::Numeric => "h",
                        Numeric::TwoDigit => "hh",
                    },
                    Some(HourCycle::H11) => match hour {
                        Numeric::Numeric => "K",
                        Numeric::TwoDigit => "KK",
                    },
                });
            }
            if let Some(minute) = self.minute {
                string.push_str(match minute {
                    Numeric::Numeric => "m",
                    Numeric::TwoDigit => "mm",
                });
            }
            if let Some(second) = self.second {
                string.push_str(match second {
                    Numeric::Numeric => "s",
                    Numeric::TwoDigit => "ss",
                });
            }
            if let Some(time_zone_name) = self.time_zone_name {
                // TODO - There are a lot more time zone options here.
                string.push_str(match time_zone_name {
                    TimeZoneName::Long => "z",
                    TimeZoneName::Short => "zzzz",
                });
            }

            serializer.serialize_str(&string)
        }
    }

    #[test]
    fn test_default_serialization() {
        let bag = Bag::default();
        assert_eq!(serde_json::to_string(&bag).unwrap(), "\"yMMMMdkms\"");
    }

    #[test]
    fn test_component_serialization_none() {
        let bag = Bag {
            era: None,
            year: None,
            month: None,
            day: None,
            weekday: None,

            hour: None,
            minute: None,
            second: None,

            time_zone_name: None,

            preferences: None,
        };
        assert_eq!(serde_json::to_string(&bag).unwrap(), "\"\"");
    }

    #[test]
    fn test_component_serialization_v1() {
        let bag = Bag {
            era: Some(Text::Long),
            year: Some(Numeric::Numeric),
            month: Some(Month::Numeric),
            day: Some(Numeric::Numeric),
            weekday: None,

            hour: Some(Numeric::Numeric),
            minute: Some(Numeric::Numeric),
            second: Some(Numeric::Numeric),

            time_zone_name: None,

            preferences: None,
        };
        assert_eq!(serde_json::to_string(&bag).unwrap(), "\"\"");
    }

    #[test]
    fn test_component_serialization_v2() {
        let bag = Bag {
            era: Some(Text::Short),
            year: Some(Numeric::TwoDigit),
            month: Some(Month::TwoDigit),
            day: Some(Numeric::TwoDigit),
            weekday: None,

            hour: Some(Numeric::TwoDigit),
            minute: Some(Numeric::TwoDigit),
            second: Some(Numeric::TwoDigit),

            time_zone_name: None,

            preferences: None,
        };
        assert_eq!(serde_json::to_string(&bag).unwrap(), "\"\"");
    }

    #[test]
    fn test_component_serialization_v3() {
        let bag = Bag {
            era: Some(Text::Narrow),
            year: None,
            month: Some(Month::Long),
            day: None,
            weekday: None,

            hour: None,
            minute: None,
            second: None,

            time_zone_name: None,

            preferences: None,
        };
        assert_eq!(serde_json::to_string(&bag).unwrap(), "\"\"");
    }

    #[test]
    fn test_component_serialization_v4() {
        let bag = Bag {
            era: None,
            year: None,
            month: Some(Month::Short),
            day: None,
            weekday: None,

            hour: None,
            minute: None,
            second: None,

            time_zone_name: None,

            preferences: None,
        };
        assert_eq!(serde_json::to_string(&bag).unwrap(), "\"\"");
    }
}
