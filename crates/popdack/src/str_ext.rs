pub fn skip(s: &String, i: usize) -> String {
    s.chars().skip(i).collect::<String>()
}

pub fn skip_while<F>(s: &String, f: F) -> String
where
    F: FnMut(&char) -> bool,
{
    s.chars().skip_while(f).collect::<String>()
}
