//! Methods defined in the [ECMAScript Language Specification](https://tc39.es/ecma262).

// [Syntax-Directed Operations](https://tc39.es/ecma262/#sec-syntax-directed-operations)
mod bound_names;
mod is_simple_parameter_list;
mod private_bound_identifiers;
mod prop_name;

// Abstract Operations
mod string_char_at;
mod string_index_of;
mod string_last_index_of;
mod string_to_big_int;
mod to_big_int;
mod to_boolean;
mod to_int_32;
mod to_number;
mod to_string;

pub use self::{
    bound_names::BoundNames,
    is_simple_parameter_list::IsSimpleParameterList,
    private_bound_identifiers::PrivateBoundIdentifiers,
    prop_name::PropName,
    string_char_at::StringCharAt,
    string_index_of::StringIndexOf,
    string_last_index_of::StringLastIndexOf,
    string_to_big_int::StringToBigInt,
    to_big_int::ToBigInt,
    to_boolean::ToBoolean,
    to_int_32::ToInt32,
    to_number::{NumberValue, ToNumber},
    to_string::ToJsString,
};
