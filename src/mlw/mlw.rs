use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

pub struct PseudoPointer<T>
where
    T: PartialEq + Clone + Hash + Eq,
{
    map: Vec<(usize, Vec<T>)>,
    typs: Vec<Option<String>>,
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
        self.map.push((id, d));
        self.typs.push(None);
        id
    }

    pub fn add_sub(&mut self, id: usize, ad: Vec<T>) {
        let id = self.map[id].0;
        self.map = self
            .map
            .iter_mut()
            .map(|x| {
                if (*x).0 == id {
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
        let target = self.map[target_id].0;
        for i in 0..self.map.len() {
            if self.map[i].0 == target {
                self.map[i] = self.map[value_id].clone();
            }
        }
    }

    pub fn unify_sub(&mut self, target: T, value: T) {
        for i in 0..self.map.len() {
            if self.map[i].1.contains(&target) {
                let vec = &mut self.map[i].1;
                for j in 0..vec.len() {
                    if vec[j] == target {
                        vec[j] = value.clone();
                    }
                }
            }
        }
    }

    pub fn set_type(&mut self, id: usize, t: String) {
        let id = self.map[id].0;
        self.typs[id] = Some(t);
    }

    pub fn get_pptr(&self) -> &Vec<(usize, Vec<T>)> {
        &self.map
    }

    pub fn get_result(&self) -> &Vec<Option<String>> {
        &self.typs
    }

    pub fn get_result_from_id(&self, id: usize) -> &Option<String> {
        &self.typs[id]
    }
}

pub struct MLWGrammarLeaf<T1, T2>
where
    T1: PartialEq + Clone + Hash + Eq,
{
    pub type_var: MLWTypeVar<T1>,
    pub function: MLWFunction<T2>,
}

impl<T1, T2> MLWGrammarLeaf<T1, T2>
where
    T1: PartialEq + Clone + Hash + Eq,
{
    pub fn new(t: MLWTypeVar<T1>, f: MLWFunction<T2>) -> Self {
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
            let mut shared_pp = pp.lock().unwrap();
            id = shared_pp.add(d);
        }
        Self { pp: pp, id: id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn add_sub(&mut self, d: Vec<T>) {
        let mut shared_pp = self.pp.lock().unwrap();
        shared_pp.add_sub(self.id, d);
    }

    pub fn set_type(&self, t: String) {
        let mut pp = self.pp.lock().unwrap();
        pp.set_type(self.id, t);
    }

    pub fn unify<T2>(&self, tv: &MLWTypeVar<T2>)
    where
        T2: PartialEq + Clone + Hash + Eq,
    {
        let mut pp = self.pp.lock().unwrap();
        pp.unify(self.id, tv.get_id());
    }
    pub fn unify_sub(&self, t1: T, t2: T) {
        let mut pp = self.pp.lock().unwrap();
        pp.unify_sub(t1, t2);
    }
}

pub struct MLWFunction<T> {
    f: Box<dyn Fn(T) -> T>,
}

impl<T> MLWFunction<T> {
    pub fn new(f: Box<dyn Fn(T) -> T>) -> Self {
        Self { f: f }
    }

    pub fn execute_function(&self, arg: T) -> T {
        (self.f)(arg)
    }
}
