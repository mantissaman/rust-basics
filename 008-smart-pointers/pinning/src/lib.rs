// Pin means promising that yiu wont move the bytes.
struct OwnedParsed{
    buf: Vec<u8>,
    repr: Parsed<'self>,
}

struct Parsed<'a>{
    name: &'a str,
}

fn parse<'a>(input: &'a [u8]) ->Result<Parsed<'a>, _>{
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
