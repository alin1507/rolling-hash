use rolling_delta::run;

fn main() {
    match run() {
        Ok(delta) => {
            println!("{:?}", delta)
        }
        Err(err) => println!("{:?}", err),
    }
}
