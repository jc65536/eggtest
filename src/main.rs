use std::{fmt::Display, io::*, time::Duration};

use egg::*;

mod kat;

fn simplify<L: kat::LangRules + FromOp + Display>(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<L> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default()
        .with_time_limit(Duration::from_secs(100))
        .with_node_limit(1000000)
        .with_iter_limit(100)
        .with_expr(&expr)
        .run(&L::rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified: {best}");
    println!("Cost: {best_cost}");
    println!("Stop reason: {:?}", runner.stop_reason.unwrap());
    best.to_string()
}

fn main() {
    print!("> ");
    stdout().flush().unwrap();
    for line in stdin().lines() {
        simplify::<kat::KATLang>(line.unwrap().as_str());
        print!("> ");
        stdout().flush().unwrap();
    }
}
