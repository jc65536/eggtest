use egg::*;

/*
Kleene Algebra identities
1 + (p + q)*q = (p*q)*
(+ 1 (. (* (+ p q)) q)) = (* (. (* p) q))
Axioms applied: 12, 19, 20
*/

pub trait LangRules
where
    Self: Language,
{
    fn rules() -> Vec<Rewrite<Self, ()>>;
}

define_language! {
    pub enum KATLang {
        "0" = Zero,
        "1" = One,
        "+" = Plus([Id;2]),
        "." = Dot([Id;2]),
        "*" = Star(Id),
        Symbol(Symbol),
    }
}

impl LangRules for KATLang {
    fn rules() -> Vec<Rewrite<Self, ()>> {
        vec![
            rewrite!("01"; "(+ ?p (+ ?q ?r))" <=> "(+ (+ ?p ?q) ?r)"),
            rewrite!("02"; "(+ ?p ?q)" <=> "(+ ?q ?p)"),
            rewrite!("03"; "(+ ?p 0)" <=> "?p"),
            rewrite!("04"; "(+ ?p ?p)" <=> "?p"),
            rewrite!("05"; "(. ?p (. ?q ?r))" <=> "(. (. ?p ?q) ?r)"),
            rewrite!("06"; "(. 1 ?p)" <=> "?p"),
            rewrite!("07"; "(. ?p 1)" <=> "?p"),
            rewrite!("08"; "(. ?p (+ ?q ?r))" <=> "(+ (. ?p ?q) (. ?p ?r))"),
            rewrite!("09"; "(. (+ ?p ?q) ?r)" <=> "(+ (. ?p ?r) (. ?q ?r))"),
            vec![
                rewrite!("10"; "(. 0 ?p)" => "0"),
                rewrite!("11"; "(. ?p 0)" => "0"),
            ],
            rewrite!("12"; "(+ 1 (. ?p (* ?p)))" <=> "(* ?p)"),
            rewrite!("13"; "(+ 1 (. (* ?p) ?p))" <=> "(* ?p)"),
            // Not sure how to write axioms 14, 15
            rewrite!("19"; "(. p (* (. q p)))" <=> "(. (* (. p q)) p)"),
            rewrite!("20"; "(. (* p) (* (. q (* p))))" <=> "(* (+ p q))"),
        ]
        .concat()
    }
}
