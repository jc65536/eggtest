use std::io::*;

use egg::*;

define_language! {
    enum FieldLang {
        "0" = Zero,
        "1" = One,
        "+" = Add([Id;2]),
        "-" = AInv(Id),
        "*" = Mul([Id;2]),
        "1/" = MInv(Id),
        Symbol(Symbol),
    }
}

fn is_not_zero(var: &'static str) -> impl Fn(&mut EGraph<FieldLang, ()>, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = FieldLang::Zero;
    move |egraph, _, subst| !egraph[subst[var]].nodes.contains(&zero)
}

fn make_rules() -> Vec<Rewrite<FieldLang, ()>> {
    vec![
        rewrite!("comm-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("comm-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rewrite!("assoc-add"; "(+ (+ ?a ?b) ?c)" => "(+ ?a (+ ?b ?c))"),
        rewrite!("assoc-mul"; "(* (* ?a ?b) ?c)" => "(* ?a (* ?b ?c))"),
        rewrite!("ident-add"; "(+ ?a 0)" => "?a"),
        rewrite!("ident-mul"; "(* ?a 1)" => "?a"),
        rewrite!("inv-add"; "(+ ?a (- ?a))" => "0"),
        rewrite!("inv-mul"; "(* ?a (1/ ?a))" => "1" if is_not_zero("?a")),
        rewrite!("distrib"; "(* ?a (+ ?b ?c))" => "(+ (* ?a ?b) (* ?a ?c))"),
        rewrite!("factor"; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
    ]
}

fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<FieldLang> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&make_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}

fn main() {
    print!("> ");
    stdout().flush().unwrap();
    for line in stdin().lines() {
        simplify(line.unwrap().as_str());
        print!("> ");
        stdout().flush().unwrap();
    }
}
