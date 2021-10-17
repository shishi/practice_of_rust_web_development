use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: String) -> Pending {
        let base: Base = Base::new(input_title, String::from("pending"));
        
        return Pending { super_struct: base }
    }
}

#[cfg(test)]
mod pending_tests {
    use super::Pending;

    #[test]
    fn new() {
        let expected_status = String::from("pending");
        let title = String::from("washing");
        let expected_title = String::from("washing");
        let pending = Pending::new(title);

        assert_eq!(expected_status, pending.super_struct.status);
        assert_eq!(expected_title, pending.super_struct.title);
    }
}
