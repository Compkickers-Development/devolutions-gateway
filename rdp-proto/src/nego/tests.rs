use super::*;

#[test]
fn cookie_is_written_to_request() {
    let mut buff = Vec::new();
    let cookie = "a";
    let expected = [
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x61,
        0x0D, 0x0A,
    ];
    let message_len = 19 + cookie.len();

    write_negotiation_request(
        &mut buff,
        cookie,
        SecurityProtocol::HYBRID | SecurityProtocol::SSL,
        NegotiationRequestFlags::default(),
    )
    .unwrap();

    assert_eq!(buff[..message_len], expected);
}

#[test]
fn rdp_negotiation_data_is_written_to_request_if_nla_security() {
    let mut buff = Vec::new();
    let cookie = "a";
    let expected = [0x01, 0x00, 0x08, 0x00, 0x03, 0x00, 0x00, 0x00];

    write_negotiation_request(
        &mut buff,
        cookie,
        SecurityProtocol::HYBRID | SecurityProtocol::SSL,
        NegotiationRequestFlags::default(),
    )
    .unwrap();

    assert_eq!(buff[buff.len() - 8..buff.len()], expected);
}

#[test]
fn rdp_negotiation_data_is_not_written_if_rdp_security() {
    let mut buff = Vec::new();
    let cookie = "a";
    let message_len = 19 + cookie.len();

    write_negotiation_request(
        &mut buff,
        cookie,
        SecurityProtocol::RDP,
        NegotiationRequestFlags::default(),
    )
    .unwrap();

    assert_eq!(buff.len(), message_len);
}

#[test]
fn negotiation_request_is_written_correclty() {
    let expected: &[u8] = &[
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x55,
        0x73, 0x65, 0x72, 0x0D, 0x0A, 0x01, 0x00, 0x08, 0x00, 0x03, 0x00, 0x00, 0x00,
    ];
    let mut buff = Vec::new();

    write_negotiation_request(
        &mut buff,
        "User",
        SecurityProtocol::HYBRID | SecurityProtocol::SSL,
        NegotiationRequestFlags::default(),
    )
    .unwrap();

    assert_eq!(buff, expected);
}

#[test]
fn negotiation_response_is_processed_correctly() {
    let expected_flags = NegotiationResponseFlags::all();
    #[rustfmt::skip]
    let stream = [
        0x02, // negotiation message
        expected_flags.bits(),
        0x08, 0x00, // length
        0x02, 0x00, 0x00, 0x00, // selected protocol
    ];

    let (selected_protocol, flags) =
        parse_negotiation_response(X224TPDUType::ConnectionConfirm, &mut stream.as_ref()).unwrap();

    assert_eq!(selected_protocol, SecurityProtocol::HYBRID);
    assert_eq!(flags, expected_flags);
}

#[test]
fn wrong_x224_code_in_negotiation_response_results_in_error() {
    let stream = [
        0x02, // negotiation message
        0x1F, // flags
        0x08, 0x00, // length
        0x02, 0x00, 0x00, 0x00, // selected protocol
    ];

    match parse_negotiation_response(X224TPDUType::ConnectionRequest, &mut stream.as_ref()) {
        Err(NegotiationError::IOError(ref e)) if e.kind() == io::ErrorKind::InvalidData => (),
        Err(_e) => panic!("wrong error type"),
        _ => panic!("error expected"),
    }
}

#[test]
fn wrong_message_code_in_negotiation_response_results_in_error() {
    let stream = [
        0xAF, // negotiation message
        0x1F, // flags
        0x08, 0x00, // length
        0x02, 0x00, 0x00, 0x00, // selected protocol
    ];

    match parse_negotiation_response(X224TPDUType::ConnectionConfirm, &mut stream.as_ref()) {
        Err(NegotiationError::IOError(ref e)) if e.kind() == io::ErrorKind::InvalidData => (),
        Err(_e) => panic!("wrong error type"),
        _ => panic!("error expected"),
    }
}

#[test]
fn negotiation_failure_in_repsonse_results_in_error() {
    let stream = [
        0x03, // negotiation message
        0x1F, // flags
        0x08, 0x00, // length
        0x06, 0x00, 0x00, 0x00, // failure code
    ];

    match parse_negotiation_response(X224TPDUType::ConnectionConfirm, &mut stream.as_ref()) {
        Err(NegotiationError::NegotiationFailure(e))
            if e == NegotiationFailureCodes::SSLWithUserAuthRequiredByServer => {}
        Err(_e) => panic!("wrong error type"),
        _ => panic!("error expected"),
    }
}

#[test]
fn cookie_in_request_is_parsed_correctly() {
    let request = [
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x55,
        0x73, 0x65, 0x72, 0x0D, 0x0A, 0xFF, 0xFF,
    ];

    let cookie = parse_request_cookie(&mut request.as_ref()).unwrap();

    assert_eq!(cookie, "User");
}

