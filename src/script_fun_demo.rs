// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

// use aptos_types::account_address::AccountAddress;
// use cached_packages::aptos_stdlib::EntryFunctionCall;
// use cached_packages::aptos_stdlib::aptos_coin_transfer;

// pub(crate) fn demo_p2p_entry_function() {
//     let payee = AccountAddress::new([
//         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
//         0x22, 0x22,
//     ]);
//     let amount = 1234567;

//     // Now encode and decode a peer to peer transaction entry function.
//     let payload = aptos_coin_transfer(payee.clone(), amount);
//     let function_call = EntryFunctionCall::decode(&payload);
//     // match function_call {
//     //     Some(EntryFunctionCall::AptosCoinMint { amount: a, to: p }) => {
//     //         assert_eq!(a, amount);
//     //         assert_eq!(p, payee.clone());
//     //     }
//     //     _ => panic!("unexpected type of entry function"),
//     // };

//     let output = bcs::to_bytes(&payload).unwrap();
//     for o in output {
//         print!("{} ", o);
//     }
//     println!();
// }

// fn main() {
//     demo_p2p_entry_function();
// }
