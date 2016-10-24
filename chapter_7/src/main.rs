#![feature(box_patterns, box_syntax)]

#[derive(Clone)]
struct Info;

#[derive(Clone)]
struct NameBind;

type Binding = NameBind;

type Context = Vec<(String, Binding)>;

#[derive(Clone)]
enum Term {
    Var(Info, i64, i64),
    Abs(Info, String, Box<Term>),
    App(Info, Box<Term>, Box<Term>),
}

fn print_tm(ctx: Context, t: Term) {
    match t {
        Term::Abs(_, x, box t_1) => {
            let (ctx_2, x_2) = pick_fresh_name(ctx, x);
            print!("(lambda {} . ", x_2);
            print_tm(ctx_2, t_1);
            println!(")");
        },
        Term::App(_, box t_1, box t_2) => {
            print!("(");
            print_tm(ctx.clone(), t_1);
            print!(" ");
            print_tm(ctx, t_2);
            println!(")");
        },
        Term::Var(fi, x, n) => {
            if ctx_length(&ctx) == n {
                println!("{}", index2name(fi, &ctx, x));
                return;
            }
            println!("[bad index]");
        }
    }
}

fn index2name(_: Info, ctx: &Context, x: i64) -> String {
    ctx[x as usize].0.clone()
}

fn ctx_length(ctx: &Context) -> i64 {
    ctx.len() as i64
}
 
fn pick_fresh_name(mut ctx: Context, x: String) -> (Context, String) {
    let x_2 = format!("{}_{}", x, x);
    if contains_key(&ctx, &x_2) {
        return pick_fresh_name(ctx, x_2);
    }
    ctx.push((x_2.clone(), NameBind));
    (ctx, x_2)
}

fn contains_key(ctx: &Context, x: &String) -> bool {
    ctx.iter().any(|e| &e.0 == x)
}

fn term_shift(d: i64, t: Term) -> Term {
    fn walk(d: i64, c: i64, t: Term) -> Term { 
        match t {
            Term::Var(fi, x, n) => {
                if x >= c {
                    return Term::Var(fi, x + d, n + d)
                }
                Term::Var(fi, x, n + d)
            },
            Term::Abs(fi, x, box t_1) => Term::Abs(fi, x, box walk(d, c + 1, t_1)),
            Term::App(fi, box t_1, box t_2) => Term::App(fi, box walk(d, c, t_1), box walk(d, c, t_2)),
        }
    };
    walk(d, 0, t)
}

fn term_subst(j: i64, s: Term, t: Term) -> Term {
    fn walk(j: i64, s: Term, c: i64, t: Term) -> Term { 
        match t {
            Term::Var(fi, x, n) => {
                if x == j + c {
                    return term_shift(c as i64, s)
                }
                Term::Var(fi, x, n)
            },
            Term::Abs(fi, x, box t_1) => Term::Abs(fi, x, box walk(j, s, c + 1, t_1)),
            Term::App(fi, box t_1, box t_2) => Term::App(fi, box walk(j, s.clone(), c, t_1), box walk(j, s, c, t_2)),
        }
    };
    walk(j, s, 0, t)
}

fn term_subst_top(s: Term, t: Term) -> Term {
    term_shift(-1, term_subst(0, term_shift(1, s), t))
}

fn is_val(ctx: &Context, t: &Term) -> bool {
    match t {
        &Term::Abs(_, _, _) => true,
        _ => false,
    }
}

fn eval_1(ctx: &Context, t: Term) -> Option<Term> {
    match t {
        Term::App(_, box Term::Abs(_, _, box ref t_1_2), box ref v_2) if is_val(ctx, v_2) => {
            Some(term_subst_top(v_2.clone(), t_1_2.clone()))
        },
        Term::App(ref fi, box ref v_1, box ref t_2) if is_val(ctx, &v_1) => {
            eval_1(ctx, t_2.clone()).map(|t_2_2| Term::App(fi.clone(), box v_1.clone(), box t_2_2))
        },
        Term::App(fi, box t_1, box t_2) => {
            eval_1(ctx, t_1).map(|t_1_2| Term::App(fi, box t_1_2, box t_2))
        },
        _ => None,
    }
}

fn eval(ctx: &Context, t: Term) -> Term {
    match eval_1(ctx, t.clone()) {
        Some(t_2) => eval(ctx, t_2),
        None => t,
    }
}

fn main() {
    println!("Hello, world!");
}