#[test]
fn parse_request_cookie_on_non_cookie_results_in_error() {
    let request = [
        0x6e, 0x6f, 0x74, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x0F, 0x42, 0x73, 0x65, 0x72, 0x0D,
        0x0A, 0xFF, 0xFF,
    ];

    match parse_request_cookie(&mut request.as_ref()) {
        Err(ref e) if e.kind() == io::ErrorKind::InvalidData => (),
        Err(_e) => panic!("wrong error type"),
        _ => panic!("error expected"),
    }
}

#[test]
fn parse_request_cookie_on_unterminated_cookie_results_in_error() {
    let request = [
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x55,
        0x73, 0x65, 0x72,
    ];

    match parse_request_cookie(&mut request.as_ref()) {
        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => (),
        Err(_e) => panic!("wrong error type"),
        _ => panic!("error expected"),
    }
}

#[test]
fn negotiation_request_with_negotiation_data_is_parsed_correctly() {
    let expected_flags = NegotiationRequestFlags::RESTRICTED_ADMIN_MODE_REQUIED
        | NegotiationRequestFlags::REDIRECTED_AUTHENTICATION_MODE_REQUIED;
    #[rustfmt::skip]
    let request = [
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x55,
        0x73, 0x65, 0x72, 0x0D, 0x0A, // cookie
        0x01, // request code
        expected_flags.bits(),
        0x08, 0x00, 0x03, 0x00, 0x00, 0x00, // request message
    ];

    let (cookie, protocol, flags) =
        parse_negotiation_request(X224TPDUType::ConnectionRequest, request.as_ref()).unwrap();

    assert_eq!(cookie, "User");
    assert_eq!(flags, expected_flags);
    assert_eq!(protocol, SecurityProtocol::HYBRID | SecurityProtocol::SSL);
}

#[test]
fn negotiation_request_without_negotiation_data_is_parsed_correctly() {
    let request = [
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x55,
        0x73, 0x65, 0x72, 0x0D, 0x0A, // cookie
    ];

    let (cookie, protocol, flags) =
        parse_negotiation_request(X224TPDUType::ConnectionRequest, request.as_ref()).unwrap();

    assert_eq!(cookie, "User");
    assert_eq!(flags, NegotiationRequestFlags::default());
    assert_eq!(protocol, SecurityProtocol::RDP);
}

#[test]
fn negotiation_request_with_invalid_negotiation_code_results_in_error() {
    let request = [
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x55,
        0x73, 0x65, 0x72, 0x0D, 0x0A, // cookie
        0x03, // request code
        0x00, 0x08, 0x00, 0x03, 0x00, 0x00, 0x00, // request message
    ];

    match parse_negotiation_request(X224TPDUType::ConnectionRequest, request.as_ref()) {
        Err(ref e) if e.kind() == io::ErrorKind::InvalidData => (),
        Err(_e) => panic!("wrong error type"),
        _ => panic!("error expected"),
    }
}

#[test]
fn negotiation_request_with_invalid_x224_code_results_in_error() {
    let request = [
        0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x3A, 0x20, 0x6D, 0x73, 0x74, 0x73, 0x68, 0x61, 0x73, 0x68, 0x3D, 0x55,
        0x73, 0x65, 0x72, 0x0D, 0x0A, // cookie
        0x01, // request code
        0x00, 0x08, 0x00, 0x03, 0x00, 0x00, 0x00, // request message
    ];

    match parse_negotiation_request(X224TPDUType::ConnectionConfirm, request.as_ref()) {
        Err(ref e) if e.kind() == io::ErrorKind::InvalidData => (),
        Err(_e) => panic!("wrong error type"),
        _ => panic!("error expected"),
    }
}

#[test]
fn negotiation_response_is_written_correctly() {
    let flags = NegotiationResponseFlags::all();
    #[rustfmt::skip]
    let expected = [
        0x02, // negotiation message
        flags.bits(),
        0x08, 0x00, // length
        0x02, 0x00, 0x00, 0x00, // selected protocol
    ];

    let mut buffer = vec![0; expected.len()];

    write_negotiation_response(&mut buffer.as_mut_slice(), flags, SecurityProtocol::HYBRID).unwrap();

    assert_eq!(buffer, expected);
}

#[test]
fn negotiation_error_is_written_correclty() {
    #[rustfmt::skip]
    let expected = [
        0x03, // negotiation message
        0x00,
        0x08, 0x00, // length
        0x02, 0x00, 0x00, 0x00, // error code
    ];

    let mut buffer = vec![0; expected.len()];

    write_negotiation_response_error(
        &mut buffer.as_mut_slice(),
        NegotiationFailureCodes::SSLNotAllowedByServer,
    )
    .unwrap();

    assert_eq!(buffer, expected);
}
