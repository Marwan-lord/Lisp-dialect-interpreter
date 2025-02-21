use std::{cell::RefCell, rc::Rc};

use crate::{lsymc::Lsymc, store::Store};

fn print_list(list: &[Lsymc], st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let mut new = Vec::new();

    for lsymc in list[1..].iter() {
        new.push(eval_lsymc(lsymc, st)?);
    }

    for lsymc in new.iter() {
        print!("{} ", lsymc);
    }
    println!();
    Ok(Lsymc::Nil)
}

fn eval_len(list: &[Lsymc], st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let lsymc = eval_lsymc(&list[1], st)?;
    match lsymc {
        Lsymc::List(lst) => Ok(Lsymc::Int(lst.len() as i64)),
        Lsymc::ListSyms(lst) => Ok(Lsymc::Int(lst.len() as i64)),
        _ => Err(format!("{} is not a list", lsymc)),
    }
}

fn eval_nil(list: &[Lsymc], st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let lsymc = eval_lsymc(&list[1], st)?;
    match lsymc {
        Lsymc::List(lst) => Ok(Lsymc::Bool(lst.is_empty())),
        Lsymc::ListSyms(lst) => Ok(Lsymc::Bool(list.is_empty())),
        _ => Err(format!("{} is not a nullable list", lsymc)),
    }
}

fn eval_keyword(list: &[Lsymc], st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let head = &list[0];
    match head {
        Lsymc::Def => eval_def(list, st),
        Lsymc::Defn => eval_defn(list, st),
        _ => Err(format!("Unknown Keyword")),
    }
}

fn eval_syms(s: &str, st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let val = match s {
        "#true" => return Ok(Lsymc::Bool(true)),
        "#false" => return Ok(Lsymc::Bool(false)),
        "#nil" => return Ok(Lsymc::Nil),
        _ => st.borrow_mut().get(s),
    };
    if val.is_none() {
        return Err(format!("Unbound syms: {}", s));
    }

    Ok(val.unwrap().clone())
}

fn eval_lsymc(lsymc: &Lsymc, st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let mut current = Box::new(lsymc.clone());

}

fn eval_def(list: &[Lsymc], st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let mut res = Lsymc::Nil;

    let bind_st = Rc::new(RefCell::new(Store::new()));

    if list.len() < 3 {
        return Err("Invalid def statement".to_owned());
    }


    let binds = match list[1].clone() {
        Lsymc::List(binds) => binds,
        _ => { return Err("Invalid def binds".to_owned()); }
    };

    for bind in binds.iter() {
        let bind = match bind {
            Lsymc::List(bind) => bind,
            _ => {
                return Err("Invalid def binding".to_owned());
            }

        };

        if bind.len() != 2 {
            return Err("Invalid binding for def ".to_owned());
        }

        let name = match bind[0].clone() {
            Lsymc::Ident(name) => name,
            _ => {
                return Err("Invalid def binding".to_owned());
            }
        };

        let value = eval_lsymc(&bind[1], st)?;
        bind_st.borrow_mut().set(name.as_str(), value);
    }
    println!("def args: {:?}", bind_st);

    let mut new_st = Rc::new(RefCell::new(Store::extend(st.clone())));
    new_st.borrow_mut().update(bind_st);

    for sym in list[2..].iter() {
        res = eval_lsymc(sym, &mut new_st)?;
    }

    Ok(res)
}

fn eval_defn(list: &[Lsymc], st: &mut Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    return Ok(Lsymc::Nil);
}
