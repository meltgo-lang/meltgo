mod mlw;

use mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer};
use std::sync::{Arc, Mutex};

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
        MLWFunction::<MLWTypeVar<DU>, MLWTypeVar<DU>>::new(&|mut x| {
            x.set_type(String::from("Obj"));
            x.add_sub(vec![DU::DNone]);
            x
        }),
    );
    let mut gl2 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DNone]),
        MLWFunction::<(MLWTypeVar<DU>, MLWTypeVar<DU>), (MLWTypeVar<DU>, MLWTypeVar<DU>)>::new(
            &|(x, y)| {
                x.unification(&y);
                (x, y)
            },
        ),
    );

    gl1.type_var = gl1.function.execute_function(gl1.type_var);
    (gl2.type_var, gl1.type_var) = gl2.function.execute_function((gl2.type_var, gl1.type_var));

    let pp = share_pp.lock().unwrap();
    println!("pptr: {:?}", pp.get_pptr());
    println!("res : {:?}", pp.get_result());
}
