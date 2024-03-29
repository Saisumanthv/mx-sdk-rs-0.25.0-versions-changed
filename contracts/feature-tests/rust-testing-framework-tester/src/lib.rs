#![no_std]

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct NftDummyAttributes {
    pub creation_epoch: u64,
    pub cool_factor: u8,
}

pub struct StructWithManagedTypes<M: ManagedTypeApi> {
    pub big_uint: BigUint<M>,
    pub buffer: ManagedBuffer<M>,
}

#[dharitri_wasm::derive::contract]
pub trait RustTestingFrameworkTester {
    #[init]
    fn init(&self) {
        self.total_value().set(&BigUint::from(1u32));
    }

    #[endpoint]
    fn sum(&self, first: BigUint, second: BigUint) -> BigUint {
        first + second
    }

    #[endpoint]
    fn sum_sc_result(&self, first: BigUint, second: BigUint) -> SCResult<BigUint> {
        require!(first > 0 && second > 0, "Non-zero required");
        Ok(first + second)
    }

    #[endpoint]
    fn get_caller_legacy(&self) -> Address {
        self.blockchain().get_caller_legacy()
    }

    #[endpoint]
    fn get_moax_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::moax(), 0)
    }

    #[endpoint]
    fn get_dct_balance(&self, token_id: TokenIdentifier, nonce: u64) -> BigUint {
        self.blockchain().get_sc_balance(&token_id, nonce)
    }

    #[payable("MOAX")]
    #[endpoint]
    fn receive_moax(&self) -> BigUint {
        self.call_value().moax_value()
    }

    #[payable("MOAX")]
    #[endpoint]
    fn recieve_moax_half(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().moax_value() / 2u32;
        self.send()
            .direct(&caller, &TokenIdentifier::moax(), 0, &payment_amount, &[]);
    }

    #[payable("*")]
    #[endpoint]
    fn receive_dct(&self) -> (TokenIdentifier, BigUint) {
        let token_id = self.call_value().token();
        let amount = self.call_value().dct_value();

        (token_id, amount)
    }

    #[payable("*")]
    #[endpoint]
    fn receive_dct_half(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.call_value().token();
        let amount = self.call_value().dct_value() / 2u32;

        self.send().direct(&caller, &token_id, 0, &amount, &[]);
    }

    #[payable("*")]
    #[endpoint]
    fn receive_multi_dct(&self) -> ManagedVec<DctTokenPayment<Self::Api>> {
        self.call_value().all_dct_transfers()
    }

    #[payable("*")]
    #[endpoint]
    fn send_nft(
        &self,
        to: ManagedAddress,
        token_id: TokenIdentifier,
        nft_nonce: u64,
        amount: BigUint,
    ) {
        self.send().direct(&to, &token_id, nft_nonce, &amount, &[]);
    }

    #[endpoint]
    fn mint_dct(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().dct_local_mint(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn burn_dct(&self, token_id: TokenIdentifier, nonce: u64, amount: BigUint) {
        self.send().dct_local_burn(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn create_nft(
        &self,
        token_id: TokenIdentifier,
        amount: BigUint,
        attributes: NftDummyAttributes,
    ) -> u64 {
        self.send().dct_nft_create(
            &token_id,
            &amount,
            &ManagedBuffer::new(),
            &BigUint::zero(),
            &ManagedBuffer::new(),
            &attributes,
            &ManagedVec::new(),
        )
    }

    #[endpoint]
    fn get_block_epoch(&self) -> u64 {
        self.blockchain().get_block_epoch()
    }

    #[endpoint]
    fn get_block_nonce(&self) -> u64 {
        self.blockchain().get_block_nonce()
    }

    #[endpoint]
    fn get_block_timestamp(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    #[endpoint]
    fn call_other_contract_execute_on_dest(&self, other_sc_address: ManagedAddress) -> BigUint {
        let call_result = self.raw_vm_api().execute_on_dest_context_raw(
            self.blockchain().get_gas_left(),
            &other_sc_address,
            &BigUint::zero(),
            &ManagedBuffer::new_from_bytes(b"getTotalValue"),
            &ManagedArgBuffer::new_empty(),
        );
        let raw_value = call_result.get(0).unwrap_or_default();

        BigUint::from(raw_value.parse_as_u64().unwrap_or_default())
    }

    #[endpoint]
    fn call_other_contract_add_async_call(&self, other_sc_address: ManagedAddress, value: BigUint) {
        let mut args = ManagedArgBuffer::new_empty();
        args.push_arg(&value);

        self.raw_vm_api().async_call_raw(
            &other_sc_address,
            &BigUint::zero(),
            &ManagedBuffer::new_from_bytes(b"add"),
            &args,
        );
    }

    #[callback_raw]
    fn callback_raw(&self) {
        self.callback_executed().set(&true);
    }

    #[endpoint(getTotalValue)]
    fn get_total_value(&self) -> BigUint {
        self.total_value().get()
    }

    #[endpoint]
    fn execute_on_dest_add_value(&self, other_sc_address: ManagedAddress, value: BigUint) {
        let mut args = ManagedArgBuffer::new_empty();
        args.push_arg(value);

        let _ = self.raw_vm_api().execute_on_dest_context_raw(
            self.blockchain().get_gas_left(),
            &other_sc_address,
            &BigUint::zero(),
            &ManagedBuffer::new_from_bytes(b"addValue"),
            &args,
        );
    }

    #[endpoint(addValue)]
    fn add(&self, value: BigUint) {
        let caller = self.blockchain().get_caller();

        self.total_value().update(|val| *val += &value);
        self.value_per_caller(&caller).update(|val| *val += value);
    }

    fn get_val(&self) -> BigUint {
        self.total_value().get()
    }

    #[storage_mapper("totalValue")]
    fn total_value(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("valuePerCaller")]
    fn value_per_caller(&self, caller: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("callbackExecuted")]
    fn callback_executed(&self) -> SingleValueMapper<bool>;
}
