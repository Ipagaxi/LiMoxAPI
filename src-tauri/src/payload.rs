

#[derive(Clone, serde::Serialize)]
pub struct PayloadStringMatrix {
    pub value: Vec<Vec<String>>
}

#[derive(Clone, serde::Serialize)]
pub struct PayloadUsizeVec {
    pub value: Vec<usize>
}

#[derive(Clone, serde::Serialize)]
pub struct PayloadStringVec {
    pub value: Vec<String>
}

#[derive(Clone, serde::Serialize)]
pub struct PayloadUsize {
    pub value: usize
}

#[derive(Clone, serde::Serialize)]
pub struct PayloadBool {
    pub balue: bool
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct ConnectionParamsPayload {
    pub ip_address: String,
    pub port: String,
    pub username: String,
    pub password: String
}

#[derive(Clone, serde::Serialize, Debug)]
pub struct ConnectionResponsePayload {
    pub successfull: bool,
    pub response: String
}

#[derive(Clone, serde::Serialize)]
pub struct StatementLogPayload {
    pub densed_statements: Vec<String>,
    pub full_statements: Vec<String>
}