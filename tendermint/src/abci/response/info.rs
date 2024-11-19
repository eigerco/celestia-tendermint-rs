use crate::abci::types::TimeoutsInfo;
use crate::{block, prelude::*, AppHash};
use celestia_tendermint_proto::v0_34::abci as pb;

use serde::{Deserialize, Serialize};

#[doc = include_str!("../doc/response-info.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(default, try_from = "pb::ResponseInfo", into = "pb::ResponseInfo")]
pub struct Info {
    /// Some arbitrary information.
    pub data: String,
    /// The application software semantic version.
    pub version: String,
    /// The application protocol version.
    pub app_version: u64,
    /// The latest block for which the app has called [`Commit`](super::super::Request::Commit).
    pub last_block_height: block::Height,
    /// The latest result of [`Commit`](super::super::Request::Commit).
    pub last_block_app_hash: AppHash,
    /// Timeout information.
    pub timeouts: Option<TimeoutsInfo>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_34 {
    use super::Info;
    use celestia_tendermint_proto::v0_34 as pb;
    use celestia_tendermint_proto::Protobuf;

    impl From<Info> for pb::abci::ResponseInfo {
        fn from(info: Info) -> Self {
            Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.into(),
                last_block_app_hash: info.last_block_app_hash.into(),
                timeouts: info.timeouts.map(|t| t.into()),
            }
        }
    }

    impl TryFrom<pb::abci::ResponseInfo> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::abci::ResponseInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.try_into()?,
                last_block_app_hash: info.last_block_app_hash.try_into()?,
                timeouts: info.timeouts.map(|t| t.try_into()).transpose()?,
            })
        }
    }

    impl Protobuf<pb::abci::ResponseInfo> for Info {}
}

mod v0_37 {
    use super::Info;
    use celestia_tendermint_proto::v0_37 as pb;
    use celestia_tendermint_proto::Protobuf;

    impl From<Info> for pb::abci::ResponseInfo {
        fn from(info: Info) -> Self {
            Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.into(),
                last_block_app_hash: info.last_block_app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::abci::ResponseInfo> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::abci::ResponseInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.try_into()?,
                last_block_app_hash: info.last_block_app_hash.try_into()?,
                timeouts: None,
            })
        }
    }

    impl Protobuf<pb::abci::ResponseInfo> for Info {}
}
