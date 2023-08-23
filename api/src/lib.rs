pub mod block;
pub mod chunk;
pub mod entity;
pub mod event;
pub mod modification;
pub mod world;
pub mod physics;
pub mod player;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
