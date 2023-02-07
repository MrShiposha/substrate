// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the FRAME NFTs pallet.

use std::{fmt::Debug, sync::Arc};

use codec::{Decode, Encode};
use frame_support::traits::tokens::AttributeNamespace;
use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

pub use pallet_nfts_rpc_runtime_api::NftsApi as NftsRuntimeApi;

#[rpc(client, server)]
pub trait NftsApi<BlockHash, AccountId, CollectionId, ItemId> {
	#[method(name = "nfts_itemOwner")]
	fn item_owner(
		&self,
		collection: CollectionId,
		item: ItemId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<AccountId>>;

	#[method(name = "nfts_collectionOwner")]
	fn collection_owner(
		&self,
		collection: CollectionId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<AccountId>>;

	#[method(name = "nfts_itemAttribute")]
	fn item_attribute(
		&self,
		collection: CollectionId,
		item: ItemId,
		key: Vec<u8>,
		namespace: Option<AttributeNamespace<AccountId>>,
		at: Option<BlockHash>,
	) -> RpcResult<Option<Vec<u8>>>;

	#[method(name = "nfts_collectionAttribute")]
	fn collection_attribute(
		&self,
		collection: CollectionId,
		key: Vec<u8>,
		at: Option<BlockHash>,
	) -> RpcResult<Option<Vec<u8>>>;
}

pub struct Nfts<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Nfts<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

const RUNTIME_ERROR: i32 = 1;

fn str_rpc_error<S: Debug>(data: S, err: &str) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(RUNTIME_ERROR, err, Some(format!("{:?}", data)))).into()
}

impl<C, Block, AccountId, CollectionId, ItemId>
	NftsApiServer<<Block as BlockT>::Hash, AccountId, CollectionId, ItemId> for Nfts<C, Block>
where
	Block: BlockT,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: NftsRuntimeApi<Block, AccountId, CollectionId, ItemId>,
	AccountId: Encode + Decode,
	CollectionId: Encode,
	ItemId: Encode,
{
	fn item_owner(
		&self,
		collection: CollectionId,
		item: ItemId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<AccountId>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		api.item_owner(&at, collection, item)
			.map_err(|e| str_rpc_error(e, "Unable to get an item owner."))
	}

	fn collection_owner(
		&self,
		collection: CollectionId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<AccountId>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		api.collection_owner(&at, collection)
			.map_err(|e| str_rpc_error(e, "Unable to get a collection owner."))
	}

	fn item_attribute(
		&self,
		collection: CollectionId,
		item: ItemId,
		key: Vec<u8>,
		namespace: Option<AttributeNamespace<AccountId>>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<Vec<u8>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		api.item_attribute(&at, collection, item, key, namespace)
			.map_err(|e| str_rpc_error(e, "Unable to get an item attribute."))
	}

	fn collection_attribute(
		&self,
		collection: CollectionId,
		key: Vec<u8>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<Vec<u8>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		api.collection_attribute(&at, collection, key)
			.map_err(|e| str_rpc_error(e, "Unable to get a collection attribute."))
	}
}
