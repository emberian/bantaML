

fn main() {
    bantamlisp::main()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::main;

    /// Run main, it might makes sense to test it sometimes. Here it's mostly to improve coverage.
    #[test]
    fn test_main() {
        assert_eq!(main(), ());
    }
}
