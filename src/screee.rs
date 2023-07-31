use rand::{seq::SliceRandom, Rng};
use tracing::{event, Level};

const CHARSET: [(char, u8); 3] = [('E', 2), ('e', 7), ('-', 1)];
const MAX_LEN: usize = 100;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    let len: usize = rng.gen::<usize>() % MAX_LEN;
    event!(Level::DEBUG, "Generating Screee of length {}", len);

    (0..len)
        .map(|_| CHARSET.choose_weighted(&mut rng, |el| el.1).unwrap().0)
        .collect()
}
