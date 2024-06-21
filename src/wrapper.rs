use crate::{common, ARGS};

const FILENAME: &str = "helper/wrapper.rs";
const WRAPPER_CONTENTS: &str = include_str!("../resources/wrapper.rs.in");

pub fn create() {
    let filename = ARGS.root_path().join(FILENAME);
    let helper_folder = filename.parent().expect("should have parent");
    common::create_dir_all(helper_folder);
    common::write_file(filename, WRAPPER_CONTENTS);
}
