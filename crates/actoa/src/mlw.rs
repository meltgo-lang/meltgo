use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::marker::PhantomData;

pub struct PseudoPointer<T>
where
    T: PartialEq + Clone + Hash + Eq,
{
    map: Vec<usize>,
    typs: Vec<(String, Vec<T>)>,
}

impl<T> PseudoPointer<T>
where
    T: PartialEq + Clone + Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            map: vec![],
            typs: vec![],
        }
    }

    pub fn add(&mut self, d: Vec<T>) -> usize {
        let id = self.map.len();
        self.map.push(id);
        self.typs.push((String::new(), d));
        id
    }

    pub fn add_sub(&mut self, id: usize, ad: Vec<T>) {
        let id = self.map[id];
        self.typs = self
            .typs
            .iter_mut()
            .enumerate()
            .map(|(i, x)| {
                if id == i {
                    let (_, d) = &mut *x;
                    d.extend(ad.clone());
                    let hash: HashSet<_> = d.drain(..).collect();
                    *d = hash.into_iter().collect::<Vec<T>>();
                    x
                } else {
                    x
                }
                .clone()
            })
            .collect();
    }

    pub fn unify(&mut self, target_id: usize, value_id: usize) {
        let target = self.map[target_id];
        for i in 0..self.map.len() {
            if self.map[i] == target {
                self.map[i] = self.map[value_id].clone();
            }
        }
    }

    pub fn unify_sub(&mut self, target: T, value: T) {
        for i in 0..self.typs.len() {
            if self.typs[i].1.contains(&target) {
                let vec = &mut self.typs[i].1;
                for j in 0..vec.len() {
                    if vec[j] == target {
                        vec[j] = value.clone();
                    }
                }
            }
        }
    }

    pub fn set_type(&mut self, id: usize, t: String) {
        let id = self.map[id];
        self.typs[id] = (t, self.typs[id].1.clone());
    }

    pub fn get_pptr(&self) -> &Vec<usize> {
        &self.map
    }

    pub fn get_result(&self) -> &Vec<(String, Vec<T>)> {
        &self.typs
    }

    pub fn get_result_from_id(&self, id: usize) -> &(String, Vec<T>) {
        &self.typs[id]
    }
}

pub struct MLWGrammarLeaf<T1, T2, T3>
where
    T1: PartialEq + Clone + Hash + Eq,
    T3: Fn(T2) -> T2,
{
    pub type_var: MLWTypeVar<T1>,
    pub function: MLWFunction<T2, T3>,
}

impl<T1, T2, T3> MLWGrammarLeaf<T1, T2, T3>
where
    T1: PartialEq + Clone + Hash + Eq,
    T3: Fn(T2) -> T2,
{
    pub fn new(t: MLWTypeVar<T1>, f: MLWFunction<T2, T3>) -> Self {
        Self {
            type_var: t,
            function: f,
        }
    }
}

pub struct MLWTypeVar<T>
where
    T: PartialEq + Clone + Hash + Eq,
{
    pub pp: Arc<Mutex<PseudoPointer<T>>>,
    id: usize,
}

impl<T> MLWTypeVar<T>
where
    T: PartialEq + Clone + Hash + Eq,
{
    pub fn new(pp: Arc<Mutex<PseudoPointer<T>>>, d: Vec<T>) -> Self {
        let id: usize;
        {
            id = pp.lock().unwrap().add(d);
        }
        Self { pp: pp, id: id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn add_sub(&self, d: Vec<T>) {
        self.pp.lock().unwrap().add_sub(self.id, d);
    }

    pub fn set_type(&self, t: String) {
        self.pp.lock().unwrap().set_type(self.id, t);
    }

    pub fn unify<T2>(&self, tv: &MLWTypeVar<T2>)
    where
        T2: PartialEq + Clone + Hash + Eq,
    {
        self.pp.lock().unwrap().unify(self.id, tv.get_id());
    }
    pub fn unify_sub(&self, t1: T, t2: T) {
        self.pp.lock().unwrap().unify_sub(t1, t2);
    }
}

pub struct MLWFunction<T, F>
where
    F: Fn(T) -> T,
{
    f: F,
    _marker: PhantomData<T>,
}

impl<T, F> MLWFunction<T, F>
where
    F: Fn(T) -> T,
{
    pub fn new(f: F) -> Self {
        Self { f: f, _marker: PhantomData.clone(), }
    }

    pub fn execute_function(&self, arg: T) -> T {
        (self.f)(arg)
    }
}
