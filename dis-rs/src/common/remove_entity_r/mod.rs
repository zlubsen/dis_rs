pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{PduType, RequiredReliabilityService};
    use crate::model::EntityId;
    use crate::remove_entity_r::model::RemoveEntityR;
    use bytes::BytesMut;

    #[test]
    fn remove_entity_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::RemoveEntityR);

        let body = RemoveEntityR::builder()
            .with_origination_id(EntityId::new(10, 20, 30))
            .with_receiving_id(EntityId::new(40, 50, 60))
            .with_required_reliability_service(RequiredReliabilityService::Acknowledged)
            .with_request_id(55)
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf).unwrap();

        let parsed = parse_pdu(&buf);
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, pdu);
            }
            Err(ref _err) => {
                println!("{_err}");
                assert!(false);
            }
        }
    }
}
