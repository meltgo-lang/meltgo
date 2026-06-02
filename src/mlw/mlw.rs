use std::sync::{Arc, Mutex};

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
        self.map.push((id + 1, d));
        self.typs.push(None);
        id
    }

    pub fn add_sub(&mut self, id: usize, ad: Vec<T>) {
        let (_, d) = &mut self.map[id];
        d.extend(ad);
        //self.map[id] = (*p_id, *d);
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

pub struct MLWGrammarLeaf<'a, T, Arg, Result>
where
    T: PartialEq + Clone,
{
    pub type_var: MLWTypeVar<T>,
    pub function: MLWFunction<'a, Arg, Result>,
}

impl<'a, T, Arg, Result> MLWGrammarLeaf<'a, T, Arg, Result>
where
    T: PartialEq + Clone,
{
    pub fn new(t: MLWTypeVar<T>, f: MLWFunction<'a, Arg, Result>) -> Self {
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

pub struct MLWFunction<'a, Arg, Result> {
    f: &'a dyn Fn(Arg) -> Result,
}

impl<'a, Arg, Result> MLWFunction<'a, Arg, Result> {
    pub fn new(f: &'a dyn Fn(Arg) -> Result) -> Self {
        Self { f: f }
    }

    pub fn execute_function(&self, arg: Arg) -> Result {
        (self.f)(arg)
    }
}
