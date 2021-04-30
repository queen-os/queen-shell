use self::{
    span::{Span, Spanned, SpannedItem},
    token::{SpannedToken, Token},
};
use crate::ShellError;
use alloc::vec::Vec;
use enumflags2::{bitflags, BitFlags};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, none_of, space1},
    combinator::opt,
    multi::{many0_count, many1},
    IResult, InputIter, Slice,
};
use nom_locate::LocatedSpan;

pub mod command;
pub mod hir;
pub mod span;
pub mod syntax_shape;
pub mod token;

pub type NomSpan<'a> = LocatedSpan<&'a str>;

pub fn parse(input: &str) -> Result<Spanned<Vec<SpannedToken>>, ShellError> {
    match token_list(NomSpan::new(input)) {
        Ok((_rest, val)) => Ok(val),
        Err(err) => Err(ShellError::parse_error(err)),
    }
}

pub fn dq_string(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.location_offset();
    let (input, _) = char('"')(input)?;
    let start1 = input.location_offset();
    let (input, _) = many0_count(none_of("\""))(input)?;
    let end1 = input.location_offset();
    let (input, _) = char('"')(input)?;
    let end = input.location_offset();

    Ok((
        input,
        Token::String(Span::new(start1, end1)).spanned(Span::new(start, end)),
    ))
}

pub fn sq_string(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.location_offset();
    let (input, _) = char('\'')(input)?;
    let start1 = input.location_offset();
    let (input, _) = many0_count(none_of("\'"))(input)?;
    let end1 = input.location_offset();
    let (input, _) = char('\'')(input)?;
    let end = input.location_offset();

    Ok((
        input,
        Token::String(Span::new(start1, end1)).spanned(Span::new(start, end)),
    ))
}

#[inline]
pub fn string(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    alt((sq_string, dq_string))(input)
}

pub fn separator(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let left = input.location_offset();
    let (input, _) = alt((tag(";"), tag("\n")))(input)?;
    let right = input.location_offset();

    Ok((input, Token::Separator.spanned(Span::new(left, right))))
}

pub fn whitespace(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let left = input.location_offset();
    let (input, _) = space1(input)?;
    let right = input.location_offset();

    Ok((input, Token::Whitespace.spanned(Span::new(left, right))))
}

#[inline]
pub fn any_space(input: NomSpan) -> IResult<NomSpan, Vec<SpannedToken>> {
    let (input, tokens) = many1(alt((whitespace, separator)))(input)?;

    Ok((input, tokens))
}

fn word<'a, T, U, V>(
    start_predicate: impl Fn(NomSpan<'a>) -> IResult<NomSpan<'a>, U>,
    next_predicate: impl Fn(NomSpan<'a>) -> IResult<NomSpan<'a>, V> + Copy,
    into: impl Fn(Span) -> T,
) -> impl Fn(NomSpan<'a>) -> IResult<NomSpan<'a>, T> {
    move |input: NomSpan| {
        let start = input.location_offset();

        let (input, _) = start_predicate(input)?;
        let (input, _) = many0_count(next_predicate)(input)?;

        let next_char = &input.fragment().chars().next();

        match next_char {
            Some('.') => {}
            Some(next_char)
                if is_external_word_char(*next_char) || is_glob_specific_char(*next_char) =>
            {
                return Err(nom::Err::Error(nom::error::make_error(
                    input,
                    nom::error::ErrorKind::TakeWhile1,
                )));
            }
            _ => {}
        }

        let end = input.location_offset();

        Ok((input, into(Span::new(start, end))))
    }
}

pub fn matches(cond: fn(char) -> bool) -> impl Fn(NomSpan) -> IResult<NomSpan, NomSpan> + Copy {
    move |input: NomSpan| match input.iter_elements().next() {
        Option::Some(c) if cond(c) => {
            let len_utf8 = c.len_utf8();
            Ok((input.slice(len_utf8..), input.slice(0..len_utf8)))
        }
        _ => Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            input,
            nom::error::ErrorKind::Many0,
        ))),
    }
}

#[inline]
pub fn pattern(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    word(start_pattern, matches(is_glob_char), |span| {
        Token::GlobPattern.spanned(span)
    })(input)
}

#[inline]
pub fn start_pattern(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    alt((take_while1(is_dot), matches(is_start_glob_char)))(input)
}

pub fn filename(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start_pos = input.location_offset();

    let (mut input, mut saw_special) = match start_file_char(input) {
        Err(err) => return Err(err),
        Ok((input, special)) => (input, special),
    };

    loop {
        if saw_special.is_empty() {
            match continue_file_char(input) {
                Err(_) => {
                    return Ok((
                        input,
                        Token::Bare.spanned(Span::new(start_pos, input.location_offset())),
                    ));
                }
                Ok((next_input, special)) => {
                    saw_special |= special;
                    input = next_input;
                }
            }
        } else {
            let rest = after_sep_file(input);

            let (input, span, updated_special) = match rest {
                Err(_) => (input, (start_pos, input.location_offset()), saw_special),
                Ok((input, new_special)) => (
                    input,
                    (start_pos, input.location_offset()),
                    saw_special | new_special,
                ),
            };

            return if updated_special.contains(SawSpecial::Glob) {
                Ok((input, Token::GlobPattern.spanned(span)))
            } else {
                Ok((input, Token::Bare.spanned(span)))
            };
        }
    }
}

