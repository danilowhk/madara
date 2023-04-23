

#[cfg(test)]
mod tests {
    use mp_starknet::block::*;
    use sp_core::{H256, U256};
    use sp_core::Encode;
    use mp_starknet::execution::ContractAddressWrapper;

    fn generate_dummy_header() -> Header {
        Header::new(
            H256::from_slice(&[1; 32]),
            U256::from(1),
            U256::from(2),
            ContractAddressWrapper::default(),
            42,
            0,
            H256::from_slice(&[3; 32]),
            0,
            H256::from_slice(&[4; 32]),
            Some(1),
            Some(U256::from(3)),
        )
    }

    #[test]
    fn test_header_creation() {
        let header = generate_dummy_header();

        assert_eq!(header.parent_block_hash, H256::from_slice(&[1; 32]));
        assert_eq!(header.block_number, U256::from(1));
        assert_eq!(header.global_state_root, U256::from(2));
        assert_eq!(header.sequencer_address, ContractAddressWrapper::default());
        assert_eq!(header.block_timestamp, 42);
        assert_eq!(header.transaction_count, 0);
        assert_eq!(header.transaction_commitment, H256::from_slice(&[3; 32]));
        assert_eq!(header.event_count, 0);
        assert_eq!(header.event_commitment, H256::from_slice(&[4; 32]));
        assert_eq!(header.protocol_version, Some(1));
        assert_eq!(header.extra_data, Some(U256::from(3)));
    }

    #[test]
    fn test_header_hash() {
        let header = generate_dummy_header();
        let expected_hash = H256::from_slice(
            frame_support::Hashable::blake2_256(&header.block_number.encode()).as_slice(),
        );

        assert_eq!(header.hash(), expected_hash);
    }

    #[test]
    fn test_header_clone() {
        let header = generate_dummy_header();
        let cloned_header = header.clone();

        assert_eq!(header, cloned_header);
    }

    #[test]
    fn test_header_default() {
        let default_header = Header::default();
        let expected_header = Header {
            parent_block_hash: H256::default(),
            block_number: U256::default(),
            global_state_root: U256::default(),
            sequencer_address: ContractAddressWrapper::default(),
            block_timestamp: 0,
            transaction_count: 0,
            transaction_commitment: H256::default(),
            event_count: 0,
            event_commitment: H256::default(),
            protocol_version: None,
            extra_data: None,
        };

        assert_eq!(default_header, expected_header);
    }
}
