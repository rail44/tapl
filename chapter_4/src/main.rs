#![feature(box_patterns, box_syntax)]

#[derive(Clone)]
struct Info;

const DUMMY_INFO:Info = Info;

#[derive(Clone)]
enum Term {
    True(Info),
    False(Info),
    If(Info, Box<Term>, Box<Term>, Box<Term>),
    Zero(Info),
    Succ(Info, Box<Term>),
    Pred(Info, Box<Term>),
    IsZero(Info, Box<Term>),
}

fn is_numeric_val(t: &Term) -> bool {
    match t {
        &Term::Zero(_) => true,
        &Term::Succ(_, ref t_1) => is_numeric_val(t_1),
        _ => false,
    }
}

fn is_val(t: &Term) -> bool {
    match t {
        &Term::True(_) => true,
        &Term::False(_) => true,
        t if is_numeric_val(t) => true,
        _ => false,
    }
}

fn eval_1(t: Term) -> Term {
    match t {
        Term::If(_, box Term::True(_), box t_2, _) => t_2,
        Term::If(_, box Term::False(_), _, box t_3) => t_3,
        Term::If(info, box t_1, t_2, t_3) => {
            let t_1_2 = eval_1(t_1);
            Term::If(info, box t_1_2, t_2, t_3)
        },
        Term::Succ(info, box t_1) => {
            let t_1_2 = eval_1(t_1);
            Term::Succ(info, box t_1_2)
        },
        Term::Pred(_, box Term::Zero(_)) =>  Term::Zero(DUMMY_INFO),
        Term::Pred(_, box Term::Succ(_, box ref nv_1)) if is_numeric_val(nv_1) =>  nv_1.clone(),
        Term::Pred(info, box t_1) => {
            let t_1_2 = eval_1(t_1);
            Term::Pred(info, box t_1_2)
        },
        Term::IsZero(_, box Term::Zero(_)) => Term::True(DUMMY_INFO),
        Term::IsZero(_, box Term::Succ(_, box ref nv_1)) if is_numeric_val(nv_1) => Term::False(DUMMY_INFO),
        Term::IsZero(info, box t_1) => {
            let t_1_2 = eval_1(t_1);
            Term::IsZero(info, box t_1_2)
        },
        _ => panic!("NoRuleApplies"),
    }
}

fn main() {
    println!("Hello, world!");
}
