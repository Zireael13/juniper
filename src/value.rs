use std::collections::HashMap;
use std::hash::Hash;

use parser::Spanning;
use ast::{InputValue, ToInputValue};

/// Serializable value returned from query and field execution.
///
/// Used by the execution engine and resolvers to build up the response
/// structure. Similar to the `Json` type found in the serialize crate.
///
/// It is also similar to the `InputValue` type, but can not contain enum
/// values or variables. Also, lists and objects do not contain any location
/// information since they are generated by resolving fields and values rather
/// than parsing a source query.
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Value {
    Null,
    Int(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    // CONSTRUCTORS

    /// Construct a null value.
    pub fn null() -> Value { Value::Null }

    /// Construct an integer value.
    pub fn int(i: i32) -> Value { Value::Int(i) }

    /// Construct a floating point value.
    pub fn float(f: f64) -> Value { Value::Float(f) }

    /// Construct a string value.
    pub fn string<T: AsRef<str>>(s: T) -> Value { Value::String(s.as_ref().to_owned()) }

    /// Construct a boolean value.
    pub fn boolean(b: bool) -> Value { Value::Boolean(b) }

    /// Construct a list value.
    pub fn list(l: Vec<Value>) -> Value { Value::List(l) }

    /// Construct an object value.
    pub fn object<K>(o: HashMap<K, Value>) -> Value
        where K: AsRef<str> + Eq + Hash
    {
        Value::Object(
            o.into_iter().map(|(k, v)| (k.as_ref().to_owned(), v)).collect()
        )
    }

    // DISCRIMINATORS

    /// Does this value represent null?
    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            _ => false,
        }
    }

    /// View the underlying object value, if present.
    pub fn as_object_value(&self) -> Option<&HashMap<String, Value>> {
        match *self {
            Value::Object(ref o) => Some(o),
            _ => None,
        }
    }

    /// Mutable view into the underlying object value, if present.
    pub fn as_mut_object_value(&mut self) -> Option<&mut HashMap<String, Value>> {
        match *self {
            Value::Object(ref mut o) => Some(o),
            _ => None,
        }
    }

    /// View the underlying list value, if present.
    pub fn as_list_value(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::List(ref l) => Some(l),
            _ => None,
        }
    }

    /// View the underlying string value, if present.
    pub fn as_string_value(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }
}

impl ToInputValue for Value {
    fn to(&self) -> InputValue {
        match *self {
            Value::Null => InputValue::Null,
            Value::Int(i) => InputValue::Int(i),
            Value::Float(f) => InputValue::Float(f),
            Value::String(ref s) => InputValue::String(s.clone()),
            Value::Boolean(b) => InputValue::Boolean(b),
            Value::List(ref l) => InputValue::List(l.iter().map(|x|
                Spanning::unlocated(x.to())).collect()),
            Value::Object(ref o) => InputValue::Object(o.iter().map(|(k,v)|
                (Spanning::unlocated(k.clone()), Spanning::unlocated(v.to()))).collect()),
        }
    }
}
