use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: String) -> Done {
        let base: Base = Base::new(input_title, String::from("done"));
        
        return Done { super_struct: base }
    }
}

#[cfg(test)]
mod done_test {
    use super::Done;

    #[test]
    fn new() {
        let expected_status = String::from("done");
        let title = String::from("excel date");
        let expected_title = String::from("excel date");
        let done = Done::new(title);

        assert_eq!(expected_status, done.super_struct.status);
        assert_eq!(expected_title, done.super_struct.title);
    }
}
