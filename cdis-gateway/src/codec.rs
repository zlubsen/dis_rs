use std::collections::HashMap;
use bytes::{Bytes, BytesMut};
use cdis_assemble::{BitBuffer, CdisError, CdisPdu, Codec, SerializeCdisPdu};
use cdis_assemble::constants::MTU_BYTES;
use dis_rs::model::Pdu;
use dis_rs::{DisError, parse};
use crate::config::GatewayMode;

pub struct Encoder {
    mode: GatewayMode,
    cdis_buffer: BitBuffer,
    // hold a buffer/map of received PDUs to look up which fields can be left out
    lookup: HashMap<(u16,u16,u16), Pdu>
}

impl Encoder {
    pub fn new(mode: GatewayMode) -> Self {
        Self {
            mode,
            cdis_buffer: cdis_assemble::create_bit_buffer(),
            lookup: HashMap::new(),
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<Pdu>, DisError> {
        parse(&bytes)
    }

    fn writing(&mut self, cdis_pdus: Vec<CdisPdu>) -> Vec<u8> {
        let (total_bits, _cursor) = cdis_pdus.iter()
            .fold((0usize, 0usize),
                  | (total_bits, cursor), pdu| {
                      ( total_bits + pdu.pdu_length(), pdu.serialize(&mut self.cdis_buffer, cursor) )
                  });

        let cdis_wire: Vec<u8> = Vec::from(&self.cdis_buffer.data[0..total_bits.div_ceil(8)]);
        cdis_wire
    }

    // TODO make fallible, result from parse (and encode) function(s)
    pub fn encode_buffer(&mut self, bytes_in: Bytes) -> Vec<u8> {
        let pdus = self.parsing(bytes_in);
        let cdis_pdus = match pdus {
            Ok(pdus) => {
                self.encode_pdus(&pdus)
            }
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            }
        };
        self.writing(cdis_pdus)
    }

    // TODO make fallible, result from encode function
    pub fn encode_pdus(&self, pdus: &[Pdu]) -> Vec<CdisPdu> {
        let cdis_pdus: Vec<CdisPdu> = pdus.iter()
            .map(CdisPdu::encode )
            .collect();
        cdis_pdus
    }
}

pub struct Decoder {
    mode: GatewayMode,
    dis_buffer: BytesMut,
}

impl Decoder {
    pub fn new(mode: GatewayMode) -> Self {
        let dis_buffer = BytesMut::with_capacity(MTU_BYTES);

        Self {
            mode,
            dis_buffer,
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<CdisPdu>, CdisError> {
        cdis_assemble::parse(&bytes)
    }

    fn writing(&mut self, dis_pdus: Vec<Pdu>) -> Vec<u8> {
        // FIXME now we reset the buffer by assigning a new BytesMut; should not cause reallocation of under lying memory, but need to check.
        self.dis_buffer = BytesMut::with_capacity(MTU_BYTES);

        // FIXME number of bytes reported 'serialize' is too large for observed cases (208 actual, reported 252)...
        // number_of_bytes not used in creating the slice to put in the vec.
        let _number_of_bytes: usize = dis_pdus.iter()
            .map(| pdu| {
                pdu.serialize(&mut self.dis_buffer).unwrap() as usize
            } ).sum();

        // TODO perhaps replace Vec with Bytes, but unsure how to assign the latter Bytes::from_iter(&self.dis_buffer[..].iter()), or Bytes::from(&self.dis_buffer[..])?
        Vec::from(&self.dis_buffer[..])
    }

    // TODO make fallible, result from parse (and encode) function(s)
    pub fn decode_buffer(&mut self, bytes_in: Bytes) -> Vec<u8> {
        let cdis_pdus = self.parsing(bytes_in);
        let pdus = match cdis_pdus {
            Ok(pdus) => {
                self.decode_pdus(&pdus)
            }
            Err(err) => {
                println!("{}", err); // TODO tracing or Result return value
                Vec::new()
            }
        };

        self.writing(pdus)
    }

    // TODO make fallible, result from encode function
    pub fn decode_pdus(&self, pdus: &[CdisPdu]) -> Vec<Pdu> {
        let dis_pdus: Vec<Pdu> = pdus.iter()
            .map(|cdis_pdu| cdis_pdu.decode() )
            .collect();
        dis_pdus
    }
}