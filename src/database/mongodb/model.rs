use async_trait::async_trait;
use futures::stream::TryStreamExt;
use wither::bson::Bson;

use crate::database;