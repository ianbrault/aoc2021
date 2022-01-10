/*
** src/puzzles/day_16.rs
** https://adventofcode.com/2021/day/16
*/

use crate::types::{Puzzle, Result, Solution};

use num::{Integer, NumCast};

#[derive(Debug, PartialEq)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    Greater,
    Less,
    Equal,
}

impl From<u8> for PacketType {
    fn from(x: u8) -> Self {
        match x {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => Self::Literal,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum PacketData {
    Literal(u64),
    Subpackets(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    type_id: PacketType,
    length_type_id: u8,
    data: PacketData,
}

impl Packet {
    fn literal(&self) -> u64 {
        match self.data {
            PacketData::Literal(n) => n,
            _ => unreachable!(),
        }
    }

    fn subpackets(&self) -> &Vec<Packet> {
        match &self.data {
            PacketData::Subpackets(subpackets) => subpackets,
            _ => unreachable!(),
        }
    }

    fn version_sum(&self) -> u64 {
        let v = self.version as u64;
        match self.type_id {
            PacketType::Literal => v,
            _ => {
                v + self
                    .subpackets()
                    .iter()
                    .map(|p| p.version_sum())
                    .sum::<u64>()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match self.type_id {
            PacketType::Literal => self.literal(),
            PacketType::Sum => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .sum(),
            PacketType::Product => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .product(),
            PacketType::Minimum => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .min()
                .unwrap(),
            PacketType::Maximum => self
                .subpackets()
                .iter()
                .map(|packet| packet.evaluate())
                .max()
                .unwrap(),
            PacketType::Greater => match self.subpackets().as_slice() {
                [packet_a, packet_b] => {
                    if packet_a.evaluate() > packet_b.evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
            PacketType::Less => match self.subpackets().as_slice() {
                [packet_a, packet_b] => {
                    if packet_a.evaluate() < packet_b.evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
            PacketType::Equal => match self.subpackets().as_slice() {
                [packet_a, packet_b] => {
                    if packet_a.evaluate() == packet_b.evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

pub struct Day16 {
    packets: Vec<Packet>,
}

impl Day16 {
    pub fn new(input: &'static str) -> Self {
        let transmission = Self::parse_transmission(input);
        let packets = Self::parse_packets(transmission);
        Self { packets }
    }

    fn parse_transmission(transmission: &str) -> Vec<u8> {
        let chars = transmission.chars().collect::<Vec<_>>();
        let n_chars = chars.len();

        let mut data = Vec::with_capacity(n_chars);
        for c in 0..(n_chars / 2) {
            let b0 = chars[c * 2].to_digit(16).unwrap() as u8;
            let b1 = chars[(c * 2) + 1].to_digit(16).unwrap() as u8;
            data.push((b0 << 4) | b1);
        }
        if n_chars % 2 == 1 {
            let b = chars[n_chars - 1].to_digit(16).unwrap() as u8;
            data.push(b << 4);
        }

        data
    }

    fn grab_bit(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> u8 {
        let offset = 7 - *bit_offset;
        let mask = 0x1 << offset;
        let bit = (data[*byte_offset] & mask) >> offset;

        *bit_offset += 1;
        if *bit_offset == 8 {
            *byte_offset += 1;
            *bit_offset = 0;
        }

        bit
    }

    fn grab_bits<T, const N: usize>(
        data: &[u8],
        byte_offset: &mut usize,
        bit_offset: &mut usize,
    ) -> T
    where
        T: Integer + NumCast,
    {
        // grab bits
        let mut bits = [0; N];
        for bit in bits.iter_mut().take(N) {
            let offset = 7 - *bit_offset;
            let mask = 0x1 << offset;
            *bit = (data[*byte_offset] & mask) >> offset;

            *bit_offset += 1;
            if *bit_offset == 8 {
                *byte_offset += 1;
                *bit_offset = 0;
            }
        }
        // combine into a single integer
        let mut n = 0u64;
        for (i, &b) in bits.iter().rev().enumerate() {
            n |= (b as u64) << i;
        }
        num::cast(n).unwrap()
    }

    fn parse_packet_header(
        data: &[u8],
        byte_offset: &mut usize,
        bit_offset: &mut usize,
    ) -> (u8, PacketType, u8) {
        let version = Self::grab_bits::<u8, 3>(data, byte_offset, bit_offset);
        let type_id = Self::grab_bits::<u8, 3>(data, byte_offset, bit_offset);
        // note: length type ID is only valid for operators
        let length_type_id = match type_id {
            4 => 0,
            _ => Self::grab_bit(data, byte_offset, bit_offset),
        };

        (version, type_id.into(), length_type_id)
    }

    fn parse_packet_literal(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> u64 {
        let flag = 0x10;

        // grab the chunks of the literal
        let mut chunks = vec![];
        while chunks.is_empty() || chunks[chunks.len() - 1] & flag == flag {
            let chunk = Self::grab_bits::<u8, 5>(data, byte_offset, bit_offset);
            chunks.push(chunk);
        }

        let mut n = 0;
        let mask = 0xF;
        for (byte, chunk) in chunks.iter().rev().enumerate() {
            n |= ((chunk & mask) as u64) << (byte * 4);
        }

        n
    }

    fn parse_packet_operator_length(
        data: &[u8],
        length_type_id: u8,
        byte_offset: &mut usize,
        bit_offset: &mut usize,
    ) -> u16 {
        match length_type_id {
            // operator length is 15 bits
            0 => Self::grab_bits::<u16, 15>(data, byte_offset, bit_offset),
            // operator length is 11 bits
            1 => Self::grab_bits::<u16, 11>(data, byte_offset, bit_offset),
            _ => unreachable!(),
        }
    }

    fn parse_subpacket(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> Packet {
        // parse the packet header
        let (version, type_id, length_type_id) =
            Self::parse_packet_header(data, byte_offset, bit_offset);

        // parse the remaining portion of the packet based on the type ID
        let packet_data = match type_id {
            // literal
            PacketType::Literal => {
                let literal = Self::parse_packet_literal(data, byte_offset, bit_offset);
                PacketData::Literal(literal)
            }
            // operator
            _ => {
                let mut subpackets = vec![];
                let op_length = Self::parse_packet_operator_length(
                    data,
                    length_type_id,
                    byte_offset,
                    bit_offset,
                ) as usize;
                match length_type_id {
                    0 => {
                        // length is the total length in bits of the subpackets
                        let end = (*byte_offset * 8) + *bit_offset + op_length;
                        while (*byte_offset * 8) + *bit_offset < end {
                            let subpacket = Self::parse_subpacket(data, byte_offset, bit_offset);
                            subpackets.push(subpacket);
                        }
                    }
                    1 => {
                        // length is the number of subpackets
                        for _ in 0..op_length {
                            let subpacket = Self::parse_subpacket(data, byte_offset, bit_offset);
                            subpackets.push(subpacket);
                        }
                    }
                    _ => unreachable!(),
                }
                PacketData::Subpackets(subpackets)
            }
        };

        Packet {
            version,
            type_id,
            length_type_id,
            data: packet_data,
        }
    }

    fn parse_packet(data: &[u8], byte_offset: &mut usize, bit_offset: &mut usize) -> Packet {
        // parse the packet header
        let (version, type_id, length_type_id) =
            Self::parse_packet_header(data, byte_offset, bit_offset);

        // parse the remaining portion of the packet based on the type ID
        let packet_data = match type_id {
            // literal
            PacketType::Literal => {
                let literal = Self::parse_packet_literal(data, byte_offset, bit_offset);
                PacketData::Literal(literal)
            }
            // operator
            _ => {
                let mut subpackets = vec![];
                let op_length = Self::parse_packet_operator_length(
                    data,
                    length_type_id,
                    byte_offset,
                    bit_offset,
                ) as usize;
                match length_type_id {
                    0 => {
                        // length is the total length in bits of the subpackets
                        let end = (*byte_offset * 8) + *bit_offset + op_length;
                        while (*byte_offset * 8) + *bit_offset < end {
                            let subpacket = Self::parse_subpacket(data, byte_offset, bit_offset);
                            subpackets.push(subpacket);
                        }
                    }
                    1 => {
                        // length is the number of subpackets
                        for _ in 0..op_length {
                            let subpacket = Self::parse_subpacket(data, byte_offset, bit_offset);
                            subpackets.push(subpacket);
                        }
                    }
                    _ => unreachable!(),
                }
                PacketData::Subpackets(subpackets)
            }
        };

        // account for trailing bits
        if *bit_offset != 0 {
            *byte_offset += 1;
            *bit_offset = 0;
        }

        Packet {
            version,
            type_id,
            length_type_id,
            data: packet_data,
        }
    }

    fn parse_packets(transmission: Vec<u8>) -> Vec<Packet> {
        let mut packets = vec![];
        let mut byte_offset = 0;
        let mut bit_offset = 0;

        while byte_offset < transmission.len() {
            let packet = Self::parse_packet(&transmission, &mut byte_offset, &mut bit_offset);
            packets.push(packet);
        }
        packets
    }
}

impl Puzzle for Day16 {
    // Decode the structure of your hexadecimal-encoded BITS transmission; what do you get if you
    // add up the version numbers in all packets?
    fn part_1(&self) -> Result<Solution> {
        let version_sum = self
            .packets
            .iter()
            .map(|packet| packet.version_sum())
            .sum::<u64>();
        Ok(version_sum.into())
    }

    // What do you get if you evaluate the expression represented by your hexadecimal-encoded
    // BITS transmission?
    fn part_2(&self) -> Result<Solution> {
        let packet = &self.packets[0];
        Ok(packet.evaluate().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_packets(transmission: &str) -> Vec<Packet> {
        let data = Day16::parse_transmission(transmission);
        Day16::parse_packets(data)
    }

    #[test]
    fn test_parse_packet_literal() {
        let packets = parse_packets("D2FE28");
        assert_eq!(packets.len(), 1);

        let packet = &packets[0];
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, PacketType::Literal);
        assert_eq!(packet.data, PacketData::Literal(2021));
    }

    #[test]
    fn test_parse_packets_multiple_literals() {
        let packets = parse_packets("D2FE28D2FE28D2FE28D2FE28");
        assert_eq!(packets.len(), 4);

        for packet in packets.iter() {
            assert_eq!(packet.version, 6);
            assert_eq!(packet.type_id, PacketType::Literal);
            assert_eq!(packet.data, PacketData::Literal(2021));
        }
    }

    #[test]
    fn test_parse_packet_operator_length_type_0() {
        let packets = parse_packets("38006F45291200");
        assert_eq!(packets.len(), 1);

        let packet = &packets[0];
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, PacketType::Less);
        assert_eq!(packet.length_type_id, 0);
        assert!(matches!(packet.data, PacketData::Subpackets(_)));
        assert_eq!(packet.subpackets().len(), 2);

        let subpacket = &packet.subpackets()[0];
        assert_eq!(subpacket.data, PacketData::Literal(10));

        let subpacket = &packet.subpackets()[1];
        assert_eq!(subpacket.data, PacketData::Literal(20));
    }

    #[test]
    fn test_parse_packet_operator_length_type_1() {
        let packets = parse_packets("EE00D40C823060");
        assert_eq!(packets.len(), 1);

        let packet = &packets[0];
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, PacketType::Maximum);
        assert_eq!(packet.length_type_id, 1);
        assert!(matches!(packet.data, PacketData::Subpackets(_)));
        assert_eq!(packet.subpackets().len(), 3);

        let subpacket = &packet.subpackets()[0];
        assert_eq!(subpacket.data, PacketData::Literal(1));

        let subpacket = &packet.subpackets()[1];
        assert_eq!(subpacket.data, PacketData::Literal(2));

        let subpacket = &packet.subpackets()[2];
        assert_eq!(subpacket.data, PacketData::Literal(3));
    }

    #[test]
    fn test_evaluate_packets() {
        let packet = &parse_packets("C200B40A82")[0];
        assert_eq!(packet.evaluate(), 3);

        let packet = &parse_packets("04005AC33890")[0];
        assert_eq!(packet.evaluate(), 54);

        let packet = &parse_packets("880086C3E88112")[0];
        assert_eq!(packet.evaluate(), 7);

        let packet = &parse_packets("CE00C43D881120")[0];
        assert_eq!(packet.evaluate(), 9);
    }
}
