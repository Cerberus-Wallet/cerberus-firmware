use crate::{
    error::{Error, Result},
    messages::CerberusMessage,
    protos, Cerberus,
};
use std::fmt;

// Some types with raw protos that we use in the public interface so they have to be exported.
pub use protos::{
    button_request::ButtonRequestType, pin_matrix_request::PinMatrixRequestType, Features,
};

#[cfg(feature = "bitcoin")]
pub use protos::InputScriptType;

/// The different options for the number of words in a seed phrase.
pub enum WordCount {
    W12 = 12,
    W18 = 18,
    W24 = 24,
}

/// The different types of user interactions the Cerberus device can request.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum InteractionType {
    Button,
    PinMatrix,
    Passphrase,
    PassphraseState,
}

//TODO(stevenroose) should this be FnOnce and put in an FnBox?
/// Function to be passed to the `Cerberus.call` method to process the Cerberus response message into a
/// general-purpose type.
pub type ResultHandler<'a, T, R> = dyn Fn(&'a mut Cerberus, R) -> Result<T>;

/// A button request message sent by the device.
pub struct ButtonRequest<'a, T, R: CerberusMessage> {
    pub message: protos::ButtonRequest,
    pub client: &'a mut Cerberus,
    pub result_handler: Box<ResultHandler<'a, T, R>>,
}

impl<'a, T, R: CerberusMessage> fmt::Debug for ButtonRequest<'a, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.message, f)
    }
}

impl<'a, T, R: CerberusMessage> ButtonRequest<'a, T, R> {
    /// The type of button request.
    pub fn request_type(&self) -> ButtonRequestType {
        self.message.code()
    }

    /// Ack the request and get the next message from the device.
    pub fn ack(self) -> Result<CerberusResponse<'a, T, R>> {
        let req = protos::ButtonAck::new();
        self.client.call(req, self.result_handler)
    }
}

/// A PIN matrix request message sent by the device.
pub struct PinMatrixRequest<'a, T, R: CerberusMessage> {
    pub message: protos::PinMatrixRequest,
    pub client: &'a mut Cerberus,
    pub result_handler: Box<ResultHandler<'a, T, R>>,
}

impl<'a, T, R: CerberusMessage> fmt::Debug for PinMatrixRequest<'a, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.message, f)
    }
}

impl<'a, T, R: CerberusMessage> PinMatrixRequest<'a, T, R> {
    /// The type of PIN matrix request.
    pub fn request_type(&self) -> PinMatrixRequestType {
        self.message.type_()
    }

    /// Ack the request with a PIN and get the next message from the device.
    pub fn ack_pin(self, pin: String) -> Result<CerberusResponse<'a, T, R>> {
        let mut req = protos::PinMatrixAck::new();
        req.set_pin(pin);
        self.client.call(req, self.result_handler)
    }
}

/// A passphrase request message sent by the device.
pub struct PassphraseRequest<'a, T, R: CerberusMessage> {
    pub message: protos::PassphraseRequest,
    pub client: &'a mut Cerberus,
    pub result_handler: Box<ResultHandler<'a, T, R>>,
}

impl<'a, T, R: CerberusMessage> fmt::Debug for PassphraseRequest<'a, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.message, f)
    }
}

impl<'a, T, R: CerberusMessage> PassphraseRequest<'a, T, R> {
    /// Check whether the use is supposed to enter the passphrase on the device or not.
    pub fn on_device(&self) -> bool {
        self.message._on_device()
    }

    /// Ack the request with a passphrase and get the next message from the device.
    pub fn ack_passphrase(self, passphrase: String) -> Result<CerberusResponse<'a, T, R>> {
        let mut req = protos::PassphraseAck::new();
        req.set_passphrase(passphrase);
        self.client.call(req, self.result_handler)
    }

    /// Ack the request without a passphrase to let the user enter it on the device
    /// and get the next message from the device.
    pub fn ack(self, on_device: bool) -> Result<CerberusResponse<'a, T, R>> {
        let mut req = protos::PassphraseAck::new();
        if on_device {
            req.set_on_device(on_device);
        }
        self.client.call(req, self.result_handler)
    }
}

