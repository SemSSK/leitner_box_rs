use std::time::Duration;

use crate::utils::DurationExt;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum BoxNumber {
    #[default]
    Box1,
    Box2,
    Box3,
    Box4,
    Box5,
    Box6,
    Box7(u8),
}

impl BoxNumber {
    pub fn move_to_next(&self) -> BoxNumber {
        type BN = BoxNumber;
        match self {
            BN::Box1 => BN::Box2,
            BN::Box2 => BN::Box3,
            BN::Box3 => BN::Box4,
            BN::Box4 => BN::Box5,
            BN::Box5 => BN::Box6,
            BN::Box6 => BN::Box7(0),
            BN::Box7(n) => BN::Box7(n + 1),
        }
    }

    pub fn show(&self) -> &'static str {
        match self {
            BoxNumber::Box1 => "Box 1",
            BoxNumber::Box2 => "Box 2",
            BoxNumber::Box3 => "Box 3",
            BoxNumber::Box4 => "Box 4",
            BoxNumber::Box5 => "Box 5",
            BoxNumber::Box6 => "Box 6",
            BoxNumber::Box7(_) => "Box 7",
        }
    }

    pub fn get_next_wait_time(&self) -> u128 {
        match self {
            BoxNumber::Box1 => Duration::from_days(1).as_millis(),
            BoxNumber::Box2 => Duration::from_days(2).as_millis(),
            BoxNumber::Box3 => Duration::from_days(3).as_millis(),
            BoxNumber::Box4 => Duration::from_days(4).as_millis(),
            BoxNumber::Box5 => Duration::from_days(7).as_millis(),
            BoxNumber::Box6 => Duration::from_days(10).as_millis(),
            BoxNumber::Box7(_) => Duration::from_days(15).as_millis(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{control::box_number::BoxNumber, utils::DurationExt};
    use std::time::Duration;
    type BN = BoxNumber;

    #[test]
    fn move_to_next_1() {
        assert_eq!(BN::Box2, BN::Box1.move_to_next());
    }
    #[test]
    fn move_to_next_2() {
        assert_eq!(BN::Box3, BN::Box2.move_to_next());
    }
    #[test]
    fn move_to_next_3() {
        assert_eq!(BN::Box4, BN::Box3.move_to_next());
    }
    #[test]
    fn move_to_next_4() {
        assert_eq!(BN::Box5, BN::Box4.move_to_next());
    }
    #[test]
    fn move_to_next_5() {
        assert_eq!(BN::Box6, BN::Box5.move_to_next());
    }
    #[test]
    fn move_to_next_6() {
        assert_eq!(BN::Box7(0), BN::Box6.move_to_next());
    }
    #[test]
    fn move_to_next_7() {
        assert_eq!(BN::Box7(1), BN::Box7(0).move_to_next());
    }

    #[test]
    fn get_next_wait_time_1() {
        assert_eq!(
            Duration::from_days(1).as_millis(),
            BN::Box1.get_next_wait_time()
        );
    }
    #[test]
    fn get_next_wait_time_2() {
        assert_eq!(
            Duration::from_days(2).as_millis(),
            BN::Box2.get_next_wait_time()
        );
    }
    #[test]
    fn get_next_wait_time_3() {
        assert_eq!(
            Duration::from_days(3).as_millis(),
            BN::Box3.get_next_wait_time()
        );
    }
    #[test]
    fn get_next_wait_time_4() {
        assert_eq!(
            Duration::from_days(4).as_millis(),
            BN::Box4.get_next_wait_time()
        );
    }
    #[test]
    fn get_next_wait_time_5() {
        assert_eq!(
            Duration::from_days(7).as_millis(),
            BN::Box5.get_next_wait_time()
        );
    }
    #[test]
    fn get_next_wait_time_6() {
        assert_eq!(
            Duration::from_days(10).as_millis(),
            BN::Box6.get_next_wait_time()
        );
    }
    #[test]
    fn get_next_wait_time_7() {
        assert_eq!(
            Duration::from_days(15).as_millis(),
            BN::Box7(0).get_next_wait_time()
        );
    }
}
