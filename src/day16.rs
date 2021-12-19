use std::str::Chars;

#[aoc_generator(day16)]
fn generator_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> i32 {
    let binary = convert_to_binary_from_hex(input);
    let mut binary_chars = binary.chars();

    let packet = parse_packet(&mut binary_chars).unwrap();

    sum_of_versions(&packet)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    let binary = convert_to_binary_from_hex(input);
    let mut binary_chars = binary.chars();

    let packet = parse_packet(&mut binary_chars).unwrap();

    eval(&packet)
}

fn parse_packet(binary_chars: &mut Chars) -> Option<Packet> {
    let first_three = &binary_chars.by_ref().take(3).collect::<String>();
    if first_three.is_empty() {
        return None;
    }

    let version = to_decimal(first_three) as i32;
    let type_id = to_decimal(&binary_chars.by_ref().take(3).collect::<String>());

    match type_id {
        4 => Some(parse_literal(version, binary_chars)),
        i => Some(parse_operator(version, i.try_into().unwrap(), binary_chars)),
    }
}

fn parse_literal(version: i32, binary_chars: &mut Chars) -> Packet {
    let mut bits = String::new();

    while binary_chars.by_ref().next().unwrap().to_digit(10).unwrap() == 1 {
        binary_chars.by_ref().take(4).for_each(|ch| bits.push(ch));
    }
    binary_chars.by_ref().take(4).for_each(|ch| bits.push(ch));

    let literal = Some(to_decimal(&bits));

    Packet {
        version,
        literal,
        operator: None,
        sub_packets: vec![],
    }
}

fn parse_operator(version: i32, type_id: i32, binary_chars: &mut Chars) -> Packet {
    let length_type_id = binary_chars.next().unwrap();
    let mut sub_packets = vec![];
    match length_type_id {
        '0' => {
            let length = to_decimal(&binary_chars.take(15).collect::<String>());
            let sub_binary = binary_chars
                .by_ref()
                .take(length as usize)
                .collect::<String>();
            let mut sub_binary_iter = sub_binary.chars();
            while let Some(sub_packet) = parse_packet(&mut sub_binary_iter) {
                sub_packets.push(sub_packet);
            }
        }
        '1' => {
            let n_packets = to_decimal(&binary_chars.by_ref().take(11).collect::<String>());
            for _ in 0..n_packets {
                sub_packets.push(parse_packet(binary_chars).unwrap());
            }
        }
        _ => unreachable!(),
    };

    let operator = Some(match type_id {
        0 => Operator::Sum,
        1 => Operator::Product,
        2 => Operator::Minimum,
        3 => Operator::Maximum,
        5 => Operator::Greater,
        6 => Operator::Lesser,
        7 => Operator::Equal,
        _ => unreachable!(),
    });

    Packet {
        version,
        literal: None,
        operator,
        sub_packets,
    }
}

fn sum_of_versions(packet: &Packet) -> i32 {
    packet
        .sub_packets
        .iter()
        .fold(packet.version, |acc, p| acc + sum_of_versions(p))
}

fn eval(packet: &Packet) -> u64 {
    if let Some(value) = packet.literal {
        return value as u64;
    }

    match packet.operator.as_ref().unwrap() {
        Operator::Sum => packet
            .sub_packets
            .iter()
            .fold(0, |acc, packet| acc + eval(packet)),
        Operator::Product => packet
            .sub_packets
            .iter()
            .fold(1, |acc, packet| acc * eval(packet)),
        Operator::Minimum => packet
            .sub_packets
            .iter()
            .fold(u64::MAX, |acc, packet| acc.min(eval(packet))),
        Operator::Maximum => packet
            .sub_packets
            .iter()
            .fold(u64::MIN, |acc, packet| acc.max(eval(packet))),
        Operator::Greater => {
            if eval(&packet.sub_packets[0]) > eval(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        Operator::Lesser => {
            if eval(&packet.sub_packets[0]) < eval(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        Operator::Equal => {
            if eval(&packet.sub_packets[0]) == eval(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
    }
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

fn to_decimal(binary: &str) -> isize {
    isize::from_str_radix(binary, 2).unwrap()
}

struct Packet {
    version: i32,
    literal: Option<isize>,
    operator: Option<Operator>,
    sub_packets: Vec<Packet>,
}

enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Lesser,
    Equal,
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT: &str = r#"D2FE28"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT);
        assert_eq!(input, INPUT);
    }

    #[test]
    fn day16_part1() {
        let input = generator_input(INPUT);
        assert_eq!(part1(&input), 6);
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn day16_part2() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}
