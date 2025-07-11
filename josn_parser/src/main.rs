use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    sequence::delimited,
    combinator::value,
    IResult
};
use serde_json::Value;
fn main() {
    println!("Hello, world");
}


fn parse_null(input: &str) -> IResult<&str , Value>{
    // null
//    value(
//         val: Value::null,
//         parse: delimited(multispace0, tag("null"), multispace0).parse(input)
//     );
}

fn parse_bool(input: &str) -> IResult<&str , Value>{
    delimited(multispace0, alt(
            (
                value(Value::Bool(true), tag("true")),
                value(Value::Bool(false), tag("false"))
            )
        ), multispace0).parse(input)
}

fn parse_number(input: &str) -> IResult<&str , Value>{
   let (input , number) = delimited(multispace0, recognize_float, multispace0).parse(input)?;
   Ok((input, Value::Number(number.parse().unwrap())));
}
fn parse_string(input: &str) -> IResult<&str , Value>{
    map(
        parser:delimited(multispace0, recognize_string, multispace0),
        |string| Value::String(string)
    ).parse(input)
}

fn recognize_string(input: &str) -> IResult<&str , String>{

    map(
        parser:delimited(
        char_parser('"'),
        many0(none_of("\"\\")),
        tag('"'),
    ).parse(input),
        |chars| chars.into_iter().collect()
    )

}

fn parser_array(input: &str) -> IResult<&str , Value>{
    delimited(
        multispace0,
        delimited(
            char_parser("["),
            separated_list0(char_parser(","), parser_value),
            char_parser("]")
        ),
        multispace0
    )
}