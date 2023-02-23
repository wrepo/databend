// Copyright 2023 Datafuse Labs.
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
//

use std::any::Any;
use std::collections::HashSet;

use common_exception::ErrorCode;
use common_expression::BlockMetaInfo;
use common_expression::DataBlock;
use common_expression::Scalar;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum MergeIntoOperation {
    Delete(DeletionByColumn),
    None,
}

pub type UniqueKeyDigest = u128;
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct DeletionByColumn {
    // used in table meta level pruning
    pub key_min: Scalar,
    pub key_max: Scalar,
    // used in block level
    pub key_hashes: HashSet<UniqueKeyDigest>,
}

#[typetag::serde(name = "merge_into_operation_meta")]
impl BlockMetaInfo for MergeIntoOperation {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn equals(&self, info: &Box<dyn BlockMetaInfo>) -> bool {
        match info.as_any().downcast_ref::<MergeIntoOperation>() {
            None => false,
            Some(other) => self == other,
        }
    }

    fn clone_self(&self) -> Box<dyn BlockMetaInfo> {
        Box::new(self.clone())
    }
}

impl<'a> TryFrom<&'a DataBlock> for &'a MergeIntoOperation {
    type Error = ErrorCode;

    fn try_from(value: &'a DataBlock) -> Result<Self, Self::Error> {
        let meta = value.get_meta().ok_or_else(|| {
            ErrorCode::Internal(
                "convert MergeIntoOperation from data block failed, no block meta found",
            )
        })?;
        match meta.as_any().downcast_ref::<MergeIntoOperation>() {
            Some(v) => Ok(v),
            None => Err(ErrorCode::Internal(
                "Cannot downcast from BlockMetaInfo to MergeIntoOperation.",
            )),
        }
    }
}
