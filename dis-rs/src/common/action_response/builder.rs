use crate::common::action_response::model::ActionResponse;
use crate::common::model::{EntityId, FixedDatum, VariableDatum};
use crate::enumerations::RequestStatus;

pub struct ActionResponseBuilder(ActionResponse);

impl Default for ActionResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionResponseBuilder {
    #[must_use]
    pub fn new() -> Self {
        ActionResponseBuilder(ActionResponse::default())
    }

    #[must_use]
    pub fn new_from_body(body: ActionResponse) -> Self {
        ActionResponseBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> ActionResponse {
        self.0
    }

    #[must_use]
    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.0.originating_id = originating_id;
        self
    }

    #[must_use]
    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }

    #[must_use]
    pub fn with_request_status(mut self, request_status: RequestStatus) -> Self {
        self.0.request_status = request_status;
        self
    }

    #[must_use]
    pub fn with_fixed_datums(mut self, fixed_datum_records: Vec<FixedDatum>) -> Self {
        self.0.fixed_datum_records = fixed_datum_records;
        self
    }

    #[must_use]
    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableDatum>) -> Self {
        self.0.variable_datum_records = variable_datum_records;
        self
    }
}
