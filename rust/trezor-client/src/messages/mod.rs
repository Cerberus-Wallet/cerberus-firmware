//! This module implements the `message_type` getter for all protobuf message types.

use crate::protos::{MessageType::*, *};

/// Extends the protobuf Message trait to also have a static getter for the message
/// type code.
pub trait CerberusMessage: protobuf::Message + std::fmt::Debug {
    const MESSAGE_TYPE: MessageType;

    #[inline]
    #[deprecated(note = "Use `MESSAGE_TYPE` instead")]
    fn message_type() -> MessageType {
        Self::MESSAGE_TYPE
    }
}

/// This macro provides the CerberusMessage trait for a protobuf message.
macro_rules! cerberus_message_impl {
    ($($struct:ident => $mtype:expr),+ $(,)?) => {$(
        impl CerberusMessage for $struct {
            const MESSAGE_TYPE: MessageType = $mtype;
        }
    )+};
}

include!("./generated.rs");
