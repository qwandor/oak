//
// Copyright 2020 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#[test]
fn test_app_config() {
    let cfg = crate::application_configuration(
        vec![("node".to_string(), vec![0x00, 0x01])],
        "lumberjack",
        "node",
        "main",
    );
    let got = format!("{:?}", cfg);
    assert_eq!(
        "node_configs {name: \"node\" wasm_config {module_bytes: \"\\000\\001\"}} node_configs {name: \"lumberjack\" log_config {}} initial_node_config_name: \"node\" initial_entrypoint_name: \"main\"", got);
}

#[test]
fn test_app_config_multi() {
    let cfg = crate::application_configuration(
        vec![
            ("node".to_string(), vec![0x00, 0x01]),
            ("another_node".to_string(), vec![0x02, 0x03]),
        ],
        "lumberjack",
        "node",
        "main",
    );
    let got = format!("{:?}", cfg);
    assert_eq!(
        "node_configs {name: \"node\" wasm_config {module_bytes: \"\\000\\001\"}} node_configs {name: \"another_node\" wasm_config {module_bytes: \"\\002\\003\"}} node_configs {name: \"lumberjack\" log_config {}} initial_node_config_name: \"node\" initial_entrypoint_name: \"main\"", got);
}

#[test]
fn test_app_config_no_logger() {
    let cfg = crate::application_configuration(
        vec![("node".to_string(), vec![0x00, 0x01])],
        "",
        "node",
        "main",
    );
    let got = format!("{:?}", cfg);
    assert_eq!(
        "node_configs {name: \"node\" wasm_config {module_bytes: \"\\000\\001\"}} initial_node_config_name: \"node\" initial_entrypoint_name: \"main\"", got);
}
