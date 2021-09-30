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
