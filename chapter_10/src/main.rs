#![feature(box_syntax, box_patterns)]

#[derive(Clone)]
struct Info;
type Context = Vec<(String, Binding)>;

#[derive(Clone)]
enum Binding {
    Name,
    Var(Type),
}

fn add_binding(ctx: &mut Context, x: String, bind: Binding) {
    ctx.push((x, bind));
}

#[derive(Clone, PartialEq, Eq)]
enum Type {
    Arr(Box<Type>, Box<Type>),
    Bool,
}

enum Term {
    Var(Info, i64, i64),
    Abs(Info, String, Type, Box<Term>),
    App(Info, Box<Term>, Box<Term>),
    True(Info),
    False(Info),
    If(Info, Box<Term>, Box<Term>, Box<Term>),
}

struct Error(Info, String);

fn index_to_name(_: &Info, ctx: &Context, x: i64) -> String {
    ctx[x as usize].0.clone()
}

fn get_binding(fi: &Info, ctx: &Context, i: i64) -> Binding {
    ctx[i as usize].1.clone()
}

fn get_type_from_context(fi: Info, ctx: &Context, i: i64) -> Result<Type, Error> {
    match get_binding(&fi, ctx, i) {
        Binding::Var(ty) => Ok(ty),
        _ => Err(
            Error(
                fi.clone(),
                format!("get_type_from_context: Wrong kind of binding for variable, {}", index_to_name(&fi, ctx, i))
            )
        ),
    }
}

fn type_of(ctx: &mut Context, t: Term) -> Result<Type, Error> {
    match t {
        Term::Var(fi, i, _) => get_type_from_context(fi, ctx, i),
        Term::Abs(fi, x, ty_t1, box t2) => {
            add_binding(ctx, x, Binding::Var(ty_t1.clone())); 
            let ty_t2 = try!(type_of(ctx, t2));
            Ok(Type::Arr(box ty_t1, box ty_t2))
        },
        Term::App(fi, box t1, box t2) => {
            let ty_t1 = try!(type_of(ctx, t1));
            let ty_t2 = try!(type_of(ctx, t2));
            match ty_t1 {
                Type::Arr(box ty_t11, box ty_t12) => {
                    if ty_t2 == ty_t11 {
                        return Ok(ty_t12);
                    }
                    Err(Error(fi, "parameter type mismatch".to_string()))
                },
                _ => Err(Error(fi, "arrow type expected".to_string())),
            }
        },
        Term::True(fi) => Ok(Type::Bool),
        Term::False(fi) => Ok(Type::Bool),
        Term::If(fi, box t1, box t2, box t3) => {
            if try!(type_of(ctx, t1)) != Type::Bool {
                return Err(Error(fi, "guard of conditional not boolean".to_string()));
            }
            let ty_t2 = try!(type_of(ctx, t2));
            if ty_t2 == try!(type_of(ctx, t3)) {
                return Ok(ty_t2);
            }
            Err(Error(fi, "arms of conditional have different types".to_string()))
        }
    }
}

fn main() {
    println!("Hello, world!");
}