#[bitflags]
#[repr(u64)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum SawSpecial {
    PathSeparator = 0b01,
    Glob = 0b10,
}

fn start_file_char(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    let path_sep_result = special_file_char(input);

    if let Ok((input, special)) = path_sep_result {
        return Ok((input, special));
    }

    start_filename(input).map(|(input, _)| (input, BitFlags::empty()))
}

fn continue_file_char(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    let path_sep_result = special_file_char(input);

    if let Ok((input, special)) = path_sep_result {
        return Ok((input, special));
    }

    matches(is_file_char)(input).map(|(input, _)| (input, BitFlags::empty()))
}

fn special_file_char(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    if let Ok((input, _)) = matches(is_path_separator)(input) {
        return Ok((input, BitFlags::empty() | SawSpecial::PathSeparator));
    }

    let (input, _) = matches(is_glob_specific_char)(input)?;

    Ok((input, BitFlags::empty() | SawSpecial::Glob))
}

fn after_sep_file(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    fn after_sep_char(c: char) -> bool {
        is_external_word_char(c) || is_file_char(c) || c == '.'
    }

    let start = input.location_offset();
    let original_input = input;
    let input = input;

    let (input, _) = take_while1(after_sep_char)(input)?;

    let slice = original_input.slice(0..input.location_offset() - start);

    let saw_special = if slice.fragment().chars().any(is_glob_specific_char) {
        BitFlags::empty() | SawSpecial::Glob
    } else {
        BitFlags::empty()
    };

    Ok((input, saw_special))
}

pub fn start_filename(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    alt((take_while1(is_dot), matches(is_start_file_char)))(input)
}

pub fn flag(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.location_offset();
    let (input, _) = tag("--")(input)?;
    let (input, bare) = filename(input)?;
    let end = input.location_offset();

    Ok((input, Token::Flag(bare.span).spanned(Span::new(start, end))))
}

pub fn external_word(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.location_offset();
    let (input, _) = take_while1(is_external_word_char)(input)?;
    let end = input.location_offset();

    Ok((input, Token::ExternalWord.spanned(Span::new(start, end))))
}

pub fn node(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let (input, node) = alt((string, flag, filename, pattern, external_word))(input)?;

    Ok((input, node))
}

pub fn token_list(input: NomSpan) -> IResult<NomSpan, Spanned<Vec<SpannedToken>>> {
    let start = input.location_offset();
    let mut node_list = vec![];

    let mut next_input = input;
    let mut before_space_input: Option<NomSpan> = None;
    let mut final_space_tokens = 0;
    loop {
        let node_result = node(next_input);
        let (after_node_input, next_node) = match node_result {
            Err(_) => {
                if let Some(before_space_input) = before_space_input {
                    next_input = before_space_input;

                    for _ in 0..final_space_tokens {
                        node_list.pop();
                    }
                }

                break;
            }
            Ok((after_node_input, next_node)) => (after_node_input, next_node),
        };

        node_list.push(next_node);

        let maybe_space = any_space(after_node_input);

        let after_space_input = match maybe_space {
            Err(_) => {
                next_input = after_node_input;

                break;
            }
            Ok((after_space_input, space)) => {
                final_space_tokens = space.len();
                node_list.extend(space);
                before_space_input = Some(after_node_input);
                after_space_input
            }
        };

        next_input = after_space_input;
    }
    let end = next_input.location_offset();

    Ok((next_input, node_list.spanned(Span::new(start, end))))
}

pub fn spaced_token_list(input: NomSpan) -> IResult<NomSpan, Spanned<Vec<SpannedToken>>> {
    let start = input.location_offset();
    let (input, _) = opt(any_space)(input)?;
    let (input, items) = token_list(input)?;
    let (input, post_ws) = opt(any_space)(input)?;
    let end = input.location_offset();

    let mut out = vec![];

    out.extend(items.item);
    if let Some(post_ws) = post_ws {
        out.extend(post_ws)
    }

    Ok((input, out.spanned(Span::new(start, end))))
}

#[inline]
fn is_external_word_char(c: char) -> bool {
    match c {
        ';' | '|' | '"' | '\'' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '`' => false,
        other if other.is_whitespace() => false,
        _ => true,
    }
}

/// These characters appear in globs and not bare words
#[inline]
fn is_glob_specific_char(c: char) -> bool {
    matches!(c, '*' | '?')
}

#[inline]
fn is_start_glob_char(c: char) -> bool {
    is_start_file_char(c) || is_glob_specific_char(c) || c == '.'
}

#[inline]
fn is_glob_char(c: char) -> bool {
    is_file_char(c) || is_glob_specific_char(c)
}

#[inline]
const fn is_dot(c: char) -> bool {
    c == '.'
}

#[inline]
const fn is_path_separator(c: char) -> bool {
    matches!(c, '\\' | '/' | ':')
}

#[inline]
fn is_start_file_char(c: char) -> bool {
    match c {
        '+' => false,
        _ if c.is_alphanumeric() => true,
        '\\' => true,
        '/' => true,
        '_' => true,
        '-' => true,
        '~' => true,
        '.' => true,
        _ => false,
    }
}

#[inline]
fn is_file_char(c: char) -> bool {
    match c {
        '+' => true,
        _ if c.is_alphanumeric() => true,
        '\\' => true,
        '/' => true,
        '_' => true,
        '-' => true,
        '=' => true,
        '~' => true,
        ':' => true,
        '?' => true,
        _ => false,
    }
}
