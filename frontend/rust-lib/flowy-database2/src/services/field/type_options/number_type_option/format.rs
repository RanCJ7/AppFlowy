#![allow(clippy::upper_case_acronyms)]

use lazy_static::lazy_static;
use rusty_money::define_currency_set;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

lazy_static! {
  pub static ref CURRENCY_SYMBOL: Vec<String> = NumberFormat::iter()
    .map(|format| format.symbol())
    .collect::<Vec<String>>();
  pub static ref STRIP_SYMBOL: Vec<String> = vec![",".to_owned(), ".".to_owned()];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum NumberFormat {
  Num = 0,
  USD = 1,
  CanadianDollar = 2,
  EUR = 4,
  Pound = 5,
  Yen = 6,
  Ruble = 7,
  Rupee = 8,
  Won = 9,
  Yuan = 10,
  Real = 11,
  Lira = 12,
  Rupiah = 13,
  Franc = 14,
  HongKongDollar = 15,
  NewZealandDollar = 16,
  Krona = 17,
  NorwegianKrone = 18,
  MexicanPeso = 19,
  Rand = 20,
  NewTaiwanDollar = 21,
  DanishKrone = 22,
  Baht = 23,
  Forint = 24,
  Koruna = 25,
  Shekel = 26,
  ChileanPeso = 27,
  PhilippinePeso = 28,
  Dirham = 29,
  ColombianPeso = 30,
  Riyal = 31,
  Ringgit = 32,
  Leu = 33,
  ArgentinePeso = 34,
  UruguayanPeso = 35,
  Percent = 36,
}

impl NumberFormat {
  pub fn value(&self) -> i64 {
    *self as i64
  }
}

impl std::default::Default for NumberFormat {
  fn default() -> Self {
    NumberFormat::Num
  }
}

impl From<i64> for NumberFormat {
  fn from(value: i64) -> Self {
    match value {
      0 => NumberFormat::Num,
      1 => NumberFormat::USD,
      2 => NumberFormat::CanadianDollar,
      4 => NumberFormat::EUR,
      5 => NumberFormat::Pound,
      6 => NumberFormat::Yen,
      7 => NumberFormat::Ruble,
      8 => NumberFormat::Rupee,
      9 => NumberFormat::Won,
      10 => NumberFormat::Yuan,
      11 => NumberFormat::Real,
      12 => NumberFormat::Lira,
      13 => NumberFormat::Rupiah,
      14 => NumberFormat::Franc,
      15 => NumberFormat::HongKongDollar,
      16 => NumberFormat::NewZealandDollar,
      17 => NumberFormat::Krona,
      18 => NumberFormat::NorwegianKrone,
      19 => NumberFormat::MexicanPeso,
      20 => NumberFormat::Rand,
      21 => NumberFormat::NewTaiwanDollar,
      22 => NumberFormat::DanishKrone,
      23 => NumberFormat::Baht,
      24 => NumberFormat::Forint,
      25 => NumberFormat::Koruna,
      26 => NumberFormat::Shekel,
      27 => NumberFormat::ChileanPeso,
      28 => NumberFormat::PhilippinePeso,
      29 => NumberFormat::Dirham,
      30 => NumberFormat::ColombianPeso,
      31 => NumberFormat::Riyal,
      32 => NumberFormat::Ringgit,
      33 => NumberFormat::Leu,
      34 => NumberFormat::ArgentinePeso,
      35 => NumberFormat::UruguayanPeso,
      36 => NumberFormat::Percent,
      _ => NumberFormat::Num,
    }
  }
}

define_currency_set!(
    number_currency {
        NUMBER : {
            code: "",
            exponent: 2,
            locale: EnEu,
            minor_units: 1,
            name: "number",
            symbol: "RUB",
            symbol_first: false,
        },
        PERCENT : {
            code: "",
            exponent: 2,
            locale: EnIn,
            minor_units: 1,
            name: "percent",
            symbol: "%",
            symbol_first: false,
        },
        USD : {
            code: "USD",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "United States Dollar",
            symbol: "$",
            symbol_first: true,
        },
        CANADIAN_DOLLAR : {
            code: "USD",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "Canadian Dollar",
            symbol: "CA$",
            symbol_first: true,
        },
         NEW_TAIWAN_DOLLAR : {
            code: "USD",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "NewTaiwan Dollar",
            symbol: "NT$",
            symbol_first: true,
        },
        HONG_KONG_DOLLAR : {
            code: "USD",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "HongKong Dollar",
            symbol: "HZ$",
            symbol_first: true,
        },
        NEW_ZEALAND_DOLLAR : {
            code: "USD",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "NewZealand Dollar",
            symbol: "NZ$",
            symbol_first: true,
        },
        EUR : {
            code: "EUR",
            exponent: 2,
            locale: EnEu,
            minor_units: 1,
            name: "Euro",
            symbol: "€",
            symbol_first: true,
        },
        GIP : {
            code: "GIP",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "Gibraltar Pound",
            symbol: "£",
            symbol_first: true,
        },
        CNY : {
            code: "CNY",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "Chinese Renminbi Yuan",
            symbol: "¥",
            symbol_first: true,
        },
        YUAN : {
            code: "CNY",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "Chinese Renminbi Yuan",
            symbol: "CN¥",
            symbol_first: true,
        },
        RUB : {
            code: "RUB",
            exponent: 2,
            locale: EnEu,
            minor_units: 1,
            name: "Russian Ruble",
            symbol: "RUB",
            symbol_first: false,
        },
        INR : {
            code: "INR",
            exponent: 2,
            locale: EnIn,
            minor_units: 50,
            name: "Indian Rupee",
            symbol: "₹",
            symbol_first: true,
        },
        KRW : {
            code: "KRW",
            exponent: 0,
            locale: EnUs,
            minor_units: 1,
            name: "South Korean Won",
            symbol: "₩",
            symbol_first: true,
        },
        BRL : {
            code: "BRL",
            exponent: 2,
            locale: EnUs,
            minor_units: 5,
            name: "Brazilian real",
            symbol: "R$",
            symbol_first: true,
        },
        TRY : {
            code: "TRY",
            exponent: 2,
            locale: EnEu,
            minor_units: 1,
            name: "Turkish Lira",
            // symbol: "₺",
            symbol: "TRY",
            symbol_first: true,
        },
        IDR : {
            code: "IDR",
            exponent: 2,
            locale: EnUs,
            minor_units: 5000,
            name: "Indonesian Rupiah",
            // symbol: "Rp",
            symbol: "IDR",
            symbol_first: true,
        },
        CHF : {
            code: "CHF",
            exponent: 2,
            locale: EnUs,
            minor_units: 5,
            name: "Swiss Franc",
            // symbol: "Fr",
            symbol: "CHF",
            symbol_first: true,
        },
        SEK : {
            code: "SEK",
            exponent: 2,
            locale: EnBy,
            minor_units: 100,
            name: "Swedish Krona",
            // symbol: "kr",
            symbol: "SEK",
            symbol_first: false,
        },
        NOK : {
            code: "NOK",
            exponent: 2,
            locale: EnUs,
            minor_units: 100,
            name: "Norwegian Krone",
            // symbol: "kr",
            symbol: "NOK",
            symbol_first: false,
        },
        MEXICAN_PESO : {
            code: "USD",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "Mexican Peso",
            symbol: "MX$",
            symbol_first: true,
        },
        ZAR : {
            code: "ZAR",
            exponent: 2,
            locale: EnUs,
            minor_units: 10,
            name: "South African Rand",
            // symbol: "R",
            symbol: "ZAR",
            symbol_first: true,
        },
        DKK : {
            code: "DKK",
            exponent: 2,
            locale: EnEu,
            minor_units: 50,
            name: "Danish Krone",
            // symbol: "kr.",
            symbol: "DKK",
            symbol_first: false,
        },
        THB : {
            code: "THB",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "Thai Baht",
            // symbol: "฿",
            symbol: "THB",
            symbol_first: true,
        },
        HUF : {
            code: "HUF",
            exponent: 0,
            locale: EnBy,
            minor_units: 5,
            name: "Hungarian Forint",
            // symbol: "Ft",
            symbol: "HUF",
            symbol_first: false,
        },
        KORUNA : {
            code: "CZK",
            exponent: 2,
            locale: EnBy,
            minor_units: 100,
            name: "Czech Koruna",
            // symbol: "Kč",
            symbol: "CZK",
            symbol_first: false,
        },
        SHEKEL : {
            code: "CZK",
            exponent: 2,
            locale: EnBy,
            minor_units: 100,
            name: "Czech Koruna",
            symbol: "Kč",
            symbol_first: false,
        },
        CLP : {
            code: "CLP",
            exponent: 0,
            locale: EnEu,
            minor_units: 1,
            name: "Chilean Peso",
            // symbol: "$",
            symbol: "CLP",
            symbol_first: true,
        },
        PHP : {
            code: "PHP",
            exponent: 2,
            locale: EnUs,
            minor_units: 1,
            name: "Philippine Peso",
            symbol: "₱",
            symbol_first: true,
        },
        AED : {
            code: "AED",
            exponent: 2,
            locale: EnUs,
            minor_units: 25,
            name: "United Arab Emirates Dirham",
            // symbol: "د.إ",
            symbol: "AED",
            symbol_first: false,
        },
        COP : {
            code: "COP",
            exponent: 2,
            locale: EnEu,
            minor_units: 20,
            name: "Colombian Peso",
            // symbol: "$",
            symbol: "COP",
            symbol_first: true,
        },
        SAR : {
            code: "SAR",
            exponent: 2,
            locale: EnUs,
            minor_units: 5,
            name: "Saudi Riyal",
            // symbol: "ر.س",
            symbol: "SAR",
            symbol_first: true,
        },
        MYR : {
            code: "MYR",
            exponent: 2,
            locale: EnUs,
            minor_units: 5,
            name: "Malaysian Ringgit",
            // symbol: "RM",
            symbol: "MYR",
            symbol_first: true,
        },
        RON : {
            code: "RON",
            exponent: 2,
            locale: EnEu,
            minor_units: 1,
            name: "Romanian Leu",
            // symbol: "ر.ق",
            symbol: "RON",
            symbol_first: false,
        },
        ARS : {
            code: "ARS",
            exponent: 2,
            locale: EnEu,
            minor_units: 1,
            name: "Argentine Peso",
            // symbol: "$",
            symbol: "ARS",
            symbol_first: true,
        },
        UYU : {
            code: "UYU",
            exponent: 2,
            locale: EnEu,
            minor_units: 100,
            name: "Uruguayan Peso",
            // symbol: "$U",
            symbol: "UYU",
            symbol_first: true,
        }
    }
);

impl NumberFormat {
  pub fn currency(&self) -> &'static number_currency::Currency {
    match self {
      NumberFormat::Num => number_currency::NUMBER,
      NumberFormat::USD => number_currency::USD,
      NumberFormat::CanadianDollar => number_currency::CANADIAN_DOLLAR,
      NumberFormat::EUR => number_currency::EUR,
      NumberFormat::Pound => number_currency::GIP,
      NumberFormat::Yen => number_currency::CNY,
      NumberFormat::Ruble => number_currency::RUB,
      NumberFormat::Rupee => number_currency::INR,
      NumberFormat::Won => number_currency::KRW,
      NumberFormat::Yuan => number_currency::YUAN,
      NumberFormat::Real => number_currency::BRL,
      NumberFormat::Lira => number_currency::TRY,
      NumberFormat::Rupiah => number_currency::IDR,
      NumberFormat::Franc => number_currency::CHF,
      NumberFormat::HongKongDollar => number_currency::HONG_KONG_DOLLAR,
      NumberFormat::NewZealandDollar => number_currency::NEW_ZEALAND_DOLLAR,
      NumberFormat::Krona => number_currency::SEK,
      NumberFormat::NorwegianKrone => number_currency::NOK,
      NumberFormat::MexicanPeso => number_currency::MEXICAN_PESO,
      NumberFormat::Rand => number_currency::ZAR,
      NumberFormat::NewTaiwanDollar => number_currency::NEW_TAIWAN_DOLLAR,
      NumberFormat::DanishKrone => number_currency::DKK,
      NumberFormat::Baht => number_currency::THB,
      NumberFormat::Forint => number_currency::HUF,
      NumberFormat::Koruna => number_currency::KORUNA,
      NumberFormat::Shekel => number_currency::SHEKEL,
      NumberFormat::ChileanPeso => number_currency::CLP,
      NumberFormat::PhilippinePeso => number_currency::PHP,
      NumberFormat::Dirham => number_currency::AED,
      NumberFormat::ColombianPeso => number_currency::COP,
      NumberFormat::Riyal => number_currency::SAR,
      NumberFormat::Ringgit => number_currency::MYR,
      NumberFormat::Leu => number_currency::RON,
      NumberFormat::ArgentinePeso => number_currency::ARS,
      NumberFormat::UruguayanPeso => number_currency::UYU,
      NumberFormat::Percent => number_currency::PERCENT,
    }
  }

  pub fn symbol(&self) -> String {
    self.currency().symbol.to_string()
  }
}
