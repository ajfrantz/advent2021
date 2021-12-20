use color_eyre::eyre::Result;

use bitvec::prelude::*;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

#[derive(Debug)]
enum Payload {
    Literal(u64),
    Operator(Vec<Packet>),
}

fn parse_packet(bits: &BitSlice<Msb0, u8>) -> (Packet, &BitSlice<Msb0, u8>) {
    let version = bits[0..3].load_be();
    let type_id = bits[3..6].load_be();

    let (payload, tail) = match type_id {
        4 => parse_literal(&bits[6..]),
        _ => parse_operator(&bits[6..]),
    };

    (Packet {
        version,
        type_id,
        payload,
    }, tail)
}

fn parse_literal(mut bits: &BitSlice<Msb0, u8>) -> (Payload, &BitSlice<Msb0, u8>) {
    let mut value = 0u64;
    let mut keep_going = true;
    while keep_going {
        keep_going = bits[0];
        let word: u64 = bits[1..5].load_be();
        value = (value << 4) | word;
        bits = &bits[5..];
    }

    (Payload::Literal(value), bits)
}

fn parse_operator(bits: &BitSlice<Msb0, u8>) -> (Payload, &BitSlice<Msb0, u8>) {
    let length_type_id = bits[0];
    if length_type_id {
        parse_operator_by_number_of_subpackets(&bits[1..])
    } else {
        parse_operator_by_number_of_bits(&bits[1..])
    }
}

fn parse_operator_by_number_of_subpackets(mut bits: &BitSlice<Msb0, u8>) -> (Payload, &BitSlice<Msb0, u8>) {
    let num_subpackets: usize = bits[0..11].load_be();
    bits = &bits[11..];

    let mut packets = Vec::with_capacity(num_subpackets);
    for _ in 0..num_subpackets {
        let (packet, tail) = parse_packet(&bits);
        packets.push(packet);
        bits = tail;
    }

    (Payload::Operator(packets), bits)
}

fn parse_operator_by_number_of_bits(mut bits: &BitSlice<Msb0, u8>) -> (Payload, &BitSlice<Msb0, u8>) {
    let mut bits_remaining: usize = bits[0..15].load_be();
    bits = &bits[15..];

    let mut packets = Vec::new();
    while bits_remaining > 0 {
        let (packet, tail) = parse_packet(&bits);
        packets.push(packet);

        let bits_used = bits.len() - tail.len();
        bits_remaining -= bits_used;
        bits = tail;
    }

    (Payload::Operator(packets), bits)
}

fn version_sum(packets: &[Packet]) -> usize {
    let mut sum = 0;
    for packet in packets {
        sum += usize::from(packet.version);
        if let Payload::Operator(subpackets) = &packet.payload {
            sum += version_sum(&subpackets);
        }
    }
    sum
}

fn evaluate(packet: &Packet) -> u64 {
    if packet.type_id == 4 {
        match packet.payload {
            Payload::Literal(v) => return v,
            _ => panic!("non-literal literal"),
        }
    }

    let subpackets = match &packet.payload {
        Payload::Operator(subpackets) => subpackets,
        _ => panic!("non-operator operator"),
    };

    let mut values = subpackets.iter().map(evaluate);
    match packet.type_id {
        0 => values.sum(),
        1 => values.product(),
        2 => values.min().unwrap(),
        3 => values.max().unwrap(),
        5 => if values.next().unwrap() > values.next().unwrap() { 1 } else { 0 },
        6 => if values.next().unwrap() < values.next().unwrap() { 1 } else { 0 },
        7 => if values.next().unwrap() == values.next().unwrap() { 1 } else { 0 },
        _ => panic!("illegal type id"),
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = INPUT;

    let digits: Vec<_> = input.chars().map(|c| c.to_digit(16).unwrap() as u8).collect();
    let data: Vec<_> = digits.chunks(2).map(|digits| digits[0] << 4 | digits[1]).collect();

    let bits = data.view_bits::<Msb0>();

    let (packet, _) = parse_packet(&bits);
    dbg!(&packet);
    dbg!(evaluate(&packet));
    dbg!(version_sum(&[packet]));

    Ok(())
}

#[allow(dead_code)]
//const EXAMPLE: &'static str = "8A004A801A8002F478";
const EXAMPLE: &'static str = "A0016C880162017C3686B18A3D4780";

const INPUT: &'static str = "820D4A801EE00720190CA005201682A00498014C04BBB01186C040A200EC66006900C44802BA280104021B30070A4016980044C800B84B5F13BFF007081800FE97FDF830401BF4A6E239A009CCE22E53DC9429C170013A8C01E87D102399803F1120B4632004261045183F303E4017DE002F3292CB04DE86E6E7E54100366A5490698023400ABCC59E262CFD31DDD1E8C0228D938872A472E471FC80082950220096E55EF0012882529182D180293139E3AC9A00A080391563B4121007223C4A8B3279B2AA80450DE4B72A9248864EAB1802940095CDE0FA4DAA5E76C4E30EBE18021401B88002170BA0A43000043E27462829318F83B00593225F10267FAEDD2E56B0323005E55EE6830C013B00464592458E52D1DF3F97720110258DAC0161007A084228B0200DC568FB14D40129F33968891005FBC00E7CAEDD25B12E692A7409003B392EA3497716ED2CFF39FC42B8E593CC015B00525754B7DFA67699296DD018802839E35956397449D66997F2013C3803760004262C4288B40008747E8E114672564E5002256F6CC3D7726006125A6593A671A48043DC00A4A6A5B9EAC1F352DCF560A9385BEED29A8311802B37BE635F54F004A5C1A5C1C40279FDD7B7BC4126ED8A4A368994B530833D7A439AA1E9009D4200C4178FF0880010E8431F62C880370F63E44B9D1E200ADAC01091029FC7CB26BD25710052384097004677679159C02D9C9465C7B92CFACD91227F7CD678D12C2A402C24BF37E9DE15A36E8026200F4668AF170401A8BD05A242009692BFC708A4BDCFCC8A4AC3931EAEBB3D314C35900477A0094F36CF354EE0CCC01B985A932D993D87E2017CE5AB6A84C96C265FA750BA4E6A52521C300467033401595D8BCC2818029C00AA4A4FBE6F8CB31CAE7D1CDDAE2E9006FD600AC9ED666A6293FAFF699FC168001FE9DC5BE3B2A6B3EED060";
