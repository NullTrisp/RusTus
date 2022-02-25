use crate::types;
use reqwest::Error;

async fn get_raw_buses() -> Result<Vec<types::RawBus>, Error> {
    let res: types::ResponseBuses = serde_json::from_str(
        &reqwest::get("https://datos.santander.es/api/rest/datasets/lineas_bus.json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    )
    .unwrap();
    Ok(res.resources)
}

async fn get_raw_bus(number: String) -> Result<Option<types::RawBus>, Error> {
    let res: types::ResponseBuses = serde_json::from_str(
        &reqwest::get(
            "https://datos.santander.es/api/rest/datasets/lineas_bus/".to_owned()
                + &number
                + ".json",
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap(),
    )
    .unwrap();

    match res.resources.get(0) {
        Some(bus) => Ok(Some(bus.clone())),
        None => Ok(None),
    }
}

fn get_destinations(destionation: String) -> types::Destinations {
    let destinations_vector = destionation
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    match destinations_vector.clone().into_iter().count() == 1 {
        true => {
            let destinations_vector: Vec<String> = destinations_vector[0]
                .split("/")
                .map(|s| s.into())
                .collect();
            types::Destinations {
                a: destinations_vector.clone(),
                b: None,
            }
        }
        false => types::Destinations {
            a: destinations_vector[0]
                .split("/")
                .map(|s| s.into())
                .collect(),
            b: Some(
                destinations_vector[1]
                    .split("/")
                    .map(|s| s.into())
                    .collect(),
            ),
        },
    }
}

pub async fn get_buses() -> Result<Vec<types::Bus>, Error> {
    Ok(get_raw_buses()
        .await
        .unwrap()
        .into_iter()
        .map(|raw_bus| {
            let destinations_vector = raw_bus
                .name
                .split('-')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let destinations: types::Destinations =
                match destinations_vector.clone().into_iter().count() == 1 {
                    true => {
                        let destinations_vector: Vec<String> = destinations_vector[0]
                            .split("/")
                            .map(|s| s.into())
                            .collect();
                        types::Destinations {
                            a: destinations_vector.clone(),
                            b: None,
                        }
                    }
                    false => types::Destinations {
                        a: destinations_vector[0]
                            .split("/")
                            .map(|s| s.into())
                            .collect(),
                        b: Some(
                            destinations_vector[1]
                                .split("/")
                                .map(|s| s.into())
                                .collect(),
                        ),
                    },
                };

            types::Bus {
                number: raw_bus.number,
                id: raw_bus.id.parse().unwrap(),
                destinations,
            }
        })
        .collect::<Vec<types::Bus>>())
}

pub async fn get_bus(bus_number: String) -> Result<Option<types::Bus>, Error> {
    match get_raw_bus(bus_number).await {
        Ok(bus_found) => match bus_found {
            Some(bus) => Ok(Some(types::Bus {
                number: bus.number,
                destinations: get_destinations(bus.name),
                id: bus.id.parse().unwrap(),
            })),
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}
