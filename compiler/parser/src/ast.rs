use std::{
    num::{NonZero, NonZeroU32, NonZeroUsize},
    sync::{Arc, Mutex},
};

use crate::{
    meltgo_error::emitter::ErrorState,
    mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer},
};

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Ownership {
    Borrow(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Ref {
    pub ptr: usize,
}

impl Ref {
    pub fn new(ptr: usize) -> Self {
        Self { ptr: ptr }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub line: NonZeroUsize,
    pub column: NonZeroUsize,
    pub sum: NonZeroUsize,
}

#[derive(Debug)]
pub struct ErrorBuf {
    pub errors: Vec<(Position, ErrorState, NonZeroU32, String, String)>,
}

impl ErrorBuf {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn add(
        &mut self,
        pos: Position,
        state: ErrorState,
        num: NonZeroU32,
        msg: String,
        note: String,
    ) {
        self.errors.push((pos, state, num, msg, note));
    }
}

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Number {
        pos: Position,
        value: i32,
    },
    AddOp {
        pos: Position,
        l: Ref,
        r: Ref,
    },
    SubOp {
        pos: Position,
        l: Ref,
        r: Ref,
    },
    Let {
        pos: Position,
        vname: &'a str,
        is_mut: bool,
        expr: Ref,
    },
}

pub enum Function {
    FSingle(
        Box<
            dyn Fn(
                Arc<Mutex<ErrorBuf>>,
                Arc<Mutex<PseudoPointer<Ownership>>>,
            ) -> MLWTypeVar<Ownership>,
        >,
    ),
    FDouble(
        Box<
            dyn Fn(
                Arc<Mutex<ErrorBuf>>,
                Arc<Mutex<PseudoPointer<Ownership>>>,
            ) -> (MLWTypeVar<Ownership>, MLWTypeVar<Ownership>),
        >,
    ),
}

pub struct NodeBuf<'a> {
    pp: Arc<Mutex<PseudoPointer<Ownership>>>,
    pub error_buf: Arc<Mutex<ErrorBuf>>,
    pub buf: Vec<Node<'a>>,
}

impl<'a> NodeBuf<'a> {
    pub fn new(pp: Arc<Mutex<PseudoPointer<Ownership>>>) -> Self {
        Self {
            pp: pp,
            error_buf: Arc::new(Mutex::new(ErrorBuf::new())),
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
            Node::Number { pos: _, value: _ } => Function::FSingle(Box::new(|_, pp| {
                let gl = MLWGrammarLeaf::new(
                    MLWTypeVar::new(Arc::clone(&pp), vec![]),
                    MLWFunction::<MLWTypeVar<Ownership>>::new(Box::new(|x| {
                        x.set_type(String::from("i32"));
                        x
                    })),
                );
                gl.function.execute_function(gl.type_var)
            })),
            Node::AddOp { pos, l, r } => {
                let fl = self.get_function(l.ptr);
                let fr = self.get_function(r.ptr);
                let inpos = *pos;
                Function::FDouble(Box::new(move |err_buf, pp| {
                    let shared_pp = Arc::clone(&pp);
                    let shared_err_buf = Arc::clone(&err_buf);
                    let gl = MLWGrammarLeaf::new(
                        MLWTypeVar::new(Arc::clone(&pp), vec![]),
                        MLWFunction::<(MLWTypeVar<Ownership>, MLWTypeVar<Ownership>)>::new(
                            Box::new(move |(x, y)| {
                                {
                                    let pp = shared_pp.lock().unwrap();
                                    let mut err_buf = shared_err_buf.lock().unwrap();
                                    let tx = pp.get_result_from_id(x.get_id());
                                    let ty = pp.get_result_from_id(y.get_id());
                                    match (tx, ty) {
                                        (Some(t1), Some(t2)) => {
                                            if t1 == t2 {
                                                err_buf.add(
                                                    inpos,
                                                    ErrorState::NormalError,
                                                    NonZeroU32::new(1).unwrap(),
                                                    format!("expcted '{}', found '{}'", t1, t2),
                                                    String::from(""),
                                                );
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
                            Function::FSingle(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)).0,
                        },
                        match &fr {
                            Function::FSingle(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)).0,
                        },
                    ))
                }))
            }
            Node::SubOp { pos, l, r } => {
                let fl = self.get_function(l.ptr);
                let fr = self.get_function(r.ptr);
                let inpos = *pos;
                Function::FDouble(Box::new(move |err_buf, pp| {
                    let shared_pp = Arc::clone(&pp);
                    let shared_err_buf = Arc::clone(&err_buf);
                    let gl = MLWGrammarLeaf::new(
                        MLWTypeVar::new(Arc::clone(&pp), vec![]),
                        MLWFunction::<(MLWTypeVar<Ownership>, MLWTypeVar<Ownership>)>::new(
                            Box::new(move |(x, y)| {
                                {
                                    let pp = shared_pp.lock().unwrap();
                                    let mut err_buf = shared_err_buf.lock().unwrap();
                                    let tx = pp.get_result_from_id(x.get_id());
                                    let ty = pp.get_result_from_id(y.get_id());
                                    match (tx, ty) {
                                        (Some(t1), Some(t2)) => {
                                            if t1 != t2 {
                                                err_buf.add(
                                                    inpos,
                                                    ErrorState::NormalError,
                                                    NonZeroU32::new(1).unwrap(),
                                                    format!("expcted '{}', found '{}'", t1, t2),
                                                    String::from(""),
                                                );
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
                            Function::FSingle(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)).0,
                        },
                        match &fr {
                            Function::FSingle(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)).0,
                        },
                    ))
                }))
            }
            Node::Let {
                pos: _,
                vname: _,
                is_mut: _,
                expr,
            } => {
                let f = self.get_function(expr.ptr);
                Function::FDouble(Box::new(move |err_buf, pp| {
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
                            Function::FSingle(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)),
                            Function::FDouble(f) => f(Arc::clone(&err_buf), Arc::clone(&pp)).0,
                        },
                    ))
                }))
            }
        }
    }
}
