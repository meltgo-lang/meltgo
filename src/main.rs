mod mlw;
mod parser;

use std::sync::{Arc, Mutex};

use mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer};

#[derive(Clone, PartialEq, Debug)]
enum DU {
    DNone,
    DSome,
}

fn main() {
    let pp = PseudoPointer::<DU>::new();
    let share_pp = Arc::new(Mutex::new(pp));

    let mut gl1 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DSome]),
        MLWFunction::<DU, MLWTypeVar<DU>>::new(Arc::clone(&share_pp), &|_, mut x| {
            x.set_type(String::from("Obj"));
            x.add_sub(vec![DU::DNone]);
            x
        }),
    );
    let mut gl2 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DNone]),
        MLWFunction::<DU, (MLWTypeVar<DU>, MLWTypeVar<DU>)>::new(
            Arc::clone(&share_pp),
            &|_, (x, y)| {
                y.unification(&x);
                (x, y)
            },
        ),
    );
    let mut gl3 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DSome]),
        MLWFunction::<DU, (MLWTypeVar<DU>, MLWTypeVar<DU>)>::new(
            Arc::clone(&share_pp),
            &|_, (x, y)| {
                y.unification(&x);
                (x, y)
            },
        ),
    );

    gl1.type_var = gl1.function.execute_function(gl1.type_var);
    (gl2.type_var, gl1.type_var) = gl2.function.execute_function((gl2.type_var, gl1.type_var));
    (gl3.type_var, gl2.type_var) = gl3.function.execute_function((gl3.type_var, gl2.type_var));

    let pp = share_pp.lock().unwrap();
    println!("pptr: {:?}", pp.get_pptr());
    println!("res : {:?}", pp.get_result());
}
