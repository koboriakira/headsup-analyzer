pub mod drawhand;

pub mod board;

pub mod analyse;

pub mod player;

pub mod position;

pub mod action;

pub mod cards;

pub mod argparse;

pub mod madehand;

pub mod straight;

pub mod range;

pub mod hand_wrapper;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
