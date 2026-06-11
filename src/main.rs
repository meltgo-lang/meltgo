mod meltgo_error;
mod mlw;

use std::num::{NonZeroU32, NonZeroUsize};
use std::sync::{Arc, Mutex};

use mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer};

use meltgo_error::emitter::*;
use parser::ast::{ErrorBuf, Function, NodeBuf};

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
enum DU {
    DNone,
    DSome,
}

fn main() {
    print_error(
        ErrorState::ImportantError,
        NonZeroU32::new(1).unwrap(),
        "a",
        "b",
        "src\\main.melg",
        NonZeroUsize::new(1).unwrap(),
        NonZeroUsize::new(1).unwrap(),
    );
    let mut sm = StatementManager::new(
        r"
package main;
import std::fmt;
;
let a = 0;
func main() {
    let b = 1;
}",
    );
    sm.marking();
    println!("{:?}", sm)
}

/*
fn f() -> Result<(), Box<dyn std::error::Error>> {
    let pp = Arc::new(Mutex::new(PseudoPointer::new()));
    let shared_pp = Arc::clone(&pp);
    let mut buf = NodeBuf::new(shared_pp);
    let (_, index) = defvar("let mut b = 0 + 1", &mut buf)?;
    println!("{}, {:?}", index, buf.buf);
    let f = buf.get_function(index);
    match f {
        Function::FSingle(f) => {
            let _ = f(Arc::clone(&buf.error_buf), Arc::clone(&pp));
        }
        Function::FDouble(f) => {
            let _ = f(Arc::clone(&buf.error_buf), Arc::clone(&pp));
        }
    }
    let pp = pp.lock().unwrap();
    println!("pptr: {:?}", pp.get_pptr());
    println!("typs: {:?}", pp.get_result());
    let err_buf = buf.error_buf.lock().unwrap();
    print_error(
        err_buf.errors[0].1,
        err_buf.errors[0].2,
        err_buf.errors[0].3.as_str(),
        err_buf.errors[0].4.as_str(),
        "src\\main.melg",
        err_buf.errors[0].0.line,
        err_buf.errors[0].0.column,
    );
    Ok(())
}
*/

fn g() {
    let pp = PseudoPointer::<DU>::new();
    let share_pp = Arc::new(Mutex::new(pp));

    let gl1 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DSome]),
        MLWFunction::<MLWTypeVar<DU>>::new(Box::new(|x| {
            x.set_type(String::from("Obj"));
            x
        })),
    );
    let gl2 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DNone]),
        MLWFunction::<(MLWTypeVar<DU>, MLWTypeVar<DU>)>::new(Box::new(|(x, y)| {
            y.unify(&x);
            (x, y)
        })),
    );
    let gl3 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DSome]),
        MLWFunction::<(MLWTypeVar<DU>, MLWTypeVar<DU>)>::new(Box::new(|(mut x, y)| {
            y.unify(&x);
            x.add_sub(vec![DU::DNone]);
            (x, y)
        })),
    );

    let res = gl1.function.execute_function(gl1.type_var);
    let (res2, _) = gl2.function.execute_function((gl2.type_var, res));
    let (_, _) = gl3.function.execute_function((gl3.type_var, res2));

    let pp = share_pp.lock().unwrap();
    println!("pptr: {:?}", pp.get_pptr());
    println!("res : {:?}", pp.get_result());
}
