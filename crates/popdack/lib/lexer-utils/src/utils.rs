fn skip(s: &String, i: usize) -> String {
    s.chars().skip(i).collect::<String>()
}

pub fn lign<'a>() -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a {
    |input: &str| Ok((input.to_string(), vec![]))
}

pub fn lstring<'a, F>(
    s: &'a str,
    f: F,
) -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a
where
    F: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
{
    move |input: &str| {
        if input.starts_with(s) {
            let mut rvec = vec![s.to_string()];
            let s2 = skip(&input.to_string(), s.chars().count());
            let s2 = s2.as_str();
            let res = f(s2);
            match res {
                Ok((rest, vec)) => {
                    for item in vec.iter() {
                        rvec.push(item.clone());
                    }
                    Ok((rest, rvec.clone()))
                }
                Err(e) => Err(e.clone()),
            }
        } else {
            Err(format!("input does not start with '{}'.", s))
        }
    }
}

pub fn lmany0<'a, F>(
    s: &'a str,
    f: F,
) -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a
where
    F: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
{
    move |input: &str| {
        let mut rvec = vec![String::new()];
        let mut input = input.to_string();
        let l = s.chars().count();
        while input.starts_with(s) {
            input = skip(&input, l);
            rvec[0] += &s.to_string();
        }
        if rvec[0].is_empty() {
            rvec.remove(0);
        }
        let res = f(input.as_str());
        match res {
            Ok((rest, vec)) => {
                for item in vec.iter() {
                    rvec.push(item.clone());
                }
                Ok((rest, rvec.clone()))
            }
            res => res,
        }
    }
}

pub fn lfmany0<'a, F>(f: F) -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a
where
    F: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
{
    move |input: &str| {
        let mut rvec = vec![];
        let mut input = input.to_string();
        while let Ok((rest, vec)) = f(input.as_str()) {
            for item in vec.iter() {
                rvec.push(item.clone());
            }
            input = rest.to_string();
        }
        Ok((input, rvec.clone()))
    }
}

pub fn lmany1<'a, F>(
    s: &'a str,
    f: F,
) -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a
where
    F: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
{
    move |input: &str| {
        let mut rvec = vec![String::new()];
        let mut input = input.to_string();
        let l = s.chars().count();
        while input.starts_with(s) {
            input = skip(&input, l);
            rvec[0] += &s.to_string();
        }
        if rvec[0].is_empty() {
            return Err(format!("input does not start with '{}'.", s));
        }
        let res = f(input.as_str());
        match res {
            Ok((rest, vec)) => {
                for item in vec.iter() {
                    rvec.push(item.clone());
                }
                Ok((rest, rvec.clone()))
            }
            res => res,
        }
    }
}

pub fn lfmany1<'a, F>(f: F) -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a
where
    F: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
{
    move |input: &str| {
        let mut rvec = vec![];
        let mut input = input.to_string();
        let mut r = None;
        while let Ok((rest, vec)) = {
            let res = f(input.as_str());
            if let Err(e) = &res {
                r = Some(e.clone());
            }
            res
        } {
            for item in vec.iter() {
                rvec.push(item.clone());
            }
            input = rest.to_string();
        }
        if rvec.is_empty() {
            Err(format!(
                "input does not match any patterns.\n{}",
                r.unwrap(),
            ))
        } else {
            Ok((input, rvec.clone()))
        }
    }
}

pub fn lor<'a, F1, F2>(
    f1: F1,
    f2: F2,
) -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a
where
    F1: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
    F2: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
{
    move |input: &str| {
        let res = f1(input);
        match res {
            Err(e1) => {
                let res2 = f2(input);
                match &res2 {
                    Ok(_) => res2,
                    Err(e2) => Err(format!("{}\nor {}", e1, e2)),
                }
            }
            res => res,
        }
    }
}

pub fn predicate<'a, P, F>(
    p: P,
    f: F,
) -> impl Fn(&str) -> Result<(String, Vec<String>), String> + 'a
where
    P: Fn(&char) -> bool + 'a,
    F: Fn(&str) -> Result<(String, Vec<String>), String> + 'a,
{
    move |input: &str| {
        let mut input = input.to_string();
        let mut rvec = vec![String::new()];
        while let Some(ch) = input.chars().next() {
            if p(&ch) {
                input = skip(&input, 1);
                rvec[0] += &ch.to_string();
            } else {
                break;
            }
        }
        if rvec[0].is_empty() {
            return Err(String::from("input does not match predicate."));
        }
        let res = f(input.as_str());
        match res {
            Ok((rest, vec)) => {
                for item in vec.iter() {
                    rvec.push(item.clone());
                }
                Ok((rest, rvec.clone()))
            }
            res => res,
        }
    }
}
