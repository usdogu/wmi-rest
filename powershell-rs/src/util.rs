use rand::{thread_rng, Rng, distributions::Alphanumeric};
pub(crate) fn create_boundary() -> String {
    let rand_str = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect::<String>();
    format!("$pwsh{rand_str}$")
}