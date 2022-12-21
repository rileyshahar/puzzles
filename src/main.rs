#![feature(test)]

use pj_euler::{aoc, euler};

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().unwrap_or_default().as_str() {
        "--euler" | "" => euler(args.next().unwrap_or_default().as_str()),
        "--aoc" => aoc(args.next().unwrap_or_default().as_str()),
        unknown => panic!("please specify --aoc or --euler; {unknown} is not a valid argument"),
    }
}

/// Generate a main function which relies on `solve` and a benchmark.
macro_rules! problems {
    ( $($mod:ident: $($num:pat => $name:ident)* );* ) => {
        $(
            fn $mod(s: &str) {
                let solution = match s {
                    $(
                        $num => $mod::$name::solve().to_string(),
                    )*
                    unknown => panic!("no matching problem: {}", unknown)
                };
                println!("{}", solution);
            }
        )*


        #[cfg(test)]
        mod problem_bench {
            $(
                mod $mod {
                    extern crate test;
                    use test::Bencher;
                    $(
                        #[bench]
                        fn $name(b: &mut Bencher) {
                            b.iter(|| pj_euler::$mod::$name::solve());
                        }
                    )*
                }
            )*
        }
    };
}

problems! {
    euler:
    "1" => p001
    "2" => p002
    "3" => p003
    "4" => p004
    "5" => p005
    "6" => p006
    "7" => p007
    "8" => p008
    "9" => p009
    "10" => p010
    "11" => p011
    "12" => p012
    "13" => p013
    "14" => p014
    "15" => p015
    "16" => p016
    "17" => p017
    "18" => p018
    "19" => p019
    "20" => p020
    "21" => p021
    "22" => p022
    "23" => p023
    ;

    aoc:
    "21-1" => y21p01
    "21-2" => y21p02
    "21-3" => y21p03
    "21-4" => y21p04
    "21-5" => y21p05
    "21-6" => y21p06
    "21-7" => y21p07
    "21-8" => y21p08
    "21-9" => y21p09
    "21-10" => y21p10
    "21-11" => y21p11
    "21-12" => y21p12
    "21-13" => y21p13
    "21-14" => y21p14
    "21-15" => y21p15
    "21-17" => y21p17

    "22-01" => y22p01
}
