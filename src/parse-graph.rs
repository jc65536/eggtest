use std::{collections::HashMap, io::stdin, hash::Hash};

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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
struct Node {
    id: u32,
    access: Option<MemAccess>,
    ending: bool,
    in_edges: HashMap<EdgeType, u32>,
    out_edges: HashMap<EdgeType, u32>,
}

fn parse_edge(input: &str) -> IResult<&str, (EdgeType, u32)> {
    let (input, (_, edge_type_str, _, id_str)) =
        tuple((tag(" | "), alt((alpha1, tag("~"))), tag(" -> "), digit1))(input)?;
    let edge_type = match edge_type_str {
        "po" => EdgeType::Po,
        "rf" => EdgeType::Rf,
        "dmb" => EdgeType::Dmb,
        "lwsync" => EdgeType::Lwsync,
        "addr" => EdgeType::Addr,
        "ctrl" => EdgeType::Ctrl,
        "ctrlisb" => EdgeType::Ctrlisb,
        "co" => EdgeType::Co,
        "data" => EdgeType::Data,
        "fr" => EdgeType::Fr,
        _ => panic!("Invalid edge type"),
    };
    Ok((input, (edge_type, id_str.parse().unwrap())))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, (ending, id_str, _, access_ch, addr, _, value_str)) = tuple((
        opt(tag("$")),
        digit1,
        tag(": "),
        one_of("RW"),
        alpha1,
        tag("="),
        digit1,
    ))(input)?;

    let (input, out_edges) = many0(parse_edge)(input)?;

    let node = Node {
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
        in_edges: HashMap::new(),
        out_edges: HashMap::from_iter(out_edges.into_iter()),
    };

    Ok((input, node))
}

fn parse_init_node(input: &str) -> IResult<&str, Node> {
    let (input, _) = tuple((take_until("init"), tag("init")))(input)?;
    let (input, out_edges) = many0(parse_edge)(input)?;

    let node = Node {
        id: 0,
        access: None,
        ending: false,
        in_edges: HashMap::new(),
        out_edges: HashMap::from_iter(out_edges.into_iter()),
    };

    Ok((input, node))
}

fn parse_graph() -> HashMap<u32, Node> {
    let mut lines = stdin().lines();
    let first_line = lines.next().unwrap().unwrap();
    let (_, init_node) = parse_init_node(&first_line).unwrap();
    let mut graph: HashMap<u32, Node> = HashMap::new();
    graph.insert(0, init_node);
    lines
        .map(|line| parse_node(&line.unwrap()).unwrap().1)
        .for_each(|node| {
            graph.insert(node.id, node);
        });

    graph.iter_mut().for_each(|(_, node)| {
        node.out_edges.iter_mut().for_each(|(edge, n)| {
            graph.get_mut(n).unwrap().in_edges.insert(*edge, node.id);
        })
    });

    graph
}

fn main() {
    let graph = parse_graph();

    println!("{graph:?}");
}
