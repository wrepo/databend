// Copyright 2021 Datafuse Labs
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

use std::sync::Arc;

use common_base::base::GlobalInstance;
use common_catalog::catalog::Catalog;
use common_exception::Result;
use common_meta_app::schema::CreateIndexReply;
use common_meta_app::schema::CreateIndexReq;
use common_meta_app::schema::DropIndexReply;
use common_meta_app::schema::DropIndexReq;
use common_meta_app::schema::GetIndexReply;
use common_meta_app::schema::GetIndexReq;

#[async_trait::async_trait]
pub trait AggregatingIndexHandler: Sync + Send {
    async fn do_create_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: CreateIndexReq,
    ) -> Result<CreateIndexReply>;

    async fn do_drop_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: DropIndexReq,
    ) -> Result<DropIndexReply>;

    async fn do_get_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: GetIndexReq,
    ) -> Result<GetIndexReply>;
}

pub struct AggregatingIndexHandlerWrapper {
    handler: Box<dyn AggregatingIndexHandler>,
}

impl AggregatingIndexHandlerWrapper {
    pub fn new(handler: Box<dyn AggregatingIndexHandler>) -> Self {
        Self { handler }
    }

    #[async_backtrace::framed]
    pub async fn do_create_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: CreateIndexReq,
    ) -> Result<CreateIndexReply> {
        self.handler.do_create_index(catalog, req).await
    }

    #[async_backtrace::framed]
    pub async fn do_drop_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: DropIndexReq,
    ) -> Result<DropIndexReply> {
        self.handler.do_drop_index(catalog, req).await
    }

    #[async_backtrace::framed]
    pub async fn do_get_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: GetIndexReq,
    ) -> Result<GetIndexReply> {
        self.handler.do_get_index(catalog, req).await
    }
}

pub fn get_agg_index_handler() -> Arc<AggregatingIndexHandlerWrapper> {
    GlobalInstance::get()
}
