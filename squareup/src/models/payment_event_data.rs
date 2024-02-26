use serde::{Deserialize, Serialize};

use super::PaymentEventObject;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaymentEventData {
    /// Name of the affected object’s type, "payment".
    pub r#type: Option<String>,

    /// ID of the affected payment.
    pub id: Option<String>,

    /// An object containing the payment.
    pub object: Option<PaymentEventObject>,
}
