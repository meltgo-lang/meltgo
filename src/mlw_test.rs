use actoa::mlw::*;

use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
enum DU {
    DNone,
    DSome,
}

pub fn g() {
    let pp = PseudoPointer::<DU>::new();
    let share_pp = Arc::new(Mutex::new(pp));

    let gl1 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DSome]),
        MLWFunction::<MLWTypeVar<DU>, _>::new(|x| {
            x.set_type(String::from("Obj"));
            x
        }),
    );
    let gl2 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DNone]),
        MLWFunction::<(MLWTypeVar<DU>, MLWTypeVar<DU>), _>::new(|(x, y)| {
            x.unify(&y);
            (x, y)
        }),
    );
    let gl3 = MLWGrammarLeaf::new(
        MLWTypeVar::new(Arc::clone(&share_pp), vec![DU::DSome]),
        MLWFunction::<(MLWTypeVar<DU>, MLWTypeVar<DU>), _>::new(|(x, y)| {
            x.unify(&y);
            x.add_sub(vec![DU::DNone]);
            (x, y)
        }),
    );

    let res = gl1.function.execute_function(gl1.type_var);
    let (res2, _) = gl2.function.execute_function((gl2.type_var, res));
    let (_, _) = gl3.function.execute_function((gl3.type_var, res2));

    let pp = share_pp.lock().unwrap();
    println!("pptr: {:?}", pp.get_pptr());
    println!("res : {:?}", pp.get_result());
}
