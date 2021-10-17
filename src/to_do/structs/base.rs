use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(input_title: String, input_status: String) -> Base {
        return Base {
            title: input_title.to_string(),
            status: input_status.to_string(),
        };
    }
}

#[cfg(test)]
mod base_test {
    use super::Base;

    #[test]
    fn new() {
        let title = String::from("test title");
        let expected_title = String::from("test title");
        let status = String::from("test status");
        let expected_status = String::from("test status");

        let new_base_struct = Base::new(title, status);
        assert_eq!(expected_title, new_base_struct.title);
        assert_eq!(expected_status, new_base_struct.status);
    }
}
