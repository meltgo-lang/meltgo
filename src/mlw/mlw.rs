use std::sync::{Arc, Mutex, MutexGuard};

pub struct PseudoPointer<T>
where
    T: PartialEq + Clone,
{
    map: Vec<(usize, Vec<T>)>,
    typs: Vec<Option<String>>,
}

impl<T> PseudoPointer<T>
where
    T: PartialEq + Clone,
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
        let (_, d) = &mut self.map[id];
        d.extend(ad);
    }

    pub fn unification(&mut self, target_id: usize, value_id: usize) {
        let target = &self.map[target_id];
        let value = &self.map[value_id];
        self.map = self
            .map
            .iter()
            .map(|x| {
                if *x == *target {
                    value.clone()
                } else {
                    x.clone()
                }
            })
            .collect();
    }

    pub fn set_type(&mut self, id: usize, t: String) {
        self.typs[id] = Some(t);
    }

    pub fn get_pptr(&self) -> &Vec<(usize, Vec<T>)> {
        &self.map
    }

    pub fn get_result(&self) -> &Vec<Option<String>> {
        &self.typs
    }
}

pub struct MLWGrammarLeaf<'a, T1, T2>
where
    T1: PartialEq + Clone,
{
    pub type_var: MLWTypeVar<T1>,
    pub function: MLWFunction<'a, T1, T2>,
}

impl<'a, T1, T2> MLWGrammarLeaf<'a, T1, T2>
where
    T1: PartialEq + Clone,
{
    pub fn new(t: MLWTypeVar<T1>, f: MLWFunction<'a, T1, T2>) -> Self {
        Self {
            type_var: t,
            function: f,
        }
    }
}

pub struct MLWTypeVar<T>
where
    T: PartialEq + Clone,
{
    pp: Arc<Mutex<PseudoPointer<T>>>,
    id: usize,
}

impl<T> MLWTypeVar<T>
where
    T: PartialEq + Clone,
{
    pub fn new(pp: Arc<Mutex<PseudoPointer<T>>>, d: Vec<T>) -> Self {
        let id: usize;
        {
            let mut shared_pp = pp.lock().unwrap();
            id = shared_pp.add(d);
        }
        Self { pp: pp, id: id }
    }

    fn get_id(&self) -> usize {
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

    pub fn unification<T2>(&self, tv: &MLWTypeVar<T2>)
    where
        T2: PartialEq + Clone,
    {
        let mut pp = self.pp.lock().unwrap();
        pp.unification(self.id, tv.get_id());
    }
}

pub struct MLWFunction<'a, T1, T2>
where
    T1: PartialEq + Clone,
{
    pp: Arc<Mutex<PseudoPointer<T1>>>,
    f: &'a dyn Fn(MutexGuard<PseudoPointer<T1>>, T2) -> T2,
}

impl<'a, T1, T2> MLWFunction<'a, T1, T2>
where
    T1: PartialEq + Clone,
{
    pub fn new(
        pp: Arc<Mutex<PseudoPointer<T1>>>,
        f: &'a dyn Fn(MutexGuard<PseudoPointer<T1>>, T2) -> T2,
    ) -> Self {
        Self { pp: pp, f: f }
    }

    pub fn execute_function(&self, arg: T2) -> T2 {
        let pp = self.pp.lock().unwrap();
        (self.f)(pp, arg)
    }
}
