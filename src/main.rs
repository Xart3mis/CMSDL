mod fetcher;
use fetcher::fetch_geckodriver;

fn main() {
    println!("{}", fetch_geckodriver().unwrap().to_str().unwrap());
}
