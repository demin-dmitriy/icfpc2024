use solver::submission_api::communicate;

fn main() {
    let text = "S'%4}).$%8";
    let response = communicate(text);
    println!("Response for {}:\n {}", text, response);
}