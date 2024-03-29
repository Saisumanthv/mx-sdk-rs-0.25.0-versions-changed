use super::VmApiImpl;
use dharitri_wasm::{
    api::CallValueApi,
    types::{BigUint, DctTokenType, ManagedType, TokenIdentifier},
};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    fn bigIntNew(value: i64) -> i32;
    #[cfg(not(feature = "unmanaged-ei"))]
    fn mBufferNew() -> i32;

    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);
    fn bigIntGetDCTCallValue(dest: i32);
    fn getDCTTokenName(resultOffset: *const u8) -> i32;
    fn getDCTTokenNonce() -> i64;
    fn getDCTTokenType() -> i32;

    // multi-transfer API
    fn getNumDCTTransfers() -> i32;
    fn bigIntGetDCTCallValueByIndex(dest: i32, index: i32);
    fn getDCTTokenNameByIndex(resultOffset: *const u8, index: i32) -> i32;
    fn getDCTTokenNonceByIndex(index: i32) -> i64;
    fn getDCTTokenTypeByIndex(index: i32) -> i32;
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedGetMultiDCTCallValue(resultHandle: i32);

    /// TODO: decide if it is worth using or not
    #[allow(dead_code)]
    fn getCallValueTokenName(callValueOffset: *const u8, resultOffset: *const u8) -> i32;
}

impl CallValueApi for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn moax_value(&self) -> BigUint<Self> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetCallValue(value_handle);
            BigUint::from_raw_handle(value_handle)
        }
    }

    fn dct_value(&self) -> BigUint<Self> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetDCTCallValue(value_handle);
            BigUint::from_raw_handle(value_handle)
        }
    }

    fn token(&self) -> TokenIdentifier<Self> {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                TokenIdentifier::moax()
            } else {
                TokenIdentifier::from_dct_bytes(&name_buffer[..name_len as usize])
            }
        }
    }

    fn dct_token_nonce(&self) -> u64 {
        unsafe { getDCTTokenNonce() as u64 }
    }

    fn dct_token_type(&self) -> DctTokenType {
        unsafe { (getDCTTokenType() as u8).into() }
    }

    fn dct_num_transfers(&self) -> usize {
        unsafe { getNumDCTTransfers() as usize }
    }

    fn dct_value_by_index(&self, index: usize) -> BigUint<Self> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetDCTCallValueByIndex(value_handle, index as i32);
            BigUint::from_raw_handle(value_handle)
        }
    }

    fn token_by_index(&self, index: usize) -> TokenIdentifier<Self> {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getDCTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);
            if name_len == 0 {
                TokenIdentifier::moax()
            } else {
                TokenIdentifier::from_dct_bytes(&name_buffer[..name_len as usize])
            }
        }
    }

    fn dct_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getDCTTokenNonceByIndex(index as i32) as u64 }
    }

    fn dct_token_type_by_index(&self, index: usize) -> DctTokenType {
        unsafe { (getDCTTokenTypeByIndex(index as i32) as u8).into() }
    }

    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_all_dct_transfers(
        &self,
    ) -> dharitri_wasm::types::ManagedVec<Self, dharitri_wasm::types::DctTokenPayment<Self>> {
        unsafe {
            let result_handle = mBufferNew();
            managedGetMultiDCTCallValue(result_handle);
            dharitri_wasm::types::ManagedVec::from_raw_handle(result_handle)
        }
    }
}
