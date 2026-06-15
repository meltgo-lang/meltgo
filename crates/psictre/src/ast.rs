use std::{
    hash::Hash,
    num::{NonZeroU32, NonZeroUsize},
    sync::{Arc, Mutex},
};

use actoa::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer};
use suzlun_errors::*;

pub trait Node<T1, T2>
where
    T1: Clone + Hash + Eq + PartialEq,
    T2: Node<T1, T2>,
{
    fn mapping(&self, node_buf: &NodeBuf<T1, T2>) -> Function<T1>;
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

pub enum Function<T>
where
    T: Clone + Hash + Eq + PartialEq,
{
    FSingle(Box<dyn Fn(Arc<Mutex<ErrorBuf>>, Arc<Mutex<PseudoPointer<T>>>) -> MLWTypeVar<T>>),
    FDouble(
        Box<
            dyn Fn(
                Arc<Mutex<ErrorBuf>>,
                Arc<Mutex<PseudoPointer<T>>>,
            ) -> (MLWTypeVar<T>, MLWTypeVar<T>),
        >,
    ),
}

pub struct NodeBuf<T1, T2>
where
    T1: Clone + Hash + Eq + PartialEq,
    T2: Node<T1, T2>,
{
    pp: Arc<Mutex<PseudoPointer<T1>>>,
    pub error_buf: Arc<Mutex<ErrorBuf>>,
    pub buf: Vec<T2>,
}

impl<T1, T2> NodeBuf<T1, T2>
where
    T1: Clone + Hash + Eq + PartialEq,
    T2: Node<T1, T2>,
{
    pub fn new(pp: Arc<Mutex<PseudoPointer<T1>>>) -> Self {
        Self {
            pp: pp,
            error_buf: Arc::new(Mutex::new(ErrorBuf::new())),
            buf: vec![],
        }
    }

    pub fn push(&mut self, node: T2) -> usize {
        let size = self.buf.len();
        self.buf.push(node);
        size
    }

    pub fn get_function(&self, id: usize) -> Function<T1> {
        self.buf[id].mapping(&self)
    }
}
