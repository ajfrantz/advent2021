use color_eyre::eyre::Result;
use reformation::Reformation;

use std::collections::HashMap;

#[derive(Reformation)]
#[reformation("{first}{second} -> {insertion}")]
struct Rule {
    first: char,
    second: char,
    insertion: char,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = INPUT;
    let mut lines = input.split("\n");

    let template: Vec<u8> = lines.next().unwrap().bytes().collect();
    lines.next();

    let rules: HashMap<_, _> = lines
        .map(|l| Rule::parse(l).unwrap())
        .map(|rule| ([rule.first as u8, rule.second as u8], rule.insertion as u8))
        .collect();

    let mut pairs: HashMap<[u8; 2], u64> = HashMap::new();
    for pair in template.windows(2) {
        *pairs.entry(pair.try_into().unwrap()).or_insert(0) += 1;
    }

    // Part One
    for _ in 0..10 {
        let mut next = HashMap::new();
        for (pair, count) in pairs {
            // NN + (NN -> C) => NC + CN
            let inserted = *rules.get(&pair).unwrap();
            let left_pair = [pair[0], inserted];
            let right_pair = [inserted, pair[1]];
            *next.entry(left_pair).or_insert(0) += count;
            *next.entry(right_pair).or_insert(0) += count;
        }
        pairs = next;
    }

    let mut frequencies = HashMap::new();
    for (pair, count) in &pairs {
        *frequencies.entry(pair[0]).or_insert(0) += count;
        // don't take the second one, they all show up twice except for the very last character...
    }
    // ...so take care of the last character separately
    *frequencies.entry(*template.last().unwrap()).or_insert(0) += 1;

    let most_common = frequencies.values().max().unwrap();
    let least_common = frequencies.values().min().unwrap();
    dbg!(most_common - least_common);

    // Part Two
    for _ in 10..40 {
        let mut next = HashMap::new();
        for (pair, count) in pairs {
            // NN + (NN -> C) => NC + CN
            let inserted = *rules.get(&pair).unwrap();
            let left_pair = [pair[0], inserted];
            let right_pair = [inserted, pair[1]];
            *next.entry(left_pair).or_insert(0) += count;
            *next.entry(right_pair).or_insert(0) += count;
        }
        pairs = next;
    }

    let mut frequencies = HashMap::new();
    for (pair, count) in &pairs {
        *frequencies.entry(pair[0]).or_insert(0) += count;
        // don't take the second one, they all show up twice except for the very last character...
    }
    // ...so take care of the last character separately
    *frequencies.entry(*template.last().unwrap()).or_insert(0) += 1;

    let most_common = frequencies.values().max().unwrap();
    let least_common = frequencies.values().min().unwrap();
    dbg!(most_common - least_common);

    Ok(())
}

#[allow(dead_code)]
const EXAMPLE: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

const INPUT: &'static str = "NCOPHKVONVPNSKSHBNPF

ON -> C
CK -> H
HC -> B
NP -> S
NH -> H
CB -> C
BB -> H
BC -> H
NN -> C
OH -> B
SF -> V
PB -> H
CP -> P
BN -> O
NB -> B
KB -> P
PV -> F
SH -> V
KP -> S
OF -> K
BS -> V
PF -> O
BK -> S
FB -> B
SV -> B
BH -> V
VK -> N
CS -> V
FV -> F
HS -> C
KK -> O
SP -> N
FK -> B
CF -> C
HP -> F
BF -> O
KC -> C
VP -> O
BP -> P
FF -> V
NO -> C
HK -> C
HV -> B
PK -> P
OV -> F
VN -> H
PC -> K
SB -> H
VO -> V
BV -> K
NC -> H
OB -> S
SN -> B
HF -> P
VF -> B
HN -> H
KS -> S
SC -> S
CV -> B
NS -> P
KO -> V
FS -> O
PH -> K
BO -> C
FH -> B
CO -> O
FO -> F
VV -> N
CH -> V
NK -> N
PO -> K
OK -> K
PP -> O
OC -> P
FC -> N
VH -> S
PN -> C
VB -> C
VS -> P
HO -> F
OP -> S
HB -> N
CC -> K
KN -> S
SK -> C
OS -> N
KH -> B
FP -> S
NF -> S
CN -> S
KF -> C
SS -> C
SO -> S
NV -> O
FN -> B
PS -> S
HH -> C
VC -> S
OO -> C
KV -> P";
