use bakkie_schema::{InitializeRequestParams, InitializeResult};

#[derive(Debug)]
pub struct NegotiatedAgreement {
    pub init_req: InitializeRequestParams,
    pub init_resp: InitializeResult,
}

impl NegotiatedAgreement {
    pub fn new(init_req: InitializeRequestParams, init_resp: InitializeResult) -> Self {
        Self {
            init_req,
            init_resp,
        }
    }
}
