#[macro_use]
extern crate nom;

use nom::{alpha, digit};
use std::str;

// Unit representation and parser
#[derive(Debug, Clone, PartialEq)]
pub enum UnitRepr<'a, 'b> {
    Length(LengthRepr<'a, 'b>),
    Angle(AngleRepr<'a, 'b>),
}

named!(pub unit(&[u8]) -> UnitRepr, alt!(length | angle));

// Length representation and parser
#[derive(Debug, Clone, PartialEq)]
pub struct LengthRepr<'a, 'b> {
    pub value: &'a str,
    pub unit: &'b str,
}

named!(length_type<&[u8], &str>, alt!(
  tag!("%")   => { |_| "percent" } |
  tag!("n")   => { |_| "number" } | 
  tag!("px")  => { |_| "point" }
));

named!(pub length(&[u8]) -> UnitRepr, do_parse!(
  value: digit      >>
  unit: length_type >>
  (UnitRepr::Length(LengthRepr {
    value: str::from_utf8(value).unwrap(),
    unit
  }))
));

// Angle representation and parser
#[derive(Debug, Clone, PartialEq)]
pub struct AngleRepr<'a, 'b> {
    pub value: &'a str,
    pub angle: &'b str,
}

named!(angle_type<&[u8], &str>, alt!(
  tag!("rad") => { |_| "radians" } |
  tag!("deg") => { |_| "degrees" }
));

named!(pub angle(&[u8]) -> UnitRepr, do_parse!(
  value: digit      >>
  angle: angle_type >>
  (UnitRepr::Angle(AngleRepr {
    value: str::from_utf8(value).unwrap(),
    angle
  }))
));

#[derive(Debug, Clone, PartialEq)]
pub struct TransformFunction<'a, 'b, 'c> {
    pub args: Vec<UnitRepr<'a, 'b>>,
    pub name: &'c str,
}

named!(fn_name(&[u8]) -> &[u8], ws!(alpha));

named!(args(&[u8]) -> Vec<UnitRepr>,
  delimited!(
    char!('('),
      separated_list!(char!(','), unit),
    char!(')')
  )
);

named!(pub transform_parse(&[u8]) -> TransformFunction, do_parse!(
  name: fn_name >>
  args: args    >>
  (TransformFunction {
    name: str::from_utf8(name).unwrap(),
    args,
  })
));

fn main() {
    let my_str = "func(10px,10deg)";
    let parsed = transform_parse(my_str.as_bytes())
        .expect("Can't parse transform")
        .1;

    println!("Parsed:\n {:#?}", parsed);
}
