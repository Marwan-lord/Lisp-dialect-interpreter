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

fn eval_syms(s: &str, st: Rc<RefCell<Store>>) -> Result<Lsymc, String> {
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

fn eval_lsymc(lsymc: &Lsymc, st: Rc<RefCell<Store>>) -> Result<Lsymc, String> {
    let mut current = Box::new(lsymc.clone());
}
