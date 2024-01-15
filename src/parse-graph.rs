use std::{collections::HashMap, io::stdin};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{
        complete::{alpha1, digit1},
        streaming::one_of,
    },
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum AccessType {
    Read,
    Write,
}

#[derive(Debug)]
enum EdgeType {
    Po,
    Rf,
    Dmb,
    Lwsync,
    Addr,
    Ctrl,
    Ctrlisb,
    Co,
    Data,
    Fr,
}

#[derive(Debug)]
struct MemAccess {
    t: AccessType,
    addr: String,
    value: u32,
}

#[derive(Debug)]
struct State {
    id: u32,
    access: Option<MemAccess>,
    ending: bool,
    out_edges: Vec<(Option<EdgeType>, u32)>,
}

fn parse_edge(input: &str) -> IResult<&str, (Option<EdgeType>, u32)> {
    let (input, (_, edge_type_str, _, id_str)) =
        tuple((tag(" | "), alt((alpha1, tag("~"))), tag(" -> "), digit1))(input)?;
    let edge_type = match edge_type_str {
        "po" => Some(EdgeType::Po),
        "rf" => Some(EdgeType::Rf),
        "dmb" => Some(EdgeType::Dmb),
        "lwsync" => Some(EdgeType::Lwsync),
        "addr" => Some(EdgeType::Addr),
        "ctrl" => Some(EdgeType::Ctrl),
        "ctrlisb" => Some(EdgeType::Ctrlisb),
        "co" => Some(EdgeType::Co),
        "data" => Some(EdgeType::Data),
        "fr" => Some(EdgeType::Fr),
        "~" => None,
        _ => panic!("Invalid edge type"),
    };
    Ok((input, (edge_type, id_str.parse().unwrap())))
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (input, (id_str, ending, _, access_ch, addr, _, value_str)) = tuple((
        digit1,
        opt(tag("$")),
        tag(": "),
        one_of("RW"),
        alpha1,
        tag("="),
        digit1,
    ))(input)?;

    let (input, out_edges) = many0(parse_edge)(input)?;

    let state = State {
        id: id_str.parse().unwrap(),
        access: Some(MemAccess {
            t: match access_ch {
                'R' => AccessType::Read,
                'W' => AccessType::Write,
                _ => panic!("Invalid access type"),
            },
            addr: addr.to_string(),
            value: value_str.parse().unwrap(),
        }),
        ending: ending.is_some(),
        out_edges,
    };

    Ok((input, state))
}

fn parse_init_state(input: &str) -> IResult<&str, State> {
    let (input, _) = tuple((take_until("~"), tag("~")))(input)?;
    let (input, out_edges) = many0(parse_edge)(input)?;

    let state = State {
        id: 0,
        access: None,
        ending: false,
        out_edges,
    };

    Ok((input, state))
}

fn parse_graph() -> HashMap<u32, State> {
    let mut lines = stdin().lines();
    let first_line = lines.next().unwrap().unwrap();
    let (_, init_state) = parse_init_state(&first_line).unwrap();
    let mut graph: HashMap<u32, State> = HashMap::new();
    graph.insert(0, init_state);
    lines
        .map(|line| parse_state(&line.unwrap()).unwrap().1)
        .for_each(|state| {
            graph.insert(state.id, state);
        });
    graph
}

fn main() {
    let graph = parse_graph();

    println!("{graph:?}");
}
