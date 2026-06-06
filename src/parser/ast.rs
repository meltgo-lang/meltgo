use std::sync::{Arc, Mutex};

use crate::{
    mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer},
    parser::ast::Ownership::Borrow,
};

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Ownership {
    Borrow(usize),
}

#[derive(Debug)]
pub struct Ref {
    pub ptr: usize,
}

impl Ref {
    pub fn new(ptr: usize) -> Self {
        Self { ptr: ptr }
    }
}

#[derive(Debug)]
pub enum Node<'a> {
    Number {
        value: i32,
    },
    AddOp {
        l: Ref,
        r: Ref,
    },
    Let {
        vname: &'a str,
        is_mut: bool,
        expr: Ref,
    },
}

pub enum Function {
    FSingle(Box<dyn Fn(Arc<Mutex<PseudoPointer<Ownership>>>) -> MLWTypeVar<Ownership>>),
    FDouble(
        Box<
            dyn Fn(
                Arc<Mutex<PseudoPointer<Ownership>>>,
            ) -> (MLWTypeVar<Ownership>, MLWTypeVar<Ownership>),
        >,
    ),
}

pub struct NodeBuf<'a> {
    pp: Arc<Mutex<PseudoPointer<Ownership>>>,
    pub buf: Vec<Node<'a>>,
}

impl<'a> NodeBuf<'a> {
    pub fn new(pp: Arc<Mutex<PseudoPointer<Ownership>>>) -> Self {
        Self {
            pp: pp,
            buf: vec![],
        }
    }

    pub fn push(&mut self, node: Node<'a>) -> usize {
        let size = self.buf.len();
        self.buf.push(node);
        size
    }

    pub fn get_function(&self, id: usize) -> Function {
        match &self.buf[id] {
            Node::Number { value: _ } => Function::FSingle(Box::new(|pp| {
                let gl = MLWGrammarLeaf::new(
                    MLWTypeVar::new(Arc::clone(&pp), vec![]),
                    MLWFunction::<MLWTypeVar<Ownership>>::new(Box::new(|x| {
                        x.set_type(String::from("i32"));
                        x
                    })),
                );
                gl.function.execute_function(gl.type_var)
            })),
            Node::AddOp { l, r } => {
                let fl = self.get_function(l.ptr);
                let fr = self.get_function(r.ptr);
                Function::FDouble(Box::new(move |pp| {
                    let shared_pp = Arc::clone(&pp);
                    let gl = MLWGrammarLeaf::new(
                        MLWTypeVar::new(Arc::clone(&pp), vec![]),
                        MLWFunction::<(MLWTypeVar<Ownership>, MLWTypeVar<Ownership>)>::new(
                            Box::new(move |(x, y)| {
                                {
                                    let pp = shared_pp.lock().unwrap();
                                    let tx = pp.get_result_from_id(x.get_id());
                                    let ty = pp.get_result_from_id(y.get_id());
                                    match (tx, ty) {
                                        (Some(t1), Some(t2)) => {
                                            if t1 != t2 {
                                                panic!()
                                            }
                                        }
                                        _ => panic!(),
                                    }
                                }
                                (x, y)
                            }),
                        ),
                    );
                    gl.function.execute_function((
                        match &fl {
                            Function::FSingle(f) => f(Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&pp)).0,
                        },
                        match &fr {
                            Function::FSingle(f) => f(Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&pp)).0,
                        },
                    ))
                }))
            }
            Node::Let {
                vname: _,
                is_mut: _,
                expr,
            } => {
                let f = self.get_function(expr.ptr);
                Function::FDouble(Box::new(move |pp| {
                    let gl = MLWGrammarLeaf::new(
                        MLWTypeVar::new(Arc::clone(&pp), vec![]),
                        MLWFunction::<(MLWTypeVar<Ownership>, MLWTypeVar<Ownership>)>::new(
                            Box::new(|(x, y)| {
                                x.unify_sub(Borrow(x.get_id()), Borrow(y.get_id()));
                                x.unify(&y);
                                (x, y)
                            }),
                        ),
                    );
                    gl.function.execute_function((
                        gl.type_var,
                        match &f {
                            Function::FSingle(f) => f(Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&pp)).0,
                        },
                    ))
                }))
            }
        }
    }
}
