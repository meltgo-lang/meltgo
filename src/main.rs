mod mlw;
mod parser;

use std::sync::{Arc, Mutex};

use mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer};
use parser::parser::defvar;

use parser::ast::{MeltgoFunction, MeltgoNodeBuf};

#[derive(Clone, PartialEq, Debug)]
enum DU {
    DNone,
    DSome,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pp = Arc::new(Mutex::new(PseudoPointer::new()));
    let shared_pp = Arc::clone(&pp);
    let mut buf = MeltgoNodeBuf::new(shared_pp);
    let (_, index) = defvar("let mut b = 0 + 1", &mut buf)?;
    println!("{}, {:?}", index, buf.buf);
    let f = buf.get_function(index);
    match f {
        MeltgoFunction::FSingle(f) => {
            let _ = f(Arc::clone(&pp));
        }
        MeltgoFunction::FDouble(f) => {
            let _ = f(Arc::clone(&pp));
        }
    }
    let pp = pp.lock().unwrap();
    println!("pptr: {:?}", pp.get_pptr());
    println!("typs: {:?}", pp.get_result());
    Ok(())
}

fn f() {
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
            y.unification(&x);
            (x, y)
        })),
    );
    let gl3 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DSome]),
        MLWFunction::<(MLWTypeVar<DU>, MLWTypeVar<DU>)>::new(Box::new(|(mut x, y)| {
            y.unification(&x);
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
