use std::convert::Infallible;

use aws_sdk_s3::{
    error::SdkError,
    operation::{
        complete_multipart_upload::CompleteMultipartUploadError,
        create_multipart_upload::CreateMultipartUploadError, head_object::HeadObjectError,
        put_object::PutObjectError, upload_part::UploadPartError,
    },
    primitives::SdkBody,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("S3 infallble")]
    S3Infallible(#[from] SdkError<Infallible, http::response::Response<SdkBody>>),
    #[error("S3 head object error")]
    S3Head(#[from] SdkError<HeadObjectError, http::response::Response<SdkBody>>),
    #[error("S3 uploadpart object error")]
    S3UploadPart(#[from] SdkError<UploadPartError, http::response::Response<SdkBody>>),
    #[error("S3 create multipart error")]
    S3CreateMultipart(
        #[from] SdkError<CreateMultipartUploadError, http::response::Response<SdkBody>>,
    ),
    #[error("S3 complete multipart error")]
    S3CompleteMultipart(
        #[from] SdkError<CompleteMultipartUploadError, http::response::Response<SdkBody>>,
    ),
    #[error("S3 put object error")]
    S3PutObject(#[from] SdkError<PutObjectError, http::response::Response<SdkBody>>),
    #[error("S3 conversion error")]
    S3Conversion(#[from] aws_smithy_types::date_time::ConversionError),
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("unknown object store error")]
    Unknown,
}

impl From<Error> for object_store::Error {
    fn from(value: Error) -> Self {
        object_store::Error::Generic {
            store: "S3",
            source: Box::new(value),
        }
    }
}
