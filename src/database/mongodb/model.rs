use async_trait::async_trait;
use validator::Validate;
use wither::bson::{
	Bson, Document, self, oid::ObjectId
};
use wither::mongodb::{
	options::{
		FindOneAndDeleteOptions,FindOneOptions,FindOptions,ReturnDocument, UpdateOptions
	},
	results::{
		DeleteResult, UpdateResult
	},
};
use wither::Model as WitherModel;
use wither::ModelCursor;
use crate::common::error_handler::Error;
use crate::database::mongodb::client;

#[async_trait]
pub trait ModelExt
where
Self: WitherModel + Validate,
{
}