/// A response from a Cerberus device.  On every message exchange, instead of the expected/desired
/// response, the Cerberus can ask for some user interaction, or can send a failure.
#[derive(Debug)]
pub enum CerberusResponse<'a, T, R: CerberusMessage> {
    Ok(T),
    Failure(protos::Failure),
    ButtonRequest(ButtonRequest<'a, T, R>),
    PinMatrixRequest(PinMatrixRequest<'a, T, R>),
    PassphraseRequest(PassphraseRequest<'a, T, R>),
    //TODO(stevenroose) This should be taken out of this enum and intrinsically attached to the
    // PassphraseRequest variant.  However, it's currently impossible to do this.  It might be
    // possible to do with FnBox (currently nightly) or when Box<FnOnce> becomes possible.
}

impl<'a, T, R: CerberusMessage> fmt::Display for CerberusResponse<'a, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO(stevenroose) should we make T: Debug?
            CerberusResponse::Ok(ref _m) => f.write_str("Ok"),
            CerberusResponse::Failure(ref m) => write!(f, "Failure: {:?}", m),
            CerberusResponse::ButtonRequest(ref r) => write!(f, "ButtonRequest: {:?}", r),
            CerberusResponse::PinMatrixRequest(ref r) => write!(f, "PinMatrixRequest: {:?}", r),
            CerberusResponse::PassphraseRequest(ref r) => write!(f, "PassphraseRequest: {:?}", r),
        }
    }
}

impl<'a, T, R: CerberusMessage> CerberusResponse<'a, T, R> {
    /// Get the actual `Ok` response value or an error if not `Ok`.
    pub fn ok(self) -> Result<T> {
        match self {
            CerberusResponse::Ok(m) => Ok(m),
            CerberusResponse::Failure(m) => Err(Error::FailureResponse(m)),
            CerberusResponse::ButtonRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::Button))
            }
            CerberusResponse::PinMatrixRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::PinMatrix))
            }
            CerberusResponse::PassphraseRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::Passphrase))
            }
        }
    }

    /// Get the button request object or an error if not `ButtonRequest`.
    pub fn button_request(self) -> Result<ButtonRequest<'a, T, R>> {
        match self {
            CerberusResponse::ButtonRequest(r) => Ok(r),
            CerberusResponse::Ok(_) => Err(Error::UnexpectedMessageType(R::MESSAGE_TYPE)),
            CerberusResponse::Failure(m) => Err(Error::FailureResponse(m)),
            CerberusResponse::PinMatrixRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::PinMatrix))
            }
            CerberusResponse::PassphraseRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::Passphrase))
            }
        }
    }

    /// Get the PIN matrix request object or an error if not `PinMatrixRequest`.
    pub fn pin_matrix_request(self) -> Result<PinMatrixRequest<'a, T, R>> {
        match self {
            CerberusResponse::PinMatrixRequest(r) => Ok(r),
            CerberusResponse::Ok(_) => Err(Error::UnexpectedMessageType(R::MESSAGE_TYPE)),
            CerberusResponse::Failure(m) => Err(Error::FailureResponse(m)),
            CerberusResponse::ButtonRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::Button))
            }
            CerberusResponse::PassphraseRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::Passphrase))
            }
        }
    }

    /// Get the passphrase request object or an error if not `PassphraseRequest`.
    pub fn passphrase_request(self) -> Result<PassphraseRequest<'a, T, R>> {
        match self {
            CerberusResponse::PassphraseRequest(r) => Ok(r),
            CerberusResponse::Ok(_) => Err(Error::UnexpectedMessageType(R::MESSAGE_TYPE)),
            CerberusResponse::Failure(m) => Err(Error::FailureResponse(m)),
            CerberusResponse::ButtonRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::Button))
            }
            CerberusResponse::PinMatrixRequest(_) => {
                Err(Error::UnexpectedInteractionRequest(InteractionType::PinMatrix))
            }
        }
    }
}

pub fn handle_interaction<T, R: CerberusMessage>(resp: CerberusResponse<'_, T, R>) -> Result<T> {
    match resp {
        CerberusResponse::Ok(res) => Ok(res),
        CerberusResponse::Failure(_) => resp.ok(), // assering ok() returns the failure error
        CerberusResponse::ButtonRequest(req) => handle_interaction(req.ack()?),
        CerberusResponse::PinMatrixRequest(_) => Err(Error::UnsupportedNetwork),
        CerberusResponse::PassphraseRequest(req) => handle_interaction({
            let on_device = req.on_device();
            req.ack(!on_device)?
        }),
    }
}

/// When resetting the device, it will ask for entropy to aid key generation.
pub struct EntropyRequest<'a> {
    pub client: &'a mut Cerberus,
}

impl<'a> EntropyRequest<'a> {
    /// Provide exactly 32 bytes or entropy.
    pub fn ack_entropy(self, entropy: Vec<u8>) -> Result<CerberusResponse<'a, (), protos::Success>> {
        if entropy.len() != 32 {
            return Err(Error::InvalidEntropy)
        }

        let mut req = protos::EntropyAck::new();
        req.set_entropy(entropy);
        self.client.call(req, Box::new(|_, _| Ok(())))
    }
}
