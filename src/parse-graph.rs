use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    io::stdin,
    rc::Rc,
};

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
enum Regex<A> {
    Term(A),
    Concat(Rc<Regex<A>>, Rc<Regex<A>>),
    Star(Rc<Regex<A>>),
    Alt(Rc<Regex<A>>, Rc<Regex<A>>),
}

impl<A: Debug> Display for Regex<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Regex::Term(x) => write!(f, "{x:?}"),
            Regex::Concat(x, y) => match (x.as_ref(), y.as_ref()) {
                (_, Regex::Alt(_, _)) => write!(f, "{x}({y})"),
                (Regex::Alt(_, _), _) => write!(f, "({x}){y}"),
                _ => write!(f, "{x}{y}"),
            },
            Regex::Star(x) => match x.as_ref() {
                Regex::Term(_) => write!(f, "{x}*"),
                _ => write!(f, "({x})*"),
            },
            Regex::Alt(x, y) => write!(f, "{x}|{y}"),
        }
    }
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
    in_edges: Vec<(Rc<Regex<EdgeType>>, u32)>,
    out_edges: Vec<(Rc<Regex<EdgeType>>, u32)>,
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

fn parse_node(input: &str) -> IResult<&str, (Node, bool)> {
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

    let out_edges = out_edges
        .into_iter()
        .map(|(edge, node)| (Rc::new(Regex::Term(edge)), node))
        .collect();

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
        in_edges: Vec::new(),
        out_edges,
    };

    Ok((input, (node, ending.is_some())))
}

fn parse_init_node(input: &str) -> IResult<&str, Node> {
    let (input, _) = tuple((take_until("init"), tag("init")))(input)?;
    let (input, out_edges) = many0(parse_edge)(input)?;

    let out_edges = out_edges
        .into_iter()
        .map(|(edge, node)| (Rc::new(Regex::Term(edge)), node))
        .collect();

    let node = Node {
        id: 0,
        access: None,
        in_edges: Vec::new(),
        out_edges,
    };

    Ok((input, node))
}

fn parse_graph() -> HashMap<u32, Node> {
    let mut lines = stdin().lines();
    let first_line = lines.next().unwrap().unwrap();
    let (_, init_node) = parse_init_node(&first_line).unwrap();

    let mut graph: HashMap<u32, Node> = HashMap::new();
    graph.insert(0, init_node);

    let ending_node = Node {
        id: u32::MAX,
        access: None,
        in_edges: Vec::new(),
        out_edges: Vec::new(),
    };

    lines
        .map(|line| parse_node(&line.unwrap()).unwrap().1)
        .for_each(|(mut node, ending)| {
            if ending {
                node.out_edges
                    .push((Rc::new(Regex::Term(EdgeType::Po)), ending_node.id));
            }
            graph.insert(node.id, node);
        });

    graph.insert(u32::MAX, ending_node);

    let in_edges: Vec<(u32, Rc<Regex<EdgeType>>, u32)> = graph
        .iter()
        .flat_map(|(_, node)| {
            node.out_edges
                .iter()
                .map(|(edge, id)| (*id, edge.clone(), node.id))
        })
        .collect();

    // Assign in edges to each node
    in_edges.into_iter().for_each(|(id, edge, from_id)| {
        graph.get_mut(&id).unwrap().in_edges.push((edge, from_id));
    });

    graph
}

fn self_loop(node: &mut Node) -> Option<Regex<EdgeType>> {
    let mut self_loops: Vec<Regex<EdgeType>> = Vec::default();

    while let Some(out_edge_idx) = node
        .out_edges
        .iter()
        .enumerate()
        .position(|(_, (_, n))| *n == node.id)
    {
        let (self_loop_regex, _) = node.out_edges.remove(out_edge_idx);

        let corresponding_in_edge_idx = node
            .in_edges
            .iter()
            .position(|(regex, n)| *n == node.id && Rc::ptr_eq(regex, &self_loop_regex))
            .unwrap();

        node.in_edges.remove(corresponding_in_edge_idx);

        self_loops.push(Rc::try_unwrap(self_loop_regex).unwrap());
    }

    let alts = self_loops.into_iter().reduce(|acc, a| Regex::Alt(Rc::new(acc), Rc::new(a)))?;
    Some(Regex::Star(Rc::new(alts)))
}

fn remove_node(node_id: u32, graph: &mut HashMap<u32, Node>) {
    let mut node = graph.remove(&node_id).unwrap();

    let self_loop = self_loop(&mut node).map(|loop_regex| Rc::new(loop_regex));

    node.in_edges.iter().for_each(|(from_regex, from_id)| {
        let mut from_node = graph.remove(from_id).unwrap();
        let from_edge_idx = from_node
            .out_edges
            .iter()
            .position(|(r, id)| *id == node.id && Rc::ptr_eq(r, from_regex))
            .unwrap();

        from_node.out_edges.remove(from_edge_idx);

        node.out_edges.iter().for_each(|(to_regex, to_id)| {
            let mut combined_regex: Option<Rc<Regex<EdgeType>>> = None;

            {
                let to_node = if from_id == to_id {
                    &mut from_node
                } else {
                    graph.get_mut(to_id).unwrap()
                };

                println!("Debug: {} -> {} : {to_regex:?}", node.id, to_id);
                match to_node
                    .in_edges
                    .iter()
                    .position(|(r, id)| *id == node.id && Rc::ptr_eq(r, to_regex))
                {
                    Some(idx) => {
                        to_node.in_edges.remove(idx);
                    }
                    None => (),
                };

                combined_regex = Some(Rc::new(self_loop.as_ref().map_or_else(
                    || Regex::Concat(from_regex.clone(), to_regex.clone()),
                    |loop_regex| {
                        Regex::Concat(
                            Rc::new(Regex::Concat(from_regex.clone(), loop_regex.clone())),
                            to_regex.clone(),
                        )
                    },
                )));

                to_node
                    .in_edges
                    .push((combined_regex.clone().unwrap().clone(), *from_id));
            }

            from_node
                .out_edges
                .push((combined_regex.unwrap().clone(), *to_id));
        });

        graph.insert(from_node.id, from_node);
    });
}

fn reduce_graph(mut graph: HashMap<u32, Node>) -> Regex<EdgeType> {
    let ids: Vec<u32> = graph
        .keys()
        .filter_map(|&x| match x {
            0 | u32::MAX => None,
            _ => Some(x),
        })
        .collect();

    ids.into_iter().for_each(|id| remove_node(id, &mut graph));

    let alt_iter = graph.remove(&0).unwrap().out_edges;

    let (regex, _) = alt_iter
        .into_iter()
        .reduce(|(acc, _), (a, _)| (Rc::new(Regex::Alt(acc, a)), u32::MAX))
        .unwrap();
    drop(graph);
    Rc::try_unwrap(regex).unwrap()
}

fn main() {
    let graph = parse_graph();

    println!("{graph:#?}");

    let regex = reduce_graph(graph);

    println!("{regex}");
}
