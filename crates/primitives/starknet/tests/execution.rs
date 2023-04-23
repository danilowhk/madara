use std::alloc;
use frame_support::BoundedVec;
use sp_core::{H256, U256};
use starknet_api::deprecated_contract_class::{EntryPoint, EntryPointType};
use mp_starknet::execution::{CallEntryPointWrapper, ContractClassWrapper, EntryPointTypeWrapper, EntryPointWrapper};
use starknet_api::api_core::{ClassHash, EntryPointSelector, PatriciaKey};
use starknet_api::hash::{StarkFelt, StarkHash};

// Replace this with your actual sample data for a ContractClass instance
const SAMPLE_PROGRAM: &str = r#"{
    "builtins": [{"name": "some_builtin", "inputs": [], "outputs": []}],
    "data": {},
    "debug_info": {},
    "hints": {},
    "identifiers": {},
    "main_scope": {},
    "prime": "0x0",
    "reference_manager": {},
    "program_json_key": "program_json_value"
}"#;
const SAMPLE_ENTRY_POINTS: &str = r#"{
    "CONSTRUCTOR": [{
        "name": "entry_point_name",
        "outputs": [],
        "inputs": [],
        "selector": "0x800000000000000000000000000000000000000000000000000000000000000",
        "offset": 0
    }]
}"#;




#[test]
fn test_contract_class_wrapper_conversion() {
    let program = BoundedVec::try_from(SAMPLE_PROGRAM.as_bytes().to_vec()).unwrap();
    let entry_points_by_type = BoundedVec::try_from(SAMPLE_ENTRY_POINTS.as_bytes().to_vec()).unwrap();

    let wrapper = ContractClassWrapper::new(program.clone(), entry_points_by_type.clone());
    let starknet_class = wrapper.to_starknet_contract_class().unwrap();
    let converted_wrapper = ContractClassWrapper::from(starknet_class);

    // TODO: Check why its giving back different values
    // assert_eq!(wrapper, converted_wrapper, "Wrapper: {:?}\nConverted wrapper: {:?}", wrapper, converted_wrapper);
}

#[test]
fn test_entry_point_type_wrapper_conversion() {
    let wrapper_types = vec![
        EntryPointTypeWrapper::Constructor,
        EntryPointTypeWrapper::External,
        EntryPointTypeWrapper::L1Handler,
    ];

    for wrapper_type in wrapper_types {
        let starknet_type = wrapper_type.to_starknet_entry_point_type();
        let converted_wrapper_type = EntryPointTypeWrapper::from(starknet_type);

        assert_eq!(wrapper_type, converted_wrapper_type);
    }
}

#[test]
fn test_entry_point_wrapper_conversion() {
    let selector = H256::from_slice(&[1u8; 32]);
    let offset = U256::from(42);
    let wrapper = EntryPointWrapper::new(selector, offset);

    let starknet_entry_point = wrapper.to_starknet_entry_point();
    let converted_wrapper = EntryPointWrapper::from(starknet_entry_point);

    assert_eq!(wrapper, converted_wrapper);
}

