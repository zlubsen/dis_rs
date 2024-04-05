use crate::entity_state::model::{CdisEntityAppearance, CdisEntityCapabilities, EntityState};
use crate::BodyProperties;
use crate::constants::{HUNDRED_TWENTY_BITS, ONE_BIT, THIRTY_TWO_BITS};
use crate::types::model::UVINT8;
use crate::writing::{BitBuffer, serialize_when_present, SerializeCdis, SerializeCdisPdu, write_value_unsigned};

impl SerializeCdisPdu for EntityState {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();
        let cursor = write_value_unsigned(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.full_update_flag.into());
        let cursor = self.entity_id.serialize(buf, cursor);
        let cursor = serialize_when_present(&self.force_id, buf, cursor);
        let cursor = if !self.variable_parameters.is_empty() {
            UVINT8::from(self.variable_parameters.len() as u8 ).serialize(buf, cursor)
        } else { cursor };
        let cursor = serialize_when_present(&self.entity_type, buf, cursor);
        let cursor = serialize_when_present(&self.alternate_entity_type, buf, cursor);
        let cursor = serialize_when_present(&self.entity_linear_velocity, buf, cursor);
        let cursor = serialize_when_present(&self.entity_location, buf, cursor);
        let cursor = serialize_when_present(&self.entity_orientation, buf, cursor);
        let cursor = serialize_when_present(&self.entity_appearance, buf, cursor);

        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.dr_algorithm.into());
        let cursor = if let Some(other) = &self.dr_params_other {
            write_value_unsigned(buf, cursor, HUNDRED_TWENTY_BITS, other.0)
        } else { cursor };
        let cursor = serialize_when_present(&self.dr_params_entity_linear_acceleration, buf, cursor);
        let cursor = serialize_when_present(&self.dr_params_entity_angular_velocity, buf, cursor);

        let cursor = serialize_when_present(&self.entity_marking, buf, cursor);
        let cursor = serialize_when_present(&self.capabilities, buf, cursor);

        let cursor = self.variable_parameters.iter()
            .fold(cursor, |cursor, vp| vp.serialize(buf, cursor) );

        cursor
    }
}

impl SerializeCdis for CdisEntityAppearance {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, self.0)
    }
}

impl SerializeCdis for CdisEntityCapabilities {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        self.0.serialize(buf, cursor)
    }
}
