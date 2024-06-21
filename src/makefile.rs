use crate::{common, ARGS};

const FILENAME: &str = "Makefile";
const MAKEFILE_CONTENTS: &str = include_str!("../resources/Makefile.in");
const MAKEFILE_CONTENTS_WITH_WRAPPER: &str = include_str!("../resources/Makefile-wrapper.in");

pub fn create(use_wrapper: bool) {
    let filename = ARGS.root_path().join(FILENAME);
    let contents = if use_wrapper {
        MAKEFILE_CONTENTS_WITH_WRAPPER
    } else {
        MAKEFILE_CONTENTS
    };
    common::write_file(filename, contents);
}
