use super::base::Base;
use super::traits::create::Create;
use super::traits::get::Get;
use super::traits::delete::Delete;
use super::traits::edit::Edit;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: String) -> Pending {
        let base: Base = Base::new(input_title, String::from("pending"));
        
        return Pending { super_struct: base }
    }
}

impl Create for Pending {}
impl Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
