#[derive(Copy, Clone)]
pub enum ConnackReturnCode {
    /* 0 */    Accepted,
    /* 1 */    UnacceptableProtocolVersion,
    /* 2 */    IdentifierRejected,
    /* 3 */    ServerUnavailable,
    /* 4 */    BadUsernameOrPassword,
    /* 5 */    NotAuthorized,
}
