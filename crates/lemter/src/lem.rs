pub fn word<'a>(
    w: &'a str,
) -> impl Fn((&'a str, Vec<&'a str>)) -> Result<(&'a str, Vec<&'a str>), &'a str> {
    move |(input, mut result)| {
        if input.starts_with(w) {
            result.push(w);

            Ok((
                match input.char_indices().nth(w.len()) {
                    Some((idx, _)) => &input[idx..],
                    None => "",
                },
                result,
            ))
        } else {
            Err("")
        }
    }
}
