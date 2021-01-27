// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::fields::{Field, FieldLength, FieldSymbol, LengthError, SymbolError};
use std::{collections::HashSet, convert::TryFrom};

#[derive(Debug)]
pub struct Skeleton(HashSet<Field>);

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

        Ok(Self(set))
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
