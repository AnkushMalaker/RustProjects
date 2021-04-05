pub struct ObscureData {
    pub number_of_yeets: i8,
}

impl ObscureData {
    pub fn new(yeets: i8) -> ObscureData {
        let number_of_yeets = yeets;
        ObscureData { number_of_yeets }
    }
}
