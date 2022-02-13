use core::fmt::{Debug, Display, Formatter, Write};
use std::io::Cursor;
use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};
use itertools::Itertools;
use textwrap::indent;
use crate::day16::OperatorType::{Equal, GreaterThan, LessThan, Max, Min, Product, Sum};
use crate::day3::bits_bool_to_usize;
use crate::read_lines;

#[derive(Debug, Copy, Clone)]
struct PacketVersion(usize);
impl PacketVersion {
    fn new(bits: &[bool]) -> Self {
        Self(bits_bool_to_usize(&bits[0..3]))
    }
}

#[derive(Debug, Copy, Clone)]
enum OperatorType {
    Sum, Product, Min, Max, GreaterThan, LessThan, Equal
}

impl OperatorType {
    fn value(&self, body: &PacketBodyOperator) -> usize {
        match self {
            Sum => body.packets.iter().map(|p| p.value()).sum(),
            Product => body.packets.iter().fold(1usize, |a, p| a * p.value()),
            Min => body.packets.iter().map(|p| p.value()).min().unwrap(),
            Max => body.packets.iter().map(|p| p.value()).max().unwrap(),
            GreaterThan => if body.packets[0].value() > body.packets[1].value() { 1 } else { 0 },
            LessThan => if body.packets[0].value() < body.packets[1].value() { 1 } else { 0 },
            Equal => if body.packets[0].value() == body.packets[1].value() { 1 } else { 0 }
        }
    }

    fn new(id: usize) -> Option<Self> {
        match id {
            0 => Some(Sum),
            1 => Some(Product),
            2 => Some(Min),
            3 => Some(Max),
            5 => Some(GreaterThan),
            6 => Some(LessThan),
            7 => Some(Equal),
            _ => None
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum PacketType {
    Literal,
    Operator(OperatorType)
}
impl PacketType {
    fn new(bits: &[bool]) -> Self {
        match bits_bool_to_usize(&bits[3..6]) {
            4 => PacketType::Literal,
            other => PacketType::Operator(OperatorType::new(other).unwrap())
        }
    }
}

#[derive(Debug)]
struct PacketBodyLiteral { value: usize, len_bits: usize }
impl PacketBodyLiteral {
    fn read(body: &[bool]) -> Self {
        // println!("src={}", Bits { bits: Vec::from(slice) });
        let mut idx = 0usize;
        let mut bits = Vec::<bool>::new();

        loop {
            let last_group = !body[idx];

            bits.push(body[idx + 1]);
            bits.push(body[idx + 2]);
            bits.push(body[idx + 3]);
            bits.push(body[idx + 4]);
            // println!("idx={}, bits={}", idx, Bits { bits: bits.clone() });
            idx += 5;

            if last_group { break }
        }

        let value = bits_bool_to_usize(&bits);
        let len_bits = idx;
        let body = Self { value, len_bits };
        body
    }
}

#[derive(Debug)]
struct PacketBodyOperator { length: OperatorLengthType, packets: Vec<Packet> }
impl PacketBodyOperator {
    fn len_bits(&self) -> usize {
        1 + self.length.bits() + self.packets.iter().map(|p| p.len_bits()).sum::<usize>()
    }

    /// Returns the body slice without the operator part.
    fn body_without_header<'a>(body: &'a [bool], length: &OperatorLengthType) -> &'a [bool] {
        &body[(1 + length.bits())..]
    }

    fn read(body: &[bool]) -> Self {
        let length = OperatorLengthType::parse(body);
        println!("packet operator body length type: {:?}", length);

        let mut counter = 0usize;
        let mut packets = Vec::<Packet>::new();
        let mut packets_slice = Self::body_without_header(body, &length);
        loop {
            println!("Reading sub-packet #{}", packets.len());

            let packet = Packet::new(Bits { bits: Vec::from(packets_slice) });
            println!("Read sub-packet #{}: {}", packets.len(), packet);
            let packet_len = packet.len_bits();
            packets.push(packet);

            packets_slice = &packets_slice[packet_len..];

            match length {
                OperatorLengthType::TotalLengthInBits { length } => {
                    counter += packet_len;
                    println!("Read {} bits, a total of {} bits", packet_len, counter);
                    if counter >= length { break }
                }
                OperatorLengthType::NumberOfSubPackets { number } => {
                    counter += 1;
                    println!("Read a packet, a total of {} packets", counter);
                    if counter >= number { break }
                }
            }
        }

        PacketBodyOperator { length, packets }
    }
}

#[derive(Debug)]
enum PacketBody {
    Literal(PacketBodyLiteral),
    Operator(PacketBodyOperator)
}
impl PacketBody {
    fn len_bits(&self) -> usize {
        match self {
            PacketBody::Literal(lit) => lit.len_bits,
            PacketBody::Operator(op) => op.len_bits()
        }
    }

