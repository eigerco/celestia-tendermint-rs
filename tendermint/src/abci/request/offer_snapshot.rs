use super::super::types::Snapshot;
// bring into scope for doc links
#[allow(unused)]
use super::ApplySnapshotChunk;
use crate::{prelude::*, AppHash};

#[doc = include_str!("../doc/request-offersnapshot.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OfferSnapshot {
    /// The snapshot offered for restoration.
    pub snapshot: Snapshot,
    /// The light client verified app hash for this height.
    pub app_hash: AppHash,
    /// The app version for this height.
    pub app_version: Option<u64>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_34 {
    use super::OfferSnapshot;
    use celestia_tendermint_proto::v0_34 as pb;
    use celestia_tendermint_proto::Protobuf;

    impl From<OfferSnapshot> for pb::abci::RequestOfferSnapshot {
        fn from(offer_snapshot: OfferSnapshot) -> Self {
            Self {
                snapshot: Some(offer_snapshot.snapshot.into()),
                app_hash: offer_snapshot.app_hash.into(),
                app_version: offer_snapshot.app_version.unwrap_or_default(),
            }
        }
    }

    impl TryFrom<pb::abci::RequestOfferSnapshot> for OfferSnapshot {
        type Error = crate::Error;

        fn try_from(offer_snapshot: pb::abci::RequestOfferSnapshot) -> Result<Self, Self::Error> {
            Ok(Self {
                snapshot: offer_snapshot
                    .snapshot
                    .ok_or_else(crate::Error::missing_data)?
                    .try_into()?,
                app_hash: offer_snapshot.app_hash.try_into()?,
                app_version: if offer_snapshot.app_version == 0 {
                    None
                } else {
                    Some(offer_snapshot.app_version)
                },
            })
        }
    }

    impl Protobuf<pb::abci::RequestOfferSnapshot> for OfferSnapshot {}
}

mod v0_37 {
    use super::OfferSnapshot;
    use celestia_tendermint_proto::v0_37 as pb;
    use celestia_tendermint_proto::Protobuf;

    impl From<OfferSnapshot> for pb::abci::RequestOfferSnapshot {
        fn from(offer_snapshot: OfferSnapshot) -> Self {
            Self {
                snapshot: Some(offer_snapshot.snapshot.into()),
                app_hash: offer_snapshot.app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::abci::RequestOfferSnapshot> for OfferSnapshot {
        type Error = crate::Error;

        fn try_from(offer_snapshot: pb::abci::RequestOfferSnapshot) -> Result<Self, Self::Error> {
            Ok(Self {
                snapshot: offer_snapshot
                    .snapshot
                    .ok_or_else(crate::Error::missing_data)?
                    .try_into()?,
                app_hash: offer_snapshot.app_hash.try_into()?,
                app_version: None,
            })
        }
    }

    impl Protobuf<pb::abci::RequestOfferSnapshot> for OfferSnapshot {}
}
