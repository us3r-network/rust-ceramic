//! Implements the /dag/* endpoints.
use std::io::{Cursor, Read, Seek};

use anyhow::anyhow;
use iroh_api::{Cid, IpfsPath};
use libipld::{
    cbor::DagCborCodec,
    multihash::{Code, MultihashDigest},
    pb::DagPbCodec,
    prelude::{Codec, Decode, Encode},
    Ipld,
};

use crate::{error::Error, IpfsDep};

#[tracing::instrument(skip(client, output_codec))]
pub async fn get<T, C>(client: T, cid: Cid, output_codec: C) -> Result<Vec<u8>, Error>
where
    T: IpfsDep,
    C: Codec,
    Ipld: Encode<C>,
{
    let bytes = client.get(cid).await?.ok_or(Error::NotFound)?;
    let dag_data =
        match cid.codec() {
            // dag-pb
            0x70 => Ipld::decode(DagPbCodec, &mut Cursor::new(&bytes))
                .map_err(|e| Error::Internal(e))?,
            // dag-cbor
            0x71 => Ipld::decode(DagCborCodec, &mut Cursor::new(&bytes))
                .map_err(|e| Error::Internal(e))?,
            _ => {
                return Err(Error::Invalid(anyhow!("unsupported codec {}", cid.codec())));
            }
        };
    let mut data: Vec<u8> = Vec::new();
    dag_data
        .encode(output_codec, &mut data)
        .map_err(|e| Error::Internal(e.into()))?;
    Ok(data)
}

#[tracing::instrument(skip_all)]
pub async fn put<T, I, S, R>(
    client: T,
    input_codec: I,
    store_codec: S,
    data: &mut R,
) -> Result<Cid, Error>
where
    T: IpfsDep,
    I: Codec,
    S: Codec,
    Ipld: Decode<I>,
    Ipld: Encode<S>,
    R: Read + Seek,
{
    let dag_data = Ipld::decode(input_codec, data).map_err(|e| Error::Invalid(e.into()))?;

    let mut blob: Vec<u8> = Vec::new();
    dag_data
        .encode(store_codec, &mut blob)
        .map_err(|e| Error::Internal(e.into()))?;

    let hash = Code::Sha2_256.digest(&blob);
    let cid = Cid::new_v1(store_codec.into(), hash);
    println!("cid {}", cid);
    client.put(cid.clone(), blob.into(), vec![]).await?;
    Ok(cid)
}

#[tracing::instrument(skip(client))]
pub async fn resolve<T>(client: T, path: &IpfsPath) -> Result<Cid, Error>
where
    T: IpfsDep,
{
    let resolved_path = client.resolve(&path).await?;

    Ok(resolved_path
        .iter()
        .last()
        .ok_or(Error::Internal(anyhow!(
            "resolved path should have at least one element"
        )))?
        .to_owned())
}
