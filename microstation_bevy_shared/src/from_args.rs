pub trait Cli<Args> {
    fn from_args(args: &Args) -> Self;
}
