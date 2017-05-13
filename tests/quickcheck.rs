extern crate yaml_rust;
#[macro_use]
extern crate quickcheck;

use quickcheck::TestResult;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use std::error::Error;

quickcheck! {
    fn test_check_weird_keys(xs: Vec<String>) -> TestResult {
        let doc = Yaml::Array(xs.into_iter().map(Yaml::String).collect());
        let out_str = YamlEmitter::dump_all_into_string(&[&doc]).unwrap();
        if let Err(err) = YamlLoader::load_from_str(&out_str) {
            return TestResult::error(err.description());
        }
        TestResult::passed()
    }
}