    fn new(type_: &PacketType, body: &[bool]) -> PacketBody {
        println!("Reading packet body");
        let body = match type_ {
            PacketType::Literal => PacketBody::Literal(PacketBodyLiteral::read(body)),
            PacketType::Operator(_) => PacketBody::Operator(PacketBodyOperator::read(body))
        };
        println!("Read packet body: {:?}", body);
        body
    }
}

#[derive(Debug)]
enum OperatorLengthType {
    TotalLengthInBits { length: usize },
    NumberOfSubPackets { number: usize }
}
impl OperatorLengthType {
    fn bits(&self) -> usize {
        match self {
            OperatorLengthType::TotalLengthInBits { .. } => 15,
            OperatorLengthType::NumberOfSubPackets { .. } => 11
        }
    }

    fn parse(body: &[bool]) -> Self {
        match body[0] {
            false => Self::TotalLengthInBits { length: bits_bool_to_usize(&body[1..16]) as usize },
            true => Self::NumberOfSubPackets { number: bits_bool_to_usize(&body[1..12]) as usize },
        }
    }
}

struct Bits {
    bits: Vec<bool>
}
impl Bits {
    fn from_str(s: &str) -> Self {
        let decoded = hex::decode(s).unwrap();
        let cursor = Cursor::new(decoded);
        let mut reader =
            BitReader::endian(cursor, BigEndian);
        let mut bits = Vec::<bool>::new();
        while let Ok(bit) = reader.read_bit() {
            bits.push(bit);
        }
        Self { bits }
    }
}
impl Display for Bits {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for bit in &self.bits {
            f.write_char(if *bit { '1' } else { '0' })?;
        }

        Ok(())
    }
}

struct Packet {
    bits: Bits,
    type_: PacketType,
    body: PacketBody
}
impl Packet {
    fn new(bits: Bits) -> Self {
        let type_ = PacketType::new(&bits.bits);
        println!("Read packet type: {:?}", type_);
        let body = PacketBody::new(&type_, Self::body_slice_(&bits.bits));
        Self { bits, type_, body }
    }

    fn from_str(s: &str) -> Self {
        Self::new(Bits::from_str(s))
    }

    fn len_bits(&self) -> usize {
        6 + self.body.len_bits()
    }

    fn version(&self) -> PacketVersion {
        PacketVersion::new(&self.bits.bits)
    }

    fn body_slice_(bits: &[bool]) -> &[bool] {
        &bits[6..]
    }

    fn value(&self) -> usize {
        match &self.body {
            PacketBody::Literal(lit) => lit.value,
            PacketBody::Operator(op_body) => {
                match self.type_ {
                    PacketType::Literal => panic!(),
                    PacketType::Operator(op) => op.value(op_body)
                }
            }
        }
    }

    fn recursive<'a, A : 'a>(&'a self, f: &'a impl Fn(&Packet) -> A) -> Vec<A> {
        vec![self].iter().flat_map(|p| {
            let mut v = vec![f(p)];
            match &p.body {
                PacketBody::Literal(_) => {}
                PacketBody::Operator(op) => {
                    for p in &op.packets {
                        for a in p.recursive(f) {
                            v.push(a);
                        }
                    }
                }
            }
            v
        }).collect_vec()
    }
}
impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Packet[\n  {}\n  {:?}, len_bits={}, type={:?}\n  body:\n{}\n]",
            &self.bits, self.version(), self.len_bits(), self.type_,
            indent(format!("{:?}", self.body).as_str(), "    ")
        ))
    }
}
impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

fn read() -> Vec<(String, Packet)> {
    read_lines("data/day16.txt").map(|line| {
        println!("Parsing {}", line);
        let packet = Packet::from_str(line.as_str());
        (line, packet)
    }).collect_vec()
}

pub fn part1() {
    let all_bits = read();
    for (source, packet) in all_bits {
        let versions_sum =
            packet.recursive(&|p| p.version()).iter().map(|v| v.0)
                .sum::<usize>();

        println!("##### {}", source);
        println!("{}", packet);
        println!("versions_sum={}", versions_sum);
        println!("value={}", packet.value());
        println!("#####\n\n");
    }
}