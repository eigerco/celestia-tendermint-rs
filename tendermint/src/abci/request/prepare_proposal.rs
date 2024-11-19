use crate::block::Data;
use crate::block::Height;
use crate::chain::id::Id;
use crate::prelude::*;
use crate::time::Time;

#[doc = include_str!("../doc/request-prepareproposal.md")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrepareProposal {
    pub block_data: Data,
    pub block_data_size: i64,
    pub chain_id: Option<Id>,
    pub height: Option<Height>,
    pub time: Option<Time>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_34 {
    use super::PrepareProposal;
    use crate::{prelude::*, Error};
    use celestia_tendermint_proto::v0_34::abci as pb;
    use celestia_tendermint_proto::Protobuf;

    impl From<PrepareProposal> for pb::RequestPrepareProposal {
        fn from(value: PrepareProposal) -> Self {
            Self {
                block_data: Some(value.block_data.into()),
                block_data_size: value.block_data_size,
                chain_id: value.chain_id.map(|id| id.into()).unwrap_or_default(),
                height: value.height.map(|id| id.into()).unwrap_or_default(),
                time: value.time.map(|tm| tm.into()),
            }
        }
    }

    impl TryFrom<pb::RequestPrepareProposal> for PrepareProposal {
        type Error = Error;

        fn try_from(message: pb::RequestPrepareProposal) -> Result<Self, Self::Error> {
            Ok(PrepareProposal {
                block_data: message
                    .block_data
                    .ok_or_else(Error::missing_data)?
                    .try_into()?,
                block_data_size: message.block_data_size,
                chain_id: if message.chain_id.is_empty() {
                    None
                } else {
                    Some(message.chain_id.try_into()?)
                },
                height: if message.height == 0 {
                    None
                } else {
                    Some(message.height.try_into()?)
                },
                time: message.time.map(|tm| tm.try_into()).transpose()?,
            })
        }
    }

    impl Protobuf<pb::RequestPrepareProposal> for PrepareProposal {}
}
