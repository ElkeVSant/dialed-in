use crate::coffee::Coffee;

pub(super) struct Query {
    search: Option<String>,
    filters: Vec<RatingFilter>,
}

impl Query {
    pub(super) fn parse(query: &str) -> Result<Query, ParsingError> {
        let mut query = query.to_owned();
        let mut query_filters = Vec::new();
        while query.contains(".")
            && query.contains(":")
            && query.find(".").unwrap() < query.find(":").unwrap()
        {
            if query.contains(" ") {
                let parts: Vec<&str> = query.splitn(2, " ").collect();
                query_filters.push(parts[0].to_string());
                query = parts[1].to_string();
            } else {
                query_filters.push(query.clone());
                query = "".to_string();
                break;
            }
        }

        let mut filters = Vec::new();
        for filter in query_filters {
            let (rc, condition) = filter
                .split_once(":")
                .expect("split_once(':') failed despite contains(':') check");

            if condition.is_empty() {
                return Err(ParsingError {
                    message: "invalid filter value; values 0-5 are possible".to_string(),
                });
            }

            let rc = RatingCharacteristic::from_string(rc)?;

            let value = condition
                .chars()
                .next()
                .and_then(|v| v.to_digit(10))
                .ok_or(ParsingError {
                    message: "invalid filter value; values 0-5 are possible".to_string(),
                })? as u8;
            let operator = {
                if condition.len() == 2 {
                    Operator::from_string(condition.chars().nth(1).unwrap())?
                } else if condition.len() == 1 {
                    Operator::Equal
                } else {
                    return Err(ParsingError {
                        message: "invalid filter condition".to_string(),
                    });
                }
            };

            filters.push(RatingFilter::Rating(rc, value, operator))
        }

        Ok(Query {
            search: { if query.is_empty() { None } else { Some(query) } },
            filters,
        })
    }
}

enum RatingFilter {
    Rating(RatingCharacteristic, u8, Operator),
}

enum Operator {
    AtLeast,
    AtMost,
    Equal,
}

impl Operator {
    fn from_string(operator: char) -> Result<Operator, ParsingError> {
        match operator {
            '+' => Ok(Operator::AtLeast),
            '-' => Ok(Operator::AtMost),
            _ => Err(ParsingError {
                message: "invalid filter operator; +, - or nothing (equality) is supported"
                    .to_string(),
            }),
        }
    }
}

enum RatingCharacteristic {
    AromaStrength,
    AromaPersonal,
    SweetnessStrength,
    SweetnessPersonal,
    AcidityStrength,
    AcidityPersonal,
    BodyStrength,
    BodyPersonal,
    AftertasteStrength,
    AftertastePersonal,
}

impl RatingCharacteristic {
    fn access(&self, coffee: &Coffee) -> Option<u8> {
        match self {
            RatingCharacteristic::AromaStrength => coffee.rating?.aroma?.strength,
            RatingCharacteristic::AromaPersonal => coffee.rating?.aroma?.personal,
            RatingCharacteristic::SweetnessStrength => coffee.rating?.sweetness?.strength,
            RatingCharacteristic::SweetnessPersonal => coffee.rating?.sweetness?.personal,
            RatingCharacteristic::AcidityStrength => coffee.rating?.acidity?.strength,
            RatingCharacteristic::AcidityPersonal => coffee.rating?.acidity?.personal,
            RatingCharacteristic::BodyStrength => coffee.rating?.body?.strength,
            RatingCharacteristic::BodyPersonal => coffee.rating?.body?.personal,
            RatingCharacteristic::AftertasteStrength => coffee.rating?.aftertaste?.strength,
            RatingCharacteristic::AftertastePersonal => coffee.rating?.aftertaste?.personal,
        }
    }

    pub(super) fn from_string(rc_string: &str) -> Result<RatingCharacteristic, ParsingError> {
        let (r, c) = rc_string
            .split_once(".")
            .expect("split_once('.') failed despite contains('.') check");
        match r {
            "ar" => match c {
                "s" => Ok(RatingCharacteristic::AromaStrength),
                "p" => Ok(RatingCharacteristic::AromaPersonal),
                _ => Err(ParsingError {
                    message: "characteristic isn't valid; use 's' for strength or 'p' for personal"
                        .to_string(),
                }),
            },
            "sw" => match c {
                "s" => Ok(RatingCharacteristic::SweetnessStrength),
                "p" => Ok(RatingCharacteristic::SweetnessPersonal),
                _ => Err(ParsingError {
                    message: "characteristic isn't valid; use 's' for strength or 'p' for personal"
                        .to_string(),
                }),
            },
            "ac" => match c {
                "s" => Ok(RatingCharacteristic::AcidityStrength),
                "p" => Ok(RatingCharacteristic::AcidityPersonal),
                _ => Err(ParsingError {
                    message: "characteristic isn't valid; use 's' for strength or 'p' for personal"
                        .to_string(),
                }),
            },
            "bo" => match c {
                "s" => Ok(RatingCharacteristic::BodyStrength),
                "p" => Ok(RatingCharacteristic::BodyPersonal),
                _ => Err(ParsingError {
                    message: "characteristic isn't valid; use 's' for strength or 'p' for personal"
                        .to_string(),
                }),
            },
            "af" => match c {
                "s" => Ok(RatingCharacteristic::AftertasteStrength),
                "p" => Ok(RatingCharacteristic::AftertastePersonal),
                _ => Err(ParsingError {
                    message: "characteristic isn't valid; use 's' for strength or 'p' for personal"
                        .to_string(),
                }),
            },
            _ => Err(ParsingError {
                message: "rating category isn't valid; options are 'ar', 'sw', 'ac', 'bo' and 'af'"
                    .to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub(super) struct ParsingError {
    pub(super) message: String,
}

pub(super) fn filter_coffees(list: &[Coffee], query: Query) -> Vec<&Coffee> {
    let mut filtered_list = list.iter().collect();
    if let Some(search_query) = query.search {
        filtered_list = search_coffees(list, &search_query)
    }
    for RatingFilter::Rating(characteristic, value, operator) in query.filters {
        match operator {
            Operator::AtLeast => {
                filtered_list.retain(|c| characteristic.access(c).is_some_and(|v| v >= value))
            }
            Operator::AtMost => {
                filtered_list.retain(|c| characteristic.access(c).is_some_and(|v| v <= value))
            }
            Operator::Equal => {
                filtered_list.retain(|c| characteristic.access(c).is_some_and(|v| v == value))
            }
        }
    }
    filtered_list
}

fn search_coffees<'a>(list: &'a [Coffee], query: &str) -> Vec<&'a Coffee> {
    let lc_query = &query.to_lowercase();
    list.iter()
        .filter(|c| {
            c.name.to_lowercase().contains(lc_query)
                || c.origin
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(lc_query)
                || c.varieties
                    .as_deref()
                    .unwrap_or_default()
                    .join(", ")
                    .to_lowercase()
                    .contains(lc_query)
                || c.process
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(lc_query)
                || c.decaffeination_process
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(lc_query)
                || c.notes
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(lc_query)
        })
        .collect()
}

pub(super) fn query_coffees<'a>(
    coffees: &'a [Coffee],
    query: Option<&str>,
) -> Result<Vec<&'a Coffee>, ParsingError> {
    if let Some(query) = query {
        Query::parse(query).map(|q| filter_coffees(coffees, q))
    } else {
        Ok(coffees.iter().collect())
    }
}
