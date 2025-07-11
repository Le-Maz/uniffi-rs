/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::CodeType;
use crate::{
    bail,
    interface::{DefaultValue, Literal, Radix, Type},
    Result,
};

fn render_literal(literal: &Literal) -> Result<String> {
    fn typed_number(type_: &Type, num_str: String) -> Result<String> {
        let unwrapped_type = match type_ {
            Type::Optional { inner_type } => inner_type,
            t => t,
        };
        Ok(match unwrapped_type {
            // special case Int32.
            Type::Int32 => num_str,
            // otherwise use constructor e.g. UInt8(x)
            Type::Int8
            | Type::UInt8
            | Type::Int16
            | Type::UInt16
            | Type::UInt32
            | Type::Int64
            | Type::UInt64
            | Type::Float32
            | Type::Float64 =>
            // XXX we should pass in the codetype itself.
            {
                format!(
                    "{}({num_str})",
                    super::SwiftCodeOracle.find(type_).type_label()
                )
            }
            _ => bail!("Unexpected literal: {num_str} for type: {type_:?}"),
        })
    }

    Ok(match literal {
        Literal::Boolean(v) => format!("{v}"),
        Literal::String(s) => format!("\"{s}\""),
        Literal::Int(i, radix, type_) => typed_number(
            type_,
            match radix {
                Radix::Octal => format!("0o{i:o}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        )?,
        Literal::UInt(i, radix, type_) => typed_number(
            type_,
            match radix {
                Radix::Octal => format!("0o{i:o}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        )?,
        Literal::Float(string, type_) => typed_number(type_, string.clone())?,
        _ => bail!("Invalid literal: {literal:?}"),
    })
}

macro_rules! impl_code_type_for_primitive {
    ($T:ident, $class_name:literal, $def:literal) => {
        #[derive(Debug)]
        pub struct $T;

        impl CodeType for $T {
            fn type_label(&self) -> String {
                $class_name.into()
            }

            fn default(&self, default: &DefaultValue) -> Result<String> {
                match default {
                    DefaultValue::Default => Ok($def.into()),
                    DefaultValue::Literal(l) => render_literal(&l),
                }
            }
        }
    };
}

impl_code_type_for_primitive!(BooleanCodeType, "Bool", "false");
impl_code_type_for_primitive!(StringCodeType, "String", "\"\"");
impl_code_type_for_primitive!(BytesCodeType, "Data", "Data([])");
impl_code_type_for_primitive!(Int8CodeType, "Int8", "Int8(0)");
impl_code_type_for_primitive!(Int16CodeType, "Int16", "Int16(0)");
impl_code_type_for_primitive!(Int32CodeType, "Int32", "0");
impl_code_type_for_primitive!(Int64CodeType, "Int64", "Int64(0)");
impl_code_type_for_primitive!(UInt8CodeType, "UInt8", "UInt8(0)");
impl_code_type_for_primitive!(UInt16CodeType, "UInt16", "UInt16(0)");
impl_code_type_for_primitive!(UInt32CodeType, "UInt32", "UInt32(0)");
impl_code_type_for_primitive!(UInt64CodeType, "UInt64", "UInt64(0)");
impl_code_type_for_primitive!(Float32CodeType, "Float", "Float(0)");
impl_code_type_for_primitive!(Float64CodeType, "Double", "Double(0)");
