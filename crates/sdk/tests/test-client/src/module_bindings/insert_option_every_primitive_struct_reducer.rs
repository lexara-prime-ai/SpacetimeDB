// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::every_primitive_struct::EveryPrimitiveStruct;
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InsertOptionEveryPrimitiveStructArgs {
    pub s: Option<EveryPrimitiveStruct>,
}

impl Reducer for InsertOptionEveryPrimitiveStructArgs {
    const REDUCER_NAME: &'static str = "insert_option_every_primitive_struct";
}

#[allow(unused)]
pub fn insert_option_every_primitive_struct(s: Option<EveryPrimitiveStruct>) {
    InsertOptionEveryPrimitiveStructArgs { s }.invoke();
}

#[allow(unused)]
pub fn on_insert_option_every_primitive_struct(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Option<EveryPrimitiveStruct>) + Send + 'static,
) -> ReducerCallbackId<InsertOptionEveryPrimitiveStructArgs> {
    InsertOptionEveryPrimitiveStructArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOptionEveryPrimitiveStructArgs { s } = __args;
        __callback(__identity, __addr, __status, s);
    })
}

#[allow(unused)]
pub fn once_on_insert_option_every_primitive_struct(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Option<EveryPrimitiveStruct>) + Send + 'static,
) -> ReducerCallbackId<InsertOptionEveryPrimitiveStructArgs> {
    InsertOptionEveryPrimitiveStructArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOptionEveryPrimitiveStructArgs { s } = __args;
        __callback(__identity, __addr, __status, s);
    })
}

#[allow(unused)]
pub fn remove_on_insert_option_every_primitive_struct(id: ReducerCallbackId<InsertOptionEveryPrimitiveStructArgs>) {
    InsertOptionEveryPrimitiveStructArgs::remove_on_reducer(id);
}