use cargo_snippet::snippet;

#[snippet("template")]
#[snippet(include = "scan")]
fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let mut sc = Scanner::new(stdin.lock());
    let mut out = std::io::BufWriter::new(stdout.lock());

    solve(&mut sc, &mut out);

    out.flush().ok().unwrap();
}

#[snippet("template")]
fn solve(sc: &mut Scanner<std::io::StdinLock>, out: &mut std::io::BufWriter<std::io::StdoutLock>) {
    todo!()
}
