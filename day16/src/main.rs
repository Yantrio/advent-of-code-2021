fn main() {
    let hex = include_str!("input");

    let mut bits = hex_to_bits(hex).into_iter();
    let parsed = Packet::from_raw(bits.by_ref()).unwrap();

    println!(
        "Part 1: {} (version number sum)",
        parsed.get_version_number_sum()
    );

    println!("Part 2: {} (evaluated output)", parsed.value);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
    header: Header,
    value: usize,
    subpackets: Option<Vec<Packet>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Header {
    version: usize,
    type_id: usize,
}

impl Packet {
    //
    fn from_raw<I>(source: &mut I) -> Option<Self>
    where
        I: Iterator<Item = usize>,
    {
        let header_to_parse = source.by_ref().take(6).collect::<Vec<_>>();
        if header_to_parse.is_empty() {
            return None;
        }
        let header = Header::from(header_to_parse.as_slice());

        let mut result = match header.type_id == 4 {
            true => Packet {
                header,
                value: parse_literal_value(source),
                subpackets: None,
            },
            false => Packet {
                header,
                value: 0,
                subpackets: Some(parse_operator(source)),
            },
        };

        result.value = result.get_value();
        Some(result)
    }

    fn get_value(&self) -> usize {
        match self.header.type_id == 4 {
            true => self.value,
            false => {
                let subpackets = self.subpackets.clone().unwrap();
                let mut sp_values = subpackets.iter().map(|sp| sp.get_value());
                match self.header.type_id {
                    // sum
                    0 => sp_values.sum::<usize>(),
                    // product
                    1 => sp_values.product(),
                    // minimum
                    2 => sp_values.min().unwrap(),
                    // maximum
                    3 => sp_values.max().unwrap(),
                    // gt
                    5 => (sp_values.next().unwrap() > sp_values.next().unwrap()) as usize,
                    // lt
                    6 => (sp_values.next().unwrap() < sp_values.next().unwrap()) as usize,
                    // eq
                    7 => (sp_values.next().unwrap() == sp_values.next().unwrap()) as usize,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn get_version_number_sum(&self) -> usize {
        match &self.subpackets {
            Some(subpackets) => subpackets.iter().fold(self.header.version, |acc, spl| {
                acc + spl.get_version_number_sum()
            }),
            None => self.header.version,
        }
    }
}

fn parse_operator<I>(source: &mut I) -> Vec<Packet>
where
    I: Iterator<Item = usize>,
{
    let length_type_id = source.next().unwrap();
    let mut result = vec![];

    match length_type_id {
        0 => {
            /*
                If the length type ID is 0, then the next 15 bits are a number
                that represents the total length in bits of the sub-packets
                contained by this packet.
            */
            let length = bits_to_decimal(source.by_ref().take(15).collect::<Vec<_>>().as_slice());
            let mut subpackets_raw = source.by_ref().take(length).collect::<Vec<_>>().into_iter();
            while let Some(subpacket) = Packet::from_raw(&mut subpackets_raw) {
                result.push(subpacket);
            }
        }
        1 => {
            /*
               If the length type ID is 1,
               then the next 11 bits are a number that represents
               the number of sub-packets immediately contained by this packet.
            */
            let mut to_parse =
                bits_to_decimal(source.by_ref().take(11).collect::<Vec<_>>().as_slice());
            while to_parse != 0 {
                result.push(Packet::from_raw(source).unwrap());
                to_parse -= 1;
            }
        }
        _ => unreachable!(),
    }

    result
}

fn parse_literal_value<I>(source: &mut I) -> usize
where
    I: Iterator<Item = usize>,
{
    let mut binary = vec![];

    loop {
        let chunk = source.take(5).collect::<Vec<_>>();
        if chunk[0] == 1 {
            binary.append(&mut chunk[1..5].to_vec());
        } else {
            binary.append(&mut chunk[1..5].to_vec());
            break;
        }
    }
    bits_to_decimal(&binary)
}

impl From<&[usize]> for Header {
    fn from(source: &[usize]) -> Self {
        Header {
            version: bits_to_decimal(&source[0..3]),
            type_id: bits_to_decimal(&source[3..6]),
        }
    }
}

fn bits_to_decimal(bits: &[usize]) -> usize {
    bits.iter().fold(0, |result, &bit| (result << 1) ^ bit)
}

fn hex_to_bits(str: &str) -> Vec<usize> {
    str.chars().fold(vec![], |mut acc, val| {
        acc.append(&mut to_binary(val));
        acc
    })
}

// dont judge me!
fn to_binary(c: char) -> Vec<usize> {
    match c {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_result(input: &str, expected: usize) {
        let bits = hex_to_bits(input);
        let result = Packet::from_raw(&mut bits.into_iter()).unwrap();

        assert_eq!(expected, result.value)
    }

    // C200B40A82 finds the sum of 1 and 2, resulting in the value 3.
    #[test]
    fn sum_test() {
        test_result("C200B40A82", 3);
    }

    // 04005AC33890 finds the product of 6 and 9, resulting in the value 54.
    #[test]
    fn product_test() {
        test_result("04005AC33890", 54);
    }

    // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7.
    #[test]
    fn min_test() {
        test_result("880086C3E88112", 7);
    }

    // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9.
    #[test]
    fn max_test() {
        test_result("CE00C43D881120", 9);
    }

    // D8005AC2A8F0 produces 1, because 5 is less than 15.
    #[test]
    fn lt_test() {
        test_result("D8005AC2A8F0", 1);
    }

    // F600BC2D8F produces 0, because 5 is not greater than 15.
    #[test]
    fn gt_test() {
        test_result("F600BC2D8F", 0);
    }

    // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
    #[test]
    fn eq_test() {
        test_result("9C005AC2F8F0", 0);
    }

    // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2.
    #[test]
    fn nested_sum_prod_eq_test() {
        test_result("9C0141080250320F1802104A08", 1);
    }

    #[test]
    fn hex_to_bits_test() {
        let input = "D2FE28";
        let result = hex_to_bits(input);
        let expected = vec![
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ];

        assert_eq!(result, expected)
    }

    #[test]
    fn bits_to_decimal_test() {
        let input = vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1];
        let result = bits_to_decimal(&input);
        assert_eq!(result, 2021);
    }

    #[test]
    fn parse_literal_value_example_1() {
        let mut test = [
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ]
        .into_iter();
        let parsed = Packet::from_raw(&mut test);
        let expected = Packet {
            header: Header {
                version: 6,
                type_id: 4,
            },
            value: 2021,
            subpackets: None,
        };

        assert_eq!(parsed, Some(expected));
    }

    #[test]
    fn parse_operator_example_1() {
        let mut test = [
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
        .into_iter();

        let parsed = Packet::from_raw(&mut test);
        let expected = Packet {
            header: Header {
                version: 1,
                type_id: 6,
            },
            value: 1,
            subpackets: Some(vec![
                Packet {
                    header: Header {
                        version: 6,
                        type_id: 4,
                    },
                    value: 10,
                    subpackets: None,
                },
                Packet {
                    header: Header {
                        version: 2,
                        type_id: 4,
                    },
                    value: 20,
                    subpackets: None,
                },
            ]),
        };

        assert_eq!(parsed, Some(expected));
    }
}